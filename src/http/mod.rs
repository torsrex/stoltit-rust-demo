use std::net::SocketAddr;

use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::config::Config;

mod errors;
mod persons;

#[derive(Clone)]
struct ApiContext {
    config: Config,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) {
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let app = api_router()
        .layer(Extension(ApiContext {
            config: config.clone(),
            db,
        }))
        .layer(TraceLayer::new_for_http());

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

fn api_router() -> Router {
    let routes = Router::new().merge(persons::router());

    Router::new().nest("/api", routes)
}
