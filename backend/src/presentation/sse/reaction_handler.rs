use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{sse::{Event, KeepAlive, Sse}, IntoResponse, Response},
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::infrastructure::{auth::JwtService, sse::ReactionStreamManager};

/// ヘッダーからユーザーIDを抽出して検証
fn extract_user_id(headers: &HeaderMap, jwt_service: &JwtService) -> Result<Uuid, Response> {
    // Method 1: Authorization header (for Fetch API)
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        return extract_from_bearer_token(auth_header, jwt_service);
    }

    // Method 2: Cookie with refresh token (for standard EventSource)
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        return extract_from_cookie(cookie_header, jwt_service);
    }

    // No authentication method provided
    Err((
        StatusCode::UNAUTHORIZED,
        "Missing authentication (Authorization header or refresh_token cookie)",
    )
        .into_response())
}

/// AuthorizationヘッダーからユーザーIDを抽出
fn extract_from_bearer_token(
    auth_header: &axum::http::HeaderValue,
    jwt_service: &JwtService,
) -> Result<Uuid, Response> {
    let auth_str = auth_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response())?;

    if !auth_str.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization format").into_response());
    }

    let token = &auth_str[7..];
    let claims = jwt_service
        .verify_access_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response())?;

    Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response())
}

/// CookieからユーザーIDを抽出
fn extract_from_cookie(
    cookie_header: &axum::http::HeaderValue,
    jwt_service: &JwtService,
) -> Result<Uuid, Response> {
    let cookies = cookie_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid cookie header").into_response())?;

    let refresh_token = cookies
        .split(';')
        .find(|c| c.trim().starts_with("refresh_token="))
        .and_then(|c| c.trim().strip_prefix("refresh_token="))
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, "Missing refresh token in cookie").into_response()
        })?;

    let claims = jwt_service
        .verify_refresh_token(refresh_token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired refresh token").into_response())?;

    Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in refresh token").into_response())
}

/// SSEハンドラー (JWT認証あり)
/// GET /api/reactions/events
/// 認証方法:
/// 1. Authorization: Bearer <access_token> (優先)
/// 2. Cookie: refresh_token (フォールバック、標準EventSource用)
pub async fn reaction_events_handler(
    headers: HeaderMap,
    State((manager, jwt_service)): State<(Arc<ReactionStreamManager>, Arc<JwtService>)>,
) -> Response {
    // Extract and verify user_id from headers
    let user_id = match extract_user_id(&headers, &jwt_service) {
        Ok(id) => id,
        Err(response) => return response,
    };

    // Subscribe to reaction stream
    let receiver = manager.subscribe(user_id).await;

    // Convert broadcast receiver to stream
    let stream = BroadcastStream::new(receiver);

    // Map events to SSE format
    let sse_stream = stream.map(|result| {
        match result {
            Ok(event) => {
                // Serialize event to JSON
                serde_json::to_string(&event).ok().map(|json| {
                    Ok::<_, Infallible>(Event::default().data(json))
                })
            }
            Err(_) => None, // Ignore lagged messages
        }
    }).filter_map(|opt| opt);

    Sse::new(sse_stream).keep_alive(KeepAlive::default()).into_response()
}
