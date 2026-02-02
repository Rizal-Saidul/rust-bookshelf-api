# Rust Bookshelf API

A REST API for managing a bookshelf built with Rust using Axum, SQLx, and PostgreSQL.

## Overview

This project implements a simple yet functional book management API with the following features:
- Create and list books
- Database persistence with PostgreSQL
- Type-safe database queries with SQLx
- Proper error handling and HTTP status codes

## Project Structure

```
├── src/
│   ├── main.rs          # Main application entry point with route handlers
│   ├── controllers/     # API route handlers
│   ├── models/          # Data models
│   ├── services/        # Business logic
│   └── db/              # Database layer
├── migrations/
│   └── 0001_books_table.sql   # Initial database schema
├── Cargo.toml           # Project dependencies
├── Dockerfile           # Container configuration
└── compose.yml          # Docker Compose setup
```

## Recent Updates (Feb 2, 2026)

### Features Implemented
- **Database Models**: Created `Book` and `BookPayload` structs with proper serialization/deserialization
- **Database Integration**: Implemented PostgreSQL connection pool using SQLx
- **API Endpoints**:
  - `GET /` - Home/welcome endpoint
  - `GET /books` - Retrieve all books from database
  - `POST /books` - Create a new book entry
- **Error Handling**: Proper HTTP status codes (201 CREATED, 500 INTERNAL_SERVER_ERROR)
- **Migrations**: SQLx migrations for database schema management

### Technical Implementation

#### Data Models
```rust
struct Book {
    id: i32,
    title: String,
    author: Option<String>,
    published_date: Option<NaiveDate>,
    stock: i32,
    created_at: NaiveDateTime,
}

struct Bookpayload {
    title: String,
    author: Option<String>,
    stock: i32,
    published_date: Option<NaiveDate>,
    created_at: NaiveDateTime,
}
```

#### Database Connection
- Uses `PgPoolOptions` for connection pooling
- Automatic migration execution on startup
- Environment variable `DATABASE_URL` for database configuration

#### Route Handlers
- `async fn home()` - Returns welcome message
- `async fn list_book()` - Fetches all books from database
- `async fn create_book()` - Inserts new book and returns created record

## Dependencies

Core dependencies used:
- **axum** - Web framework
- **tokio** - Async runtime
- **sqlx** - Type-safe SQL queries
- **serde** - Serialization/deserialization
- **chrono** - Date/time handling
- **postgres** - PostgreSQL driver

## Setup Instructions

### Prerequisites
- Rust 1.70+
- PostgreSQL 13+
- Docker & Docker Compose (optional)

### Local Development

1. Clone the repository
2. Create `.env` file with database URL:
   ```
   DATABASE_URL=postgresql://user:password@localhost:5432/bookshelf
   ```
3. Run migrations:
   ```bash
   sqlx migrate run
   ```
4. Start the server:
   ```bash
   cargo run
   ```

The API will be available at `http://127.0.0.1:3000`

### Docker Setup

Start the application with Docker Compose:
```bash
docker compose up -d app
```

## API Usage Examples

### Get all books
```bash
curl http://localhost:3000/books
```

### Create a new book
```bash
curl -X POST http://localhost:3000/books \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Learning Rust",
    "author": "John Doe",
    "stock": 5,
    "published_date": "2024-01-15",
    "created_at": "2024-01-15T10:00:00"
  }'
```

## Future Enhancements

- [ ] Implement GET /books/{id} endpoint
- [ ] Implement PUT /books/{id} update endpoint
- [ ] Implement DELETE /books/{id} endpoint
- [ ] Add pagination for book list
- [ ] Add input validation
- [ ] Add authentication/authorization
- [ ] Add logging and monitoring
- [ ] Separate concerns into controllers, services, and models
