use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::infrastructure::{auth::JwtService, sse::PostStreamManager};

#[derive(Deserialize)]
pub struct SseQueryParams {
    token: Option<String>,
}

fn extract_user_id(
    _headers: &HeaderMap,
    query_params: &SseQueryParams,
    jwt_service: &JwtService,
) -> Result<Uuid, Box<Response>> {
    // SSE接続にはクエリパラメータで短命トークン（60秒）を必須とする
    // EventSource APIはカスタムヘッダーを送信できないため、クエリパラメータのみ対応
    if let Some(token) = &query_params.token {
        // SSE専用トークンを検証（60秒の有効期限）
        match jwt_service.verify_sse_token(token) {
            Ok(claims) => {
                return Uuid::parse_str(&claims.sub)
                    .map_err(|_| Box::new((StatusCode::UNAUTHORIZED, "Invalid user ID in SSE token").into_response()))
            }
            Err(_) => {
                return Err(Box::new((StatusCode::UNAUTHORIZED, "Invalid or expired SSE token").into_response()))
            }
        }
    }

    Err(Box::new((
        StatusCode::UNAUTHORIZED,
        "Missing SSE token. Use generateSseToken mutation to get a token, then connect with ?token=<sse_token>",
    )
        .into_response()))
}

/// SSEハンドラー (短命トークン認証)
/// GET /api/posts/events?token=<sse_token>
///
/// 認証:
/// - SSE専用の短命トークン（有効期限60秒）をクエリパラメータで必須
/// - 事前に generateSseToken mutation でトークンを取得してください
/// - EventSource APIの制約によりカスタムヘッダーは使用不可
///
/// イベントの種類:
/// - new_post: 新規投稿が作成された
/// - display_count_updated: 投稿の表示回数が更新された
/// - post_deleted: 投稿が削除された（100回表示達成）
pub async fn post_events_handler(
    Query(query_params): Query<SseQueryParams>,
    headers: HeaderMap,
    State((manager, jwt_service)): State<(Arc<PostStreamManager>, Arc<JwtService>)>,
) -> Response {
    // Extract and verify user_id from query params or headers
    let _user_id = match extract_user_id(&headers, &query_params, &jwt_service) {
        Ok(id) => id,
        Err(response) => return *response,
    };

    // Subscribe to global post stream
    let receiver = manager.subscribe().await;

    // Convert broadcast receiver to stream
    let stream = BroadcastStream::new(receiver);

    // Map events to SSE format
    let sse_stream = stream
        .map(|result| {
            match result {
                Ok(event) => {
                    // Serialize event to JSON
                    serde_json::to_string(&event)
                        .ok()
                        .map(|json| Ok::<_, Infallible>(Event::default().data(json)))
                }
                Err(_) => None, // Ignore lagged messages
            }
        })
        .filter_map(|opt| opt);

    Sse::new(sse_stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}
