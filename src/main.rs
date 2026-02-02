use std::env;
use axum::{
    Json,
    Router,
    extract::{ Path, State },
    http::StatusCode,
    routing::{ get },
};
use chrono::{ NaiveDate, NaiveDateTime };
use serde::{ Deserialize, Serialize };
use sqlx::{ PgPool, postgres::PgPoolOptions, prelude::FromRow };

#[derive(Deserialize)]
struct Bookpayload {
    title: String,
    author: Option<String>,
    stock: i32,
    published_date: Option<NaiveDate>,
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
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to DB");
    sqlx::migrate!().run(&pool).await.expect("Migration failed");

    let app = Router::new()
        .route("/", get(home))
        .route("/books", get(list_book).post(create_book))
        .route("/books/{id}", get(get_book).put(update_book).delete(delete_book))
        .with_state(pool);

    let listener = tokio::net::TcpListener
        ::bind("0.0.0.0:8000").await
        .expect("Failed to bind to address");

    println!("Server running on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Welcome to Bookshelf API"
}

// Get all books
async fn list_book(State(pool): State<PgPool>) -> Result<Json<Vec<Book>>, StatusCode> {
    sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&pool).await
        .map(Json)
        .map_err(|e| {
            eprintln!("List books error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

// Create a new book
async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<Bookpayload>
) -> Result<(StatusCode, Json<Book>), StatusCode> {
    // Validate title
    if payload.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query_as::<_, Book>(
        "INSERT INTO books (title, author, published_date, stock) VALUES ($1, $2, $3, $4) RETURNING *"
    )
        .bind(payload.title.trim())
        .bind(payload.author.as_ref().map(|a| a.trim()))
        .bind(payload.published_date)
        .bind(payload.stock)
        .fetch_one(&pool).await
        .map(|book| (StatusCode::CREATED, Json(book)))
        .map_err(|e| {
            eprintln!("Create book error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

// Get book by ID
async fn get_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<Json<Book>, StatusCode> {
    sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1")
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|e| {
            eprintln!("Get book error: {}", e);
            StatusCode::NOT_FOUND
        })
}

// Update a book
async fn update_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<Bookpayload>
) -> Result<Json<Book>, StatusCode> {
    // Validate title
    if payload.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query_as::<_, Book>(
        "UPDATE books SET title = $1, author = $2, published_date = $3, stock = $4 WHERE id = $5 RETURNING *"
    )
        .bind(payload.title.trim())
        .bind(payload.author.as_ref().map(|a| a.trim()))
        .bind(payload.published_date)
        .bind(payload.stock)
        .bind(id)
        .fetch_one(&pool).await
        .map(Json)
        .map_err(|e| {
            eprintln!("Update book error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

// Delete a book
async fn delete_book(
    State(pool): State<PgPool>,
    Path(id): Path<i32>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx
        ::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&pool).await
        .map_err(|e| {
            eprintln!("Delete book error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
