# Home Inventory API — Deployment Summary

A complete record of everything we did to get the Rust/Axum API deployed on Fly.io with Neon Postgres.

---

## 1. Push Code to GitHub

Initialized git, committed code, and pushed to a new GitHub repo.

```bash
git init
git add *
git commit -am "Initial commit"
git remote add origin https://github.com/braxton-lowe/home_inventory.git
```

### Fixes along the way

**Branch name mismatch** — local was `master`, GitHub expected `main`:
```bash
git branch -m master main
git push -u origin main
```

**Non-fast-forward error** — GitHub had an auto-generated README:
```bash
git pull origin main --rebase
```

**Merge conflict in README** — resolved manually, then:
```bash
git add README.md
git rebase --continue
git push -u origin main
```

---

## 2. Set Up Neon Postgres (Free Tier)

Chose Neon over Fly's managed Postgres to stay on the free tier.

1. Created account at [neon.tech](https://neon.tech)
2. Created a new project (`home-inventory`)
3. Grabbed the **direct** connection string (non-pooler URL) from the dashboard

### Verify connection
```bash
# Using a throwaway Docker container (no local psql needed)
docker run -it --rm postgres:17 psql "your_neon_connection_string"

# Inside psql
\l    # list databases
\dt   # list tables
\q    # quit
```

### Run migrations against Neon

First, reinstall sqlx-cli with TLS support:
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls --force
```

Then run migrations:
```bash
DATABASE_URL="your_neon_direct_connection_string" sqlx migrate run
```

Confirmed tables were created:
```
_sqlx_migrations, food_items, grocery_trips, locations, meals, trip_items
```

---

## 3. Prepare SQLx Offline Cache

SQLx `query!` macros verify SQL against a live DB at compile time. Docker builds have no DB, so we generate a cache instead.

```bash
DATABASE_URL="your_neon_direct_connection_string" cargo sqlx prepare
```

> Use the **direct** connection string (no `-pooler` in hostname) — the pooler doesn't support prepared statements needed by `cargo sqlx prepare`.

This generates a `.sqlx/` folder. Commit it:
```bash
git add .sqlx
git commit -m "Add sqlx query cache for offline builds"
git push
```

---

## 4. Create Dockerfile

Multi-stage build — compiles Rust in a builder image, copies just the binary to a slim runtime image.

```dockerfile
# --- Build stage ---
FROM rust:latest AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/home_food_inventory .
COPY --from=builder /app/migrations ./migrations
EXPOSE 3000
CMD ["./home_food_inventory"]
```

Key details:
- `SQLX_OFFLINE=true` tells SQLx to use the `.sqlx/` cache instead of a live DB
- Binary name `home_food_inventory` matches `name` in `Cargo.toml`
- `migrations/` is copied so SQLx can run them at startup (handled in `main.rs`)

---

## 5. Deploy on Fly.io

### Install and log in
```bash
brew install flyctl
fly auth login
```

### Launch the app
```bash
fly launch --no-deploy
```

### Set secrets
```bash
fly secrets set DATABASE_URL="your_neon_pooler_connection_string"
fly secrets set AUTH_USERNAME=your_username
fly secrets set AUTH_PASSWORD=your_password
```

> Use the **pooler** connection string here for the running app (better for connection management at runtime).

### fly.toml (final working version)

```toml
app = 'app-floral-violet-165'
primary_region = 'iad'

[build]

[env]
  PORT = '3000'

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = 'stop'
  auto_start_machines = true
  min_machines_running = 0
  processes = ['app']

[[vm]]
  memory = '256mb'
  cpus = 1
  memory_mb = 256
```

> Critical: `PORT` in `[env]` and `internal_port` in `[http_service]` must match. App reads `PORT` from env via `config.rs` and binds to it, so Fly must route to the same port.

### Deploy
```bash
fly deploy
```

### Useful Fly commands
```bash
fly logs          # tail live logs
fly open          # open app in browser
fly secrets list  # view set secrets (values hidden)
fly status        # check machine status
```

---

## 6. Verify

```bash
curl https://app-floral-violet-165.fly.dev/health
# {"status":"ok"}
```

---

## Final Stack

| Layer | Technology | Host |
|---|---|---|
| API | Rust / Axum | Fly.io (free tier) |
| Database | PostgreSQL | Neon (free tier) |
| Migrations | SQLx | Runs automatically at startup |
| Auth | HTTP Basic Auth | Via `AUTH_USERNAME` / `AUTH_PASSWORD` env vars |

**Total monthly cost: $0**
