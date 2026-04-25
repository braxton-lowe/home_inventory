# PostgreSQL: Local to Remote Dump Guide

How to migrate data from a local Dockerized Postgres instance to a remote Postgres provider (e.g. Neon, Supabase, Railway, etc.).

---

## Overview

A `pg_dump` exports your database contents to a `.sql` file which can then be replayed against any compatible Postgres instance. We use `--data-only` when the remote DB already has the schema (e.g. migrations have already run), to avoid conflicts.

---

## Prerequisites

- Local Postgres running in Docker
- Remote Postgres instance with schema already created (migrations ran)
- Remote **direct** connection string (not a pooler URL — poolers don't handle multi-statement transactions well)
- Docker installed locally (we use throwaway containers instead of installing `psql`/`pg_dump` locally)

---

## Step 1 — Find Your Local Container Name

```bash
docker ps
```

Look for your Postgres container and grab its name or ID, e.g. `c2392060d2c2`.

---

## Step 2 — Dump Data from Local Container

```bash
docker exec -t <container_name_or_id> pg_dump \
  -U <db_user> \
  -d <db_name> \
  --data-only \
  --no-owner \
  -f /tmp/data_dump.sql
```

**Flags explained:**
- `--data-only` — skips schema/table creation, only dumps rows. Use this when the remote DB already has your schema.
- `--no-owner` — strips ownership commands that would fail if your remote DB user has a different name
- `-f /tmp/data_dump.sql` — writes the dump inside the container at this path

**Example:**
```bash
docker exec -t c2392060d2c2 pg_dump \
  -U grocery_user \
  -d home_food_inventory \
  --data-only \
  --no-owner \
  -f /tmp/data_dump.sql
```

---

## Step 3 — Copy Dump Out of the Container

```bash
docker cp <container_name_or_id>:/tmp/data_dump.sql ./data_dump.sql
```

This copies the `.sql` file from inside the container to your current directory on your host machine.

---

## Step 4 — Restore into Remote DB

Use a throwaway Postgres Docker container to run `psql` without needing it installed locally. Mount your dump file into it:

```bash
docker run -it --rm \
  -v $(pwd)/data_dump.sql:/tmp/data_dump.sql \
  postgres:17 \
  psql 'your_remote_direct_connection_string' \
  -f /tmp/data_dump.sql
```

> **Use single quotes** around the connection string to prevent the shell from interpreting special characters in the password.

> **Use the direct connection string**, not the pooler URL. Poolers (like Neon's `-pooler` endpoint) don't support the multi-statement transactions that `psql` uses during a restore.

### Expected output

```
SET
SET
...
COPY 6      ← rows inserted into table 1
COPY 22     ← rows inserted into table 2
COPY 5      ← rows inserted into table 3
```

Any `duplicate key` errors on metadata tables like `_sqlx_migrations` are **harmless** — it just means those rows already exist on the remote.

---

## Step 5 — Verify the Restore

```bash
docker run -it --rm postgres:17 psql 'your_remote_direct_connection_string' \
  -c "SELECT COUNT(*) FROM your_table;"
```

Confirm the row counts match what you expected.

---

## Step 6 — Clean Up

Delete the dump file from your local machine — it contains your data and shouldn't be committed to git:

```bash
rm data_dump.sql
```

Add it to your `.gitignore`:
```
*.sql
```

---

## Common Errors

| Error | Cause | Fix |
|---|---|---|
| `password authentication failed` | Wrong password in connection string | Grab a fresh connection string from your provider's dashboard |
| `duplicate key value violates unique constraint` | Rows already exist on remote | Harmless if it's metadata tables; for data tables, clear the remote table first |
| `connection refused` | Using pooler URL instead of direct | Switch to the direct connection string |
| Special characters in password getting mangled | Double quotes in shell | Use single quotes around the connection string |

---

## Full Command Reference

```bash
# 1. Find container
docker ps

# 2. Dump data only
docker exec -t <container> pg_dump -U <user> -d <dbname> --data-only --no-owner -f /tmp/data_dump.sql

# 3. Copy out of container
docker cp <container>:/tmp/data_dump.sql ./data_dump.sql

# 4. Restore to remote
docker run -it --rm \
  -v $(pwd)/data_dump.sql:/tmp/data_dump.sql \
  postgres:17 \
  psql 'your_direct_connection_string' \
  -f /tmp/data_dump.sql

# 5. Verify
docker run -it --rm postgres:17 psql 'your_direct_connection_string' \
  -c "SELECT COUNT(*) FROM your_table;"

# 6. Clean up
rm data_dump.sql
```

---

## Notes

- This guide uses `--data-only` because the remote schema already exists. If you need to migrate schema too, drop `--data-only` and add `--no-acl` to avoid permission errors.
- If you have foreign key constraints, the order of inserts matters. `pg_dump` handles this automatically by disabling triggers during restore — if you see FK violations, make sure you're using the full dump file and not cherry-picking tables.
- For large datasets, consider using `pg_dump -Fc` (custom format) and `pg_restore` instead of plain SQL — it's faster and supports parallel restore.
