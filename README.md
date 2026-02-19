# SeerahBase API

Islamic History (Seerah) and MCQs, built with **Rust**, **Axum**, and **Turso** (libsql).

## 🚀 Tech Stack

### Backend

- **Language**: Rust (2024 edition)
- **Framework**: Axum 0.8
- **Database**: Turso (libsql) with **Embedded Replica** support
- **Documentation**: OpenAPI (utoipa)
- **Compression**: Gzip/Brotli (tower-http)

### Frontend

- **Framework**: SvelteKit + Vite
- **Styling**: TailwindCSS
- **Package Manager**: pnpm

---

## 🛠️ Setup & Run

### 1. Prerequisites

- Rust & Cargo installed.
- `pnpm` installed (`npm install -g pnpm`).
- A [Turso](https://turso.tech) database.

### 2. Configure Environment

Create a `.env` file in the root directory:

```env
# Backend Config
TURSO_DATABASE_URL="libsql://your-db-name.turso.io"
TURSO_AUTH_TOKEN="your-turso-auth-token"
API_KEY="your-secret-api-key"
RUST_LOG="info"

# Optional: Enable Embedded Replica (Local Read Cache)
DB_FILE="seerah.db"
```

_Note: If `DB_FILE` is set, the app will create a local SQLite file that syncs with Turso for fast, zero-cost reads._

### 3. Initialize & Seed Database

run the seeder to apply the schema and populate initial categories, events, and questions.

```bash
cargo run --bin seed_db
```

_(Note: `init_db` is deprecated; `seed_db` handles schema creation now)_

### 4. Run Backend Server

Starts the API on `http://localhost:3000`.

```bash
cargo run --bin SeerahBase-api
```

Access Swagger UI at **[http://localhost:3000/swagger-ui/](http://localhost:3000/swagger-ui/)**.

### 5. Run Frontend

Navigate to the frontend directory and start the dev server:

```bash
cd frontend-demo
pnpm install
pnpm dev
# App runs at http://localhost:5173
```

---

## 🐳 Docker (Production)

Build and run the container. The Dockerfile is optimized for Turso.

```bash
# Build image
docker build -t seerahbase-api .

# Run container (pass env vars)
docker run -p 3000:3000 --env-file .env seerahbase-api
```

---

## 📚 API Endpoints

| Method | Endpoint                  | Description                                  |
| ------ | ------------------------- | -------------------------------------------- |
| GET    | `/categories`             | List all categories (Eras, Prophets, Topics) |
| GET    | `/categories/{id}/events` | List events in a specific category           |
| GET    | `/events`                 | List all events (paginated)                  |
| GET    | `/events/{id}`            | Get items details                            |
| GET    | `/events/{id}/quiz`       | Get MCQs for a specific event                |
| GET    | `/quiz/random`            | Get a random quiz (5 questions)              |

---

## 🏗️ Architecture

```mermaid
graph TD
    User[User / Frontend] -->|HTTP/JSON| API[Axum API Server]
    API -->|Validation| Handlers

    subgraph "Data Layer"
        Handlers -->|Read (Fast)| LocalDB[(Local Replica)]
        Handlers -->|Write (Sync)| LocalDB
        LocalDB <-->|Background Sync| RemoteDB[(Turso Cloud)]
    end

    Handlers -->|Response| API
    API -->|Compressed JSON| User
```
