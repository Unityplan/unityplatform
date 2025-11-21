# Unity Platform Services

Rust-based microservices for the Unity Platform.

**🔗 Issue Tracking:** [All Service Issues](http://localhost:3000/henrik/unity_platform/issues?labels=9,10,11,12,13,19,20,21,22,23) | [Forgejo Milestone](http://localhost:3000/henrik/unity_platform/milestones)

---

## 📊 Service Status

| Service | Version | Status | Issues | Description |
|---------|---------|--------|--------|-------------|
| **shared-lib** | 0.1.0-alpha.1 | ✅ Complete | - | Shared utilities, middleware, database |
| **auth-service** | 0.1.0-alpha.1 | ✅ Complete | [#51-#58](http://localhost:3000/henrik/unity_platform/issues?labels=9) | Authentication & JWT tokens |
| **user-service** | 0.1.0-alpha.1 | ✅ Complete | [#59-#67](http://localhost:3000/henrik/unity_platform/issues?labels=10) | User profiles & settings |
| **territory-service** | 0.1.0-alpha.1 | ✅ Complete | [#31-#33](http://localhost:3000/henrik/unity_platform/issues?labels=12) | Territory management |
| **badge-service** | 0.1.0-alpha.1 | ✅ Complete | [#34-#41](http://localhost:3000/henrik/unity_platform/issues?labels=11) | Badges & achievements |
| **utility-service** | 0.1.0-alpha.1 | ✅ Complete | [#120-#136](http://localhost:3000/henrik/unity_platform/issues?labels=13) | Favicon, language registry |
| **course-service** | - | 📋 Planned | [#68-#73](http://localhost:3000/henrik/unity_platform/issues?labels=19) | LMS & course management |
| **matrix-gateway** | - | 📋 Planned | [#74-#79](http://localhost:3000/henrik/unity_platform/issues?labels=20) | Matrix protocol bridge |
| **ipfs-service** | - | 📋 Planned | [#82-#87](http://localhost:3000/henrik/unity_platform/issues?labels=21) | IPFS storage integration |
| **forum-service** | - | 📋 Planned | [#88-#95](http://localhost:3000/henrik/unity_platform/issues?labels=22) | Forum & discussions |
| **translation-service** | - | 📋 Planned | [#96-#98](http://localhost:3000/henrik/unity_platform/issues?labels=23) | i18n & translation |

---

## 🏗️ Architecture

All services follow a consistent architecture pattern:

```
services/
├── shared-lib/           # Shared library
│   ├── src/
│   │   ├── config/       # Configuration management
│   │   ├── database/     # Database connection & utilities
│   │   ├── error/        # Error types & handling
│   │   ├── middleware/   # HTTP middleware (logging, CORS, etc.)
│   │   ├── nats/         # NATS message bus client
│   │   └── version/      # Version management
│   └── migrations/       # Database migrations (SQLx)
│
└── [service-name]/       # Individual microservice
    ├── src/
    │   ├── main.rs       # Server setup & middleware chain
    │   ├── lib.rs        # Public exports
    │   ├── handlers/     # HTTP request handlers
    │   ├── models/       # Request/Response types
    │   └── services/     # Business logic
    ├── tests/            # Integration tests
    ├── Cargo.toml        # Dependencies
    └── README.md         # Service documentation
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres
```

### Development

```bash
# Build all services
cargo build

# Run specific service
cd services/auth-service
cargo run

# Run tests
cargo test

# Run with watch mode
cargo watch -x run
```

---

## 🔧 Service Details

### shared-lib

**Purpose:** Shared utilities and middleware for all services

**Key Components:**

- `AppConfig` - Configuration management with environment variables
- `Database` - PostgreSQL connection pooling with SQLx
- `AppError` - Unified error handling
- `Middleware` - Logging, CORS, rate limiting, security headers
- `NatsClient` - NATS message bus integration
- `Version` - Runtime version information

**Usage:**

```rust
use shared_lib::{AppConfig, Database, LoggingMiddleware};

let config = AppConfig::from_env()?;
let db = Database::connect(&config.database_url).await?;

HttpServer::new(move || {
    App::new()
        .wrap(LoggingMiddleware::development())
        .app_data(web::Data::new(db.clone()))
})
```

**Issues:** N/A (foundational library)

---

### auth-service

**Purpose:** User authentication and JWT token management

**Port:** 8001  
**Base Path:** `/api/v1/auth`  
**Completed:** November 15, 2025  
**Issues:** [#51-#58](http://localhost:3000/henrik/unity_platform/issues?labels=9) (8 closed issues)

**Endpoints:**

- `POST /register` - Register new user
- `POST /login` - Authenticate user
- `POST /refresh` - Refresh access token
- `POST /logout` - Invalidate refresh token
- `POST /verify` - Verify access token

**Documentation:** See `auth-service/README.md`

---

### user-service

**Purpose:** User profile and social features management

**Port:** 8002  
**Base Path:** `/api/v1/users`  
**Completed:** November 16, 2025  
**Issues:** [#59-#67](http://localhost:3000/henrik/unity_platform/issues?labels=10) (9 closed issues)

**Features:**

- Profile management (bio, avatar, location, links)
- User settings (privacy, notifications, language)
- Social connections (follow/unfollow, friends list)
- Data export & GDPR compliance
- Account deletion requests

**Documentation:** See `user-service/README.md`

---

### territory-service

**Purpose:** Territory registration and management

**Port:** 8003  
**Base Path:** `/api/v1/territories`  
**Completed:** November 16, 2025  
**Issues:** [#31-#33](http://localhost:3000/henrik/unity_platform/issues?labels=12) (3 closed issues)

**Features:**

- Territory registration (countries, regions, first nations)
- Territory settings & configuration
- Manager assignment & permissions
- Territory statistics & analytics

**Documentation:** See `territory-service/README.md`

---

### badge-service

**Purpose:** Badge system and achievement tracking

**Port:** 8004  
**Base Path:** `/api/v1/badges`  
**Completed:** November 16, 2025  
**Issues:** [#34-#41](http://localhost:3000/henrik/unity_platform/issues?labels=11) (8 closed issues)

**Features:**

- Badge registry (global badge definitions)
- User badge awards
- Progress tracking
- Completion tracking & verification

**Documentation:** See `badge-service/README.md`

---

### utility-service

**Purpose:** Utility endpoints for common functionality

**Port:** 8011  
**Base Path:** `/api/v1/utility`  
**Completed:** November 16, 2025  
**Issues:** [#120-#136](http://localhost:3000/henrik/unity_platform/issues?labels=13) (18 closed issues)

**Features:**

- Favicon fetching for external URLs
- Language registry & metadata
- Language code validation
- Language name lookup

**Documentation:** See `utility-service/README.md`

---

### course-service (Planned)

**Purpose:** Learning Management System (LMS) for courses

**Port:** 8005  
**Base Path:** `/api/v1/courses`  
**Status:** Planned (Stage 7)  
**Issues:** [#68-#73](http://localhost:3000/henrik/unity_platform/issues?labels=19) (6 open issues)

**Planned Features:**

- Course creation & management
- Module & lesson structure
- Enrollment management
- Progress tracking
- Completion certificates

**Get Started:** [Issue #68 - Scaffold course-service](http://localhost:3000/henrik/unity_platform/issues/68)

---

### matrix-gateway (Planned)

**Purpose:** Matrix protocol integration for federated communication

**Port:** 8006  
**Base Path:** `/api/v1/matrix`  
**Status:** Planned (Stage 8)  
**Issues:** [#74-#79](http://localhost:3000/henrik/unity_platform/issues?labels=20) (6 open issues)

**Planned Features:**

- Matrix homeserver integration
- Room creation & management
- Message bridging
- User authentication via Matrix
- Federation with other homeservers

**Get Started:** [Issue #74 - Matrix gateway setup](http://localhost:3000/henrik/unity_platform/issues/74)

---

### ipfs-service (Planned)

**Purpose:** IPFS storage integration for decentralized file storage

**Port:** 8007  
**Base Path:** `/api/v1/ipfs`  
**Status:** Planned (Stage 9)  
**Issues:** [#82-#87](http://localhost:3000/henrik/unity_platform/issues?labels=21) (6 open issues)

**Planned Features:**

- File upload to IPFS
- File retrieval via CID
- Pin management
- Metadata storage
- Content addressing

**Get Started:** [Issue #82 - IPFS service setup](http://localhost:3000/henrik/unity_platform/issues/82)

---

### forum-service (Planned)

**Purpose:** Forum & discussion board (Matrix-based)

**Port:** 8008  
**Base Path:** `/api/v1/forums`  
**Status:** Planned (Stage 10)  
**Issues:** [#88-#95](http://localhost:3000/henrik/unity_platform/issues?labels=22) (8 open issues)

**Planned Features:**

- Forum creation & management
- Thread & post management
- Moderation tools
- Search & filtering
- Notifications

**Get Started:** [Issue #88 - Forum service setup](http://localhost:3000/henrik/unity_platform/issues/88)

---

### translation-service (Planned)

**Purpose:** Internationalization & real-time translation

**Port:** 8009  
**Base Path:** `/api/v1/translations`  
**Status:** Planned (Stage 11)  
**Issues:** [#96-#98](http://localhost:3000/henrik/unity_platform/issues?labels=23) (3 open issues)

**Planned Features:**

- Translation API integration
- Language detection
- Translation caching
- Terminology management

**Get Started:** [Issue #96 - Translation service setup](http://localhost:3000/henrik/unity_platform/issues/96)

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for specific service
cd services/auth-service
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run integration tests only
cargo test --test '*'
```

---

## 🔍 Debugging

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run specific service with trace logging
RUST_LOG=trace cargo run --bin auth-service

# Use rust-lldb for debugging
rust-lldb target/debug/auth-service
```

---

## 📚 Documentation

- **Architecture:** `docs/architecture/services/`
- **API Specifications:** Each service has `API.md`
- **Database Schemas:** Each service has `DATABASE.md`
- **Development Guide:** `docs/guides/backend/rust-service-development.md`
- **Forgejo Workflow:** `docs/guides/development/forgejo-workflow.md`

---

## 🤝 Contributing

1. **Pick an issue:** Browse [open service issues](http://localhost:3000/henrik/unity_platform/issues?state=open&labels=9,10,11,12,13,19,20,21,22,23)
2. **Create branch:** `git checkout -b issue-XX-description`
3. **Implement:** Follow service architecture pattern
4. **Test:** Achieve >80% code coverage
5. **Document:** Update API.md and README.md
6. **Submit PR:** Link to issue, request review

See [Forgejo Workflow Guide](../docs/guides/development/forgejo-workflow.md) for details.

---

## 📊 Statistics

**Total Services:** 11 (6 complete, 5 planned)  
**Total Endpoints:** ~80 (across all services)  
**Code Coverage:** >80% (target)  
**Lines of Code:** ~15,000 (excluding tests)  
**Tests:** ~200 (unit + integration)

---

## 🔗 Related

- [Frontend README](../app/README.md)
- [Phase 1 Status](../docs/status/current/phase-1-status.md)
- [All Forgejo Issues](http://localhost:3000/henrik/unity_platform/issues)
