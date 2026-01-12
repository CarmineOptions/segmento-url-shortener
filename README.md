# Segmento Url Shortener

Simple URL shortener API built with Axum and Diesel.

## Requirements

- Rust toolchain (edition 2024 compatible)
- Postgres (via `DATABASE_URL`)

## Run locally

```bash
export DATABASE_URL=postgres://user:pass@localhost:5432/segmento
cargo run -p api
```

The server listens on `0.0.0.0:3000`.

## Docker

```bash
docker build -t segmento-api .
docker run --rm -p 3000:3000 --env-file .env segmento-api
```

## Endpoints

- `POST /manager-api/links/create` -> JSON `{ "target_url": "https://example.com" }`
- `GET /manager-api/links/{code}` -> JSON link data
- `GET /{code}` -> 302 redirect to target URL
