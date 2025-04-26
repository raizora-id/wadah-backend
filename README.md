# Wadah Backend

A modern Rust backend application built with Clean Architecture principles using Axum framework, following 12-factor app methodology.

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

## 12-Factor App Principles

This application follows the 12-factor app methodology:

1. **Codebase**: One codebase tracked in Git
2. **Dependencies**: Explicitly declared in Cargo.toml
3. **Config**: Stored in environment variables and config files
4. **Backing Services**: Treated as attached resources (PostgreSQL, Redis, MinIO)
5. **Build, Release, Run**: Strictly separated stages
6. **Processes**: Stateless and share-nothing
7. **Port Binding**: Self-contained with port configuration
8. **Concurrency**: Horizontal scaling through process model
9. **Disposability**: Fast startup and graceful shutdown
10. **Dev/Prod Parity**: Keep environments as similar as possible
11. **Logs**: Treated as event streams
12. **Admin Processes**: Run as one-off processes

## Configuration

Configuration follows a layered approach:

1. Base configuration (`config/base.yaml`)
2. Environment-specific configuration (`config/[environment].yaml`)
3. Local overrides (`config/local.yaml`, git-ignored)
4. Environment variables (prefixed with `APP_`)

Environment variables take precedence over file-based configuration.

## Getting Started

1. Install Rust and Cargo
2. Clone the repository
3. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
4. Start the infrastructure services:
   ```bash
   docker-compose up -d
   ```
5. Run the application:
   ```bash
   cargo run -p api
   ```

## Features

- Clean Architecture
- 12-Factor App Methodology
- Graceful Shutdown
- Configuration Management
- Database Connection Pooling
- Request Timeout Middleware
- Health Check Endpoint
- Structured JSON Logging
- OpenTelemetry Integration
- Database Migration Management

## Development

- Format code: `cargo fmt`
- Run lints: `cargo clippy`
- Run tests: `cargo test`
- Run with different config:
  ```bash
  RUST_ENV=production cargo run -p api
  ```

## Production

For production deployment:

1. Set appropriate environment variables
2. Ensure proper logging configuration
3. Configure monitoring and tracing
4. Set up health check monitoring
5. Configure appropriate resource limits
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
