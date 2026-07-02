# SETUP_GUIDE

## Requirements
- Rust
- PostgreSQL
- Cargo

## Environment Variables

Create a `.env` file:

```env
DATABASE_URL=postgres://postgres:password@localhost/iam
JWT_SECRET=your-secret
PORT=3000
```

## Run Database Migrations

```bash
sqlx migrate run
```

## Start Server

```bash
cargo run
```

## Run Tests

```bash
cargo test
```

The API will be available at:

```text
http://localhost:3000
```
