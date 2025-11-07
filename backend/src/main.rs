mod domain;
mod application;
mod infrastructure;
mod presentation;

use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    schema: presentation::graphql::schema::AppSchema,
    stream_manager: Arc<infrastructure::sse::ReactionStreamManager>,
    jwt_service: Arc<infrastructure::auth::JwtService>,
}

async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Response {
    // Extract refresh token from cookie if present
    let refresh_token = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find(|c| c.trim().starts_with("refresh_token="))
                .map(|c| c.trim().strip_prefix("refresh_token=").unwrap().to_string())
        });

    // Extract access token from Authorization header
    let access_token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|t| t.to_string());

    // Build request with tokens and user claims in context
    let mut request = req.into_inner();

    // Add refresh token if available
    if let Some(token) = refresh_token {
        request = request.data(token);
    }

    // Verify access token and add user_id to context
    if let Some(token) = access_token {
        if let Ok(claims) = state.jwt_service.verify_access_token(&token) {
            // Add user_id to context
            if let Ok(user_id) = uuid::Uuid::parse_str(&claims.sub) {
                request = request.data(user_id);
            }
        }
    }

    // Execute GraphQL request
    let response = state.schema.execute(request).await;

    // Extract refresh token from response headers before converting
    let refresh_token_header = response.http_headers.get("X-Refresh-Token").cloned();

    // Convert to HTTP response
    let mut http_response: Response = GraphQLResponse::from(response).into_response();

    // If there's a refresh token in the response, set it as a cookie
    if let Some(refresh_token) = refresh_token_header {
        if let Ok(token_str) = refresh_token.to_str() {
            let cookie = format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                token_str,
                30 * 24 * 60 * 60 // 30 days in seconds
            );
            http_response.headers_mut().insert(
                header::SET_COOKIE,
                cookie.parse().unwrap(),
            );
        }
    }

    http_response
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    // Connect to database
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    // Create SSE stream manager
    let stream_manager = Arc::new(infrastructure::sse::ReactionStreamManager::new());

    // Create JWT service
    let jwt_service = Arc::new(infrastructure::auth::JwtService::new(&jwt_secret));

    // Build GraphQL schema (DI is handled inside build_schema)
    let schema = presentation::build_schema(db, jwt_secret, stream_manager.clone());

    let state = AppState {
        schema,
        stream_manager: stream_manager.clone(),
        jwt_service: jwt_service.clone(),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true);

    // Build router
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .with_state(state.clone())
        .route("/api/reactions/events",
            get(presentation::sse::reaction_events_handler)
                .with_state((stream_manager.clone(), jwt_service)))
        .layer(cors);

    println!("GraphQL Playground: http://localhost:{}", port);
    println!("GraphQL Endpoint: http://localhost:{}/graphql", port);
    println!("SSE Endpoint: http://localhost:{}/api/reactions/events (requires Authorization: Bearer <token>)", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await?;

    axum::serve(listener, app).await?;

    Ok(())
}
