use std::env;
use axum::{ Json, Router, extract::{ Path, State }, http::StatusCode, routing::get };
use chrono::{ NaiveDate, NaiveDateTime };
use serde::{ Deserialize, Serialize };
use sqlx::{ PgPool, postgres::PgPoolOptions, prelude::FromRow };

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
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL mus be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to DB");
    sqlx::migrate!().run(&pool).await.expect("migration failed");

    let app = Router::new()
        .route("/", get(home))
        .route("/books", get(list_book).post(create_book))
        .route("/book/{id}", get(get_book).put(update_book).delete(delete_book))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "welcome to bookself api - home"
}

// get all book
async fn list_book(State(pool): State<PgPool>) -> Result<Json<Vec<Book>>, StatusCode> {
    sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// create book
async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<Bookpayload>
) -> Result<(StatusCode, Json<Book>), StatusCode> {
    sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, auhtor, published_date, stock, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
        .bind(payload.title)
        .bind(payload.author)
        .bind(payload.published_date)
        .bind(payload.stock)
        .bind(payload.created_at)
        .fetch_one(&pool).await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// get book by Id
async fn get_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<Json<Book>, StatusCode> {
    sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1")
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

// Update book
async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<Bookpayload>
) -> Result<Json<Book>, StatusCode> {
    sqlx::query_as::<_, Book>(
        "UPDATE books SET title = $1, author = $2, published_date = $3, stock = $4, created_at = $5 WHERE id = $6"
    )
        .bind(payload.title)
        .bind(payload.author)
        .bind(payload.published_date)
        .bind(payload.stock)
        .bind(payload.created_at)
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// delete book
async fn delete_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx
        ::query(" DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if  result.rows_affected() == 0  {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
