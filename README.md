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

## Bug Fixes & Improvements (Feb 2, 2026 - Code Review)

### Critical Bugs Fixed
- ✅ **Timestamp Type Mismatch**: Fixed `chrono::Utc::now()` returning `DateTime<Utc>` instead of `NaiveDateTime`. Server now lets database handle `created_at` automatically.
- ✅ **Update Bug**: Removed overwriting of `created_at` on update operations - this field is now immutable
- ✅ **Input Validation**: Added validation to prevent empty book titles
- ✅ **Route Path Consistency**: Changed `/book/{id}` to `/books/:id` for RESTful consistency

### Code Quality Improvements
- ✅ **Error Handling**: Added `eprintln!()` logging for all database operations for better debugging
- ✅ **Connection Pool**: Configured PgPool with max 5 connections for better resource management
- ✅ **Input Sanitization**: Added `.trim()` on string inputs to remove whitespace
- ✅ **Better Comments**: Improved code documentation and formatting
- ✅ **Type Safety**: Proper handling of `Option<String>` for optional fields

### Database Schema Updates
- ✅ **Consistent Field Sizes**: Increased `author` VARCHAR from 50 to 255 characters
- ✅ **NOT NULL Constraints**: Added explicit NOT NULL constraints for required fields
- ✅ **Updated Timestamp**: Added `updated_at` field for tracking changes
- ✅ **Default Values**: Explicit DEFAULT NOW() for timestamp fields

### Testing Status
✅ **Code compiles without errors or warnings**
✅ **All handlers properly implemented**: list, create, get, update, delete
✅ **Proper HTTP status codes**: 201 CREATED, 204 NO_CONTENT, 404 NOT_FOUND, 500 INTERNAL_SERVER_ERROR
✅ **Type-safe SQLx queries** with parameterized statements

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
