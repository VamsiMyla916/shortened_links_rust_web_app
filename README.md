# Rusty Word Shortener

A simple, pure-Rust URL shortener built as a tutorial web application. Instead of generating random strings, this app allows users to create custom, readable short links like `localhost:3000/borrow-checker`.

## Tech Stack

- **Runtime:** Tokio
- **Web Framework:** Axum
- **Frontend:** Askama (Server-Side Rendering)
- **Database:** SQLite & sqlx

## Current Project Status

- [x] Initialized project and configured dependencies.
- [x] Set up SQLite database with migrations and `UNIQUE` constraints.
- [x] Implemented Axum routing and Askama HTML templates.
- [ ] **Pending:** Containerize the application using Docker and Docker Volumes.

## How to Run Locally (Development)

1. Ensure you have Rust and `sqlx-cli` installed.
2. Clone this repository.
3. Run the database migrations to build the local database:
   `sqlx migrate run`
4. Start the web server:
   `cargo run`
5. Open your browser and navigate to `http://localhost:3000`
