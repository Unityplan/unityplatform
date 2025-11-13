# territory-service

**Port:** 8008  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Scaffolded (0/7 endpoints implemented)

## Overview

Territory service manages the global territory registry, per-territory settings, and territory manager assignments. It enforces the dual ownership model where Platform Managers control infrastructure and Territory Managers control sovereignty settings.

## Quick Start

```bash
# Copy environment configuration
cp .env.example .env

# Build the service
cargo build

# Run the service
cargo run
```

## Environment Variables

See `.env.example` for all required configuration.

## API Endpoints

- **GET** `/api/v1/service/health` - Health check ✅
- **GET** `/api/v1/territories` - List all territories (planned)
- **GET** `/api/v1/territories/{code}` - Get territory details (planned)
- **PATCH** `/api/v1/territories/{code}/settings` - Update settings (planned)
- **GET** `/api/v1/territories/{code}/managers` - List managers (planned)
- **POST** `/api/v1/territories` - Create territory (planned)
- **PATCH** `/api/v1/territories/{code}/status` - Update status (planned)
- **POST** `/api/v1/territories/{code}/managers` - Assign manager (planned)

## Documentation

- Full API documentation: `/docs/architecture/services/territory-service/API.md`
- Database schema: `/docs/architecture/services/territory-service/DATABASE.md`
- Swagger UI: `http://localhost:8008/swagger-ui/`

## Database Tables

- `global.territories_registry` - Global territory registry
- `territory_{code}.territory_settings` - Territory settings (source of truth)
- `territory_{code}.territory_managers` - Manager assignments
- `territory_{code}.territory_stats` - Statistics (read-only)

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Architecture

Territory service follows the standard Unity Platform microservice pattern:

```
src/
├── main.rs              # Server setup, middleware, OpenAPI
├── lib.rs               # Public exports
├── handlers/            # HTTP request handlers
│   ├── mod.rs
│   └── territory.rs
├── models/              # Request/Response types
│   ├── mod.rs
│   └── territory.rs
└── services/            # Business logic
    ├── mod.rs
    └── territory.rs
```

## Dual Ownership Model

**Platform Manager** controls:

- `code`, `pod_url`, `api_url`, `status`

**Territory Manager** controls (replicates from pod to global):

- `name`, `display_name`, `description`, `language_code`, `timezone`, `currency_code`

Settings updated in `territory_{code}.territory_settings` automatically replicate to `global.territories_registry` via trigger.
