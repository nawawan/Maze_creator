# CLAUDE.md

This file provides guidance for Claude when working in this repository.

## Project Overview

Rust backend for nawawan, a blog platform. Uses clean architecture with layered crates.

**Stack:** Rust (edition 2024) · Axum 0.8 · SQLx + PostgreSQL · Redis · AWS S3 (Cloudflare R2)

## Architecture

```
backend/
├── src/            # Binary entrypoint (main.rs) + SQL migrations
├── handler/        # HTTP layer: Axum routes, extractors, request/response models
├── usecase/        # Business logic: services, domain models, repository traits
├── storage/        # Data access: PostgreSQL + Redis + S3 implementations
└── shared/         # Config structs shared across crates
```

Dependency direction: `handler` → `usecase` → `storage` → `shared`

## Common Commands

```bash
# Start dependencies
docker compose up -d

# Run migrations
sqlx migrate run --source src/migrations

# Build & run
cargo build
cargo run

# Lint & format
cargo clippy
cargo fmt

# Test (requires running DB + Redis)
cargo test --workspace

# Utilities
cargo make -p backend create-uuid
cargo make -p backend create-hash {password} {pepper} {salt}
```

## Key Environment Variables

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | PostgreSQL connection URL |
| `REDIS_HOST`, `REDIS_PORT` | Redis connection |
| `PASSWORD_PEPPER` | Argon2 password hashing pepper |
| `CLOUDFLARE_ACCOUNT_ID`, `CLOUDFLARE_ACCESS_KEY_ID`, `CLOUDFLARE_SECRET_ACCESS_KEY` | R2 storage |
| `PAGE_HOST`, `BLOG_PAGE` | Public URLs for the frontend |
| `SQLX_OFFLINE=true` | Enable SQLx offline mode for tests |

## API Endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/` | Hello world |
| GET | `/health` | Health check |
| GET | `/health/db` | DB connectivity check |
| POST | `/api/blogs` | Create blog (auth required) |
| GET | `/api/blogs` | List blogs |
| POST | `/api/blogs/images` | Upload blog image (auth required) |
| POST | `/users/admin/login` | Admin login (sets auth cookie) |
| POST | `/users/logout` | Logout (clears auth cookie) |

## Code Conventions

- Auth is cookie-based (migrated from token-based)
- Errors: use `thiserror` in each layer; map to HTTP errors in `handler/src/error.rs`
- Async traits use `async-trait` crate
- Mocking in tests uses `mockall`
- Logging: structured JSON via `tracing` + `tracing-subscriber`
- Password hashing: Argon2 with per-user salt and global pepper
- UUIDs: v7 (time-sortable)

## CI/CD

GitHub Actions (`.github/workflows/backend_dev.yaml`):
- Lint: `cargo clippy` + `cargo fmt -- --check`
- Test: `cargo test --workspace`
- Triggers: push to `feature/**`, pull requests

## Database

- **Local dev:** PostgreSQL 14 via Docker Compose
- **Production:** CockroachDB (some sqlx compatibility quirks in tests)
- Migrations live in `src/migrations/` (18 files, run with `sqlx migrate run`)
