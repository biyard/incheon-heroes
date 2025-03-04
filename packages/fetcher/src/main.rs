pub mod config;
pub mod incheon_contents_etl;

use incheon_contents_etl::incheon_contents_etl;
use tokio::net::TcpListener;

use by_types::DatabaseConfig;
use dto::*;
use sqlx::postgres::PgPoolOptions;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let event = dto::events::Event::get_repository(pool.clone());
    let user_nft_trasfer = dto::events::UserNftTransfer::get_repository(pool.clone());

    event.create_this_table().await?;
    user_nft_trasfer.create_this_table().await?;

    event.create_table().await?;
    user_nft_trasfer.create_table().await?;

    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let conf = config::get();

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    tokio::spawn(async move {
        tracing::info!("Starting fetcher");
        match incheon_contents_etl(&pool).await {
            Ok(_) => tracing::info!("Fetcher done"),
            Err(e) => tracing::error!("Fetcher failed: {:?}", e),
        };
    });

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let _ = by_axum::serve(listener, app).await;

    Ok(())
}
