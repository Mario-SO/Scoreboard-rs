mod handlers;
use axum::routing::{get, Router};
use handlers::check_health;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: String = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("Missing database url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app: Router = Router::new().route("/", get(check_health)).with_state(pool);
    // .route("/plusHome", post(plusHome));
    // .route("/plusGuest", post(plusGuest));
    // .route("/minusHome", post(minusHome));
    // .route("/minusGuest", post(minusGuest));
    // .route("/homeScore", post(homeScore));
    // .route("/guestScore", post(guestScore));
    // .route("/reset", post(reset));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
