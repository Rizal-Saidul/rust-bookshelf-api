# rust-bookshelf-api

## Changes Today
- Added data models for Book and BookPayload structs
- Integrated SQLx for database operations with `FromRow` derive
- Added serialization support with Serde for API responses
- Implemented chrono for date/time handling
- Implemented database connection with PgPool
- Added route handlers for home, list_book, and create_book endpoints
- Integrated database migrations with SQLx
- Added error handling with StatusCode responses
