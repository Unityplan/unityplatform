# Development Tools

Scripts for installing and managing development tools.

## Scripts

### install-dev-tools.sh

Install development dependencies for Unity Platform.

**Usage:**
```bash
./tools/install-dev-tools.sh
```

**Installs:**
- `sqlx-cli` - Database migrations and compile-time SQL verification
- `cargo-watch` - Auto-rebuild Rust services on file changes
- Other Rust development tools

---

### scaffold-service.sh

Create a new Rust microservice with standard structure.

**Usage:**
```bash
./tools/scaffold-service.sh <service-name>
```

**Example:**
```bash
./tools/scaffold-service.sh notification-service
```

**Creates:**
- `services/<service-name>/`
- `Cargo.toml` with standard dependencies
- `src/main.rs` - Server setup with middleware
- `src/handlers/` - HTTP request handlers
- `src/models/` - Request/Response types
- `src/services/` - Business logic
- `.env.example` - Configuration template
- `README.md` - Service documentation

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added
- Organized development tools into `tools/` subdirectory

#### Changed
- **BREAKING**: Scripts moved from `scripts/` root to `scripts/tools/`

---

**Category**: Development Tools  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
