# Rust Bookshelf API

A CRUD REST API for managing a bookshelf built with Rust using Axum, SQLx, PostgreSQL, Docker, and Docker Compose.

## Overview

This project implements a complete book management API following best practices from modern Rust web development. The API provides full CRUD (Create, Read, Update, Delete) operations for managing books in a PostgreSQL database.

### Features
- ✅ Complete CRUD operations for books
- ✅ Type-safe database queries with SQLx
- ✅ Automatic database migrations
- ✅ Docker containerization with multi-stage builds
- ✅ Environment-based configuration
- ✅ Proper error handling and HTTP status codes
- ✅ Input validation and sanitization

## Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)** 0.8 - Modern web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** 0.8 - Async SQL toolkit with compile-time query verification
- **[PostgreSQL](https://www.postgresql.org/)** 15 - Reliable relational database
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Serde](https://serde.rs/)** - Serialization/deserialization
- **[Chrono](https://github.com/chronotope/chrono)** - Date and time handling
- **[Docker](https://www.docker.com/)** - Containerization

## Project Structure

```
server/
├── src/
│   └── main.rs              # Application entry point with all route handlers
├── migrations/
│   └── 0001_books_table.sql # Database schema
├── Cargo.toml               # Rust dependencies
├── Dockerfile               # Multi-stage Docker build
├── compose.yml              # Docker Compose configuration
└── README.md
```

## API Endpoints

| Method | Endpoint | Description | Status Code |
|--------|----------|-------------|-------------|
| GET | `/` | Health check | 200 |
| GET | `/books` | List all books | 200 |
| POST | `/books` | Create a new book | 201 |
| GET | `/books/{id}` | Get book by ID | 200, 404 |
| PUT | `/books/{id}` | Update book by ID | 200, 404, 500 |
| DELETE | `/books/{id}` | Delete book by ID | 204, 404 |

## Data Model

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
}
```

## Quick Start

### Prerequisites
- Docker and Docker Compose
- (Optional) Rust 1.70+ for local development

### Running with Docker Compose

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd server
   ```

2. **Start the application**
   ```bash
   docker compose up -d
   ```

3. **Verify the application is running**
   ```bash
   docker ps
   curl http://localhost:8000/
   ```

4. **Check database migrations**
   ```bash
   docker exec -it db_bookshelf psql -U user -d bookshelf_db -c "\dt"
   ```

### Local Development (without Docker)

1. **Set up PostgreSQL**
   ```bash
   # Start PostgreSQL locally or use Docker
   docker run -d \
     -e POSTGRES_USER=user \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=bookshelf_db \
     -p 5432:5432 \
     postgres:15-alpine
   ```

2. **Set environment variable**
   ```bash
   export DATABASE_URL="postgres://user:password@localhost:5432/bookshelf_db"
   ```

3. **Run the application**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:8000`

## API Usage Examples

### Health Check
```bash
curl http://localhost:8000/
# Response: Welcome to Bookshelf API
```

### Create a Book
```bash
curl -X POST http://localhost:8000/books \
  -H "Content-Type: application/json" \
  -d '{
    "title": "The Rust Programming Language",
    "author": "Steve Klabnik",
    "stock": 10,
    "published_date": "2023-01-15"
  }'
```

**Response:**
```json
{
  "id": 1,
  "title": "The Rust Programming Language",
  "author": "Steve Klabnik",
  "published_date": "2023-01-15",
  "stock": 10,
  "created_at": "2026-02-02T13:19:44.434817"
}
```

### List All Books
```bash
curl http://localhost:8000/books
```

### Get a Specific Book
```bash
curl http://localhost:8000/books/1
```

### Update a Book
```bash
curl -X PUT http://localhost:8000/books/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "The Rust Programming Language (2nd Edition)",
    "author": "Steve Klabnik",
    "stock": 15,
    "published_date": "2023-01-15"
  }'
```

### Delete a Book
```bash
curl -X DELETE http://localhost:8000/books/1
# Response: HTTP 204 No Content
```

## Database Schema

```sql
CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255),
    published_date DATE,
    stock INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
```

## Configuration

The application uses environment variables for configuration:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |

Example:
```
DATABASE_URL=postgres://user:password@db:5432/bookshelf_db
```

## Docker Configuration

### Multi-stage Dockerfile
The project uses a multi-stage build to optimize image size:
- **Stage 1**: Build the Rust application
- **Stage 2**: Create minimal runtime image with Debian Bookworm Slim

### Docker Compose Services
- **app**: Rust Axum application (port 8000)
- **db**: PostgreSQL 15 Alpine (port 5432)

## Development Notes

### Recent Updates (Feb 2, 2026)

✅ **Configuration Improvements**
- Added `macros` feature to sqlx for migration support
- Updated tokio features for optimized async runtime
- Fixed DATABASE_URL to use Docker service names for container networking
- Aligned Dockerfile port exposure with application port (8000)

✅ **Code Quality**
- Environment-based configuration using `std::env`
- Automatic database migrations on startup
- Input validation for required fields
- Proper error handling with descriptive messages

✅ **Testing**
- All CRUD endpoints tested and verified
- Database migrations working correctly
- Docker build optimized (~2 minute build time)

## Troubleshooting

### Container Issues
```bash
# View logs
docker logs app_bookshelf
docker logs db_bookshelf

# Restart containers
docker compose restart

# Rebuild from scratch
docker compose down -v
docker compose build --no-cache
docker compose up -d
```

### Database Connection Issues
- Ensure DATABASE_URL uses `db` hostname in Docker environment
- Check PostgreSQL container is running: `docker ps`
- Verify network connectivity: `docker network ls`

## Future Enhancements

- [ ] Add pagination for book listings
- [ ] Implement search and filtering
- [ ] Add authentication and authorization
- [ ] Implement rate limiting
- [ ] Add comprehensive logging with tracing
- [ ] Create integration tests
- [ ] Add API documentation with OpenAPI/Swagger
- [ ] Implement caching with Redis
- [ ] Add book categories/genres
- [ ] Implement soft deletes

## References

This project follows best practices from:
- [Rust CRUD REST API with Axum and SQLx](https://dev.to/francescoxx/rust-crud-rest-api-using-axum-sqlx-postgres-docker-and-docker-compose-152a)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)

## License

This project is open source and available under the MIT License.
