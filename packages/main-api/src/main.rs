pub mod controllers;

use controllers::*;

use by_axum::{auth::authorization_middleware, axum::middleware};
use by_types::DatabaseConfig;
use dto::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub mod config;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let c = Content::get_repository(pool.clone());

    c.create_this_table().await?;
    c.create_related_tables().await?;

    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let app = app
        .nest(
            "/v1/assets",
            v1::AssetController::new(&conf.aws, &conf.bucket)
                .await
                .route()?,
        )
        .nest(
            "/v1/contents",
            v1::ContentController::new(pool.clone()).route()?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
