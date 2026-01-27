# SeerahBase API

Islamic History (Seerah) and MCQs, built with **Rust** and **Axum**.

## Tech Stack
- **Language**: Rust
- **Framework**: Axum
- **Database**: SQLite (via SQLx)
- **Documentation**: OpenAPI (utoipa)
- **Compression**: Gzip/Brotli (tower-http)

## Setup & Run

### 1. Prerequisites
- Rust & Cargo installed.

### 2. Initialize Database
Creates `seerah.db` and applies schema.
```bash
cargo run --bin init_db
```

### 3. Seed Data
Populates database with content (Eras, Events, MCQs).
```bash
cargo run --bin seed_db
```

### 4. Run Server
Starts the API on `http://localhost:3000`.
```bash
cargo run --bin SeerahBase-api
```

## API Documentation
Interactive Swagger UI is available at:
**[http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)**

## Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/eras` | List all history periods |
| GET | `/eras/{id}/events` | List events in a specific period |
| GET | `/events` | List all events |
| GET | `/questions/event/{id}` | Get MCQs for a specific event |
| GET | `/questions/random` | Get a random quiz (5 questions) |

## Project Structure
- `src/main.rs` - App entry point & router configuration.
- `src/handlers/` - API request handlers.
- `src/models/` - Database structs & OpenAPI schemas.
- `src/db/` - Database connection pool.
- `schema.sql` - Database schema definition.
