use clap::Parser;
use rest_demo::{config::Config, http};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let _env = dotenvy::dotenv().unwrap_or_default();
    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .expect("Unable to connect to database");

    http::serve(config, db).await
}
