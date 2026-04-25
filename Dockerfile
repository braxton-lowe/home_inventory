# --- Build stage ---
FROM rust:latest AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/home_inventory .
COPY --from=builder /app/migrations ./migrations
EXPOSE 3000
CMD ["./home_inventory"]
