# Build Stage
FROM rust:1-slim-bookworm as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY schema.sql .

# Build release binaries
RUN cargo build --release --bin SeerahBase-api --bin init_db --bin seed_db

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /app/target/release/SeerahBase-api .
COPY --from=builder /app/target/release/init_db .
COPY --from=builder /app/target/release/seed_db .
COPY --from=builder /app/schema.sql .

# Create data directory
RUN mkdir -p /app/data

# Environment variables
ENV DATABASE_URL=sqlite:/app/data/seerah.db
ENV RUST_LOG=info

# Expose port
EXPOSE 3000

# Entrypoint (defaults to running the API)
CMD ["./SeerahBase-api"]
