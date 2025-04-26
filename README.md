# Wadah Backend

A modern Rust backend application built with Clean Architecture principles using Axum framework.

## Project Structure

The project follows Clean Architecture principles and is organized as a monorepo with the following crates:

- `domain`: Core business logic and entities
  - Contains business entities, value objects, and repository interfaces
  - Has no external dependencies
  
- `application`: Use cases and application logic
  - Implements business use cases
  - Depends only on the domain layer
  
- `infrastructure`: External implementations
  - Implements repositories and external services
  - Contains database, storage, and messaging implementations
  - Depends on domain and application layers
  
- `api`: Web interface
  - Implements HTTP API using Axum
  - Handles routing, middleware, and request/response
  - Depends on all other layers

## Getting Started

1. Install Rust and Cargo
2. Clone the repository
3. Run the application:
   ```bash
   cargo run -p api
   ```

## Development

- Each crate can be tested independently: `cargo test -p <crate-name>`
- Run all tests: `cargo test`
- Format code: `cargo fmt`
- Check lints: `cargo clippy`

## Technologies

- Axum: Web framework
- SQLx: Database ORM
- Redis: Caching
- AWS S3: File storage
- Tower: Middleware
- Tracing: Logging and instrumentation
