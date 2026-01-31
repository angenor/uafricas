# UAfricas Backend

Backend API built with Actix Web (Rust).

## Requirements

- Rust 1.93+

## Getting Started

```bash
# Build the project
cargo build

# Run in development mode
RUST_LOG=info cargo run

# Run in release mode
cargo build --release
./target/release/uafricas_backend
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| HOST | 127.0.0.1 | Server host |
| PORT | 8080 | Server port |
| RUST_LOG | - | Log level (info, debug, error) |

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/ | API root |
| GET | /api/health | Health check |
