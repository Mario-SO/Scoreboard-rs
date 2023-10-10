use axum::{
    extract,
    http::{self, StatusCode},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct Scores {
    home_score: i32,
    guest_score: i32,
}

impl Scores {
    fn new(home_score: i32, guest_score: i32) -> Self {
        Self {
            home_score,
            guest_score,
        }
    }
}

pub async fn check_health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn add_home(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(_payload): axum::Json<Scores>,
) -> Result<(StatusCode, Json<Scores>), StatusCode> {
    let score = Scores::new(0, 0);

    let res = sqlx::query(
        r#"
        INSERT INTO scores (home_score, guest_score)
        VALUES ($1, $2)
        "#,
    )
    .bind(score.home_score)
    .bind(score.guest_score)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, axum::Json(score))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// pub async fn plusHome() -> http::StatusCode {}
