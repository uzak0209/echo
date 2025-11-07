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

/// SSEハンドラー (JWT認証あり)
/// GET /api/reactions/events
/// 認証方法:
/// 1. Authorization: Bearer <access_token> (優先)
/// 2. Cookie: refresh_token (フォールバック、標準EventSource用)
pub async fn reaction_events_handler(
    headers: HeaderMap,
    State((manager, jwt_service)): State<(Arc<ReactionStreamManager>, Arc<JwtService>)>,
) -> Response {
    // Try to extract JWT token from Authorization header first
    let user_id = if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        // Method 1: Authorization header (for Fetch API)
        match auth_header.to_str() {
            Ok(v) if v.starts_with("Bearer ") => {
                let token = &v[7..];
                match jwt_service.verify_access_token(token) {
                    Ok(claims) => match Uuid::parse_str(&claims.sub) {
                        Ok(id) => id,
                        Err(_) => {
                            return (StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response();
                        }
                    },
                    Err(_) => {
                        return (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response();
                    }
                }
            }
            _ => {
                return (StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response();
            }
        }
    } else if let Some(cookie_header) = headers.get(header::COOKIE) {
        // Method 2: Cookie with refresh token (for standard EventSource)
        match cookie_header.to_str() {
            Ok(cookies) => {
                let refresh_token = cookies
                    .split(';')
                    .find(|c| c.trim().starts_with("refresh_token="))
                    .and_then(|c| c.trim().strip_prefix("refresh_token="));

                match refresh_token {
                    Some(token) => {
                        // Verify refresh token and extract user_id
                        match jwt_service.verify_refresh_token(token) {
                            Ok(claims) => match Uuid::parse_str(&claims.sub) {
                                Ok(id) => id,
                                Err(_) => {
                                    return (StatusCode::UNAUTHORIZED, "Invalid user ID in refresh token").into_response();
                                }
                            },
                            Err(_) => {
                                return (StatusCode::UNAUTHORIZED, "Invalid or expired refresh token").into_response();
                            }
                        }
                    }
                    None => {
                        return (StatusCode::UNAUTHORIZED, "Missing refresh token in cookie").into_response();
                    }
                }
            }
            Err(_) => {
                return (StatusCode::UNAUTHORIZED, "Invalid cookie header").into_response();
            }
        }
    } else {
        // No authentication method provided
        return (StatusCode::UNAUTHORIZED, "Missing authentication (Authorization header or refresh_token cookie)").into_response();
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
