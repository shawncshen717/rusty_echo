# Rusty Echo

An async TCP echo server written in Rust using the [Tokio](https://tokio.rs/) runtime.

## Features

- Accepts multiple client connections using async tasks
- Echoes back any received messages
- Handles disconnects gracefully
- Uses `tokio::net` for non-blocking I/O

## ▶ How to Run

```bash
cargo run
