use axum::routing::get;
use axum::{routing::post, Router};

use http::Method;
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

mod handlers;
use handlers::{get, post, site};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:Tr0069er@localhost:5432/pastebin";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    let router = Router::new()
        .route("/pasted", post(post::handle_fn))
        .route("/paste/:id", get(get::handle_fn))
        .route("/", get(site::site))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([192, 168, 1, 35], 6969));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
