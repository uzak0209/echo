mod domain;
mod application;
mod infrastructure;
mod presentation;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    schema: presentation::graphql::schema::AppSchema,
}

async fn graphql_handler(
    State(state): State<AppState>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(req.into_inner()).await.into()
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
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    // Connect to database
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    // Build GraphQL schema (DI is handled inside build_schema)
    let schema = presentation::build_schema(db);

    let state = AppState { schema };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .layer(cors)
        .with_state(state);

    println!("GraphQL Playground: http://localhost:{}", port);
    println!("GraphQL Endpoint: http://localhost:{}/graphql", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await?;

    axum::serve(listener, app).await?;

    Ok(())
}
