use axum::routing::{get, Router};
use sqlx::error::BoxDynError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let PORT = 3000;
    let ADDR = format!("0.0.0.0:{}", PORT);

    let app = Router::new().route("/", get(root));

    axum::Server::bind(&ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "hello bruh"
}
