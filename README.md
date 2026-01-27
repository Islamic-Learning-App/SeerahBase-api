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

## Architecture

```mermaid
graph TD
    Client[Client App / Browser] <-->|HTTP/JSON| Server[Axum Server]
    Server <-->|SQLx| DB[(SQLite Database)]
    Server -->|Hosting| Swagger[Swagger UI]
    
    subgraph "Server Layer"
        Middleware[Compression (Gzip/Brotli)]
        Handlers[API Handlers]
        Router[Axum Router]
    end
    
    Server --- Middleware --- Router --- Handlers
```

## API Response Codes

| Status Code | Description |
|-------------|-------------|
| **200 OK** | Request processed successfully. Returns requested data or success message. |
| **404 Not Found** | The requested resource (Era, Event, or Question) does not exist. |
| **500 Internal Server Error** | Unexpected server-side error (e.g., database connection failed). |
| **400 Bad Request** | Invalid input parameters or missing required fields. |

**Common Response Format:**
All successful responses return JSON.
```json
{
  "data": ...
}
```
*(Note: Current endpoints return direct arrays/objects, structure above is generic representation)*

