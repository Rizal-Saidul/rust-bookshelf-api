use axum::{ Router, routing::get };
use chrono::{ NaiveDate, NaiveDateTime };
use serde::{ Deserialize, Serialize };
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
struct Bookpayload {
    title: String,
    author: Option<String>,
    stock: i32,
    published_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
}

#[derive(Serialize, FromRow)]
struct Book {
    id: i32,
    title: String,
    author: Option<String>,
    published_date: Option<NaiveDate>,
    stock: i32,
    created_at: NaiveDateTime,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async { "hello world" })
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
