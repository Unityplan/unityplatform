# Workspace Instructions for Copilot

## Project Overview

**Platform Name:** Unity Platform  
**Platform Type:** User sovereignty-first learning and collaboration platform  
**Version:** 0.1.0-alpha.1 (MVP Phase 1 - Early Development)  
**Status:** Infrastructure complete, auth-service operational, backend services in development  
**Example Deployment:** unityplan.org (test project using this platform)  
**Database Naming:** `unityplatform_${TERRITORY_CODE}` (e.g., `unityplatform_dk` for Denmark)  
**Important:** Use `unityplatform` (not `unityplan`) for all new naming

This workspace contains a microservices platform with:

- **Backend**: Rust-based microservices running in Docker containers
- **Frontend**: Vite + React application (not yet started)
- **Matrix**: Matrix protocol integration for decentralized communication (planned)
- **Multi-Pod Architecture**: Territory-based pod deployment (Denmark pod operational)

## Quick References

- **Version Matrix**: See `VERSIONS.md` for all component versions
- **Documentation**: `docs/` directory (consolidated structure)
- **Status**: `docs/status/current/phase-1-status.md` (18% complete)
- **Database Schema**: `services/shared-lib/migrations/` (3 migrations applied)
- **Scripts**: `scripts/README.md` for all utility scripts
- **Local Database Credentials (DK Pod)**:
  - Host: `localhost:5432`
  - Database: `unityplatform_dk`
  - User: `unityplatform`
  - Password: `unityplatform_dev_password_dk`
  - Connection string: `postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk`
- **Naming Convention**:
  - Networks: `unityplatform-mesh-network`, `unityplatform-global-net`, `unityplatform-pod-${ID}-net`
  - Containers: `service-postgres-${TERRITORY}`, `monitoring-*`, `dev-*`
  - Volumes: `unityplatform-${PROJECT}_${NAME}`
  - Projects: `unityplatform-dev`, `unityplatform-monitoring`, `unityplatform-pod-${ID}`

## Architecture

- Microservices architecture with individual Rust services
- Docker containerization for all services
- React frontend built with Vite for fast development

## Development Guidelines

- **Current Stage**: Alpha (0.1.0-alpha.1) - Infrastructure complete, services in development
- Each microservice should be independently deployable and scalable (scalable only if it makes sense)
- Use Docker Compose for local development orchestration
- Follow Rust best practices and idiomatic patterns
- Use TypeScript for React components with shadcn and tailwind where possible
- Maintain clear separation between frontend and backend concerns
- **Versioning**: Follow SemVer 2.0.0 - see `docs/guides/development/versioning-strategy.md`
- **Changelogs**: Update `CHANGELOG.md` and service-specific changelogs for all changes
- **Version Info**: Use `shared_lib::version` module for runtime version access

## AI Assistant Guidelines

- **Terminal Commands**: NEVER chain commands with `&&` - always use single commands for easier auto-approval
  - ❌ Bad: `cd services && cargo build && cargo test`
  - ✅ Good: Single command per tool call
- **MCP Tools**: ALWAYS use MCP (Model Context Protocol) tools when available:
  - PostgreSQL operations: Use `pgsql_*` tools instead of raw `psql` commands
  - Git operations: Use git MCP tools instead of terminal git commands (where it makes sense)
  - File operations: Use file MCP tools when appropriate
  - Database queries: Use `pgsql_query` and `pgsql_modify` instead of `docker exec psql`
- **Reasoning**: MCP tools provide better context, error handling, and user experience

## Forgejo Issue Management

**Overview**: This project uses Forgejo (self-hosted Git forge at `localhost:3000`) for issue tracking. All Phase 1 MVP work is tracked via 137 issues across 14 stages.

**Helper Scripts** (in `scripts/forgejo/`):

- `get-issue.sh <number>` - Retrieve and display issue details
- `create-issue.sh <title> <body> <labels> <milestone>` - Create new issue
- `update-issue.sh <number> [--title "..."] [--body "..."] [--state open|closed] [--labels "..."]` - Update issue
- `close-issue.sh <number> [comment]` - Close issue with optional comment

**Workflow**:

1. **When user mentions "#NNN" or "issue NNN"**: Use `get-issue.sh NNN` to read issue details
2. **When completing work**: Offer to close issue with `close-issue.sh NNN "Completed in <context>"`
3. **When creating tasks**: Use `create-issue.sh` with appropriate labels and milestone
4. **When updating status**: Use `update-issue.sh` to change state/labels

**Examples**:

```bash
# Read issue #147
./scripts/forgejo/get-issue.sh 147

# Close issue with comment
./scripts/forgejo/close-issue.sh 147 "Scaffolding script completed"

# Update issue state
./scripts/forgejo/update-issue.sh 147 --state closed

# Update issue labels
./scripts/forgejo/update-issue.sh 147 --labels "priority:high,status:done"
```

**Configuration**: Scripts use `.env` file in `scripts/forgejo/` with FORGEJO_TOKEN for authentication.

## Service Creation Pattern

When creating a new Rust microservice, follow this standard pattern:

### 1. **Check Documentation First**

- Review `docs/architecture/services/{service-name}/` for requirements
- Check API.md for endpoint specifications
- Review DATABASE.md for schema requirements
- Understand service boundaries (what it DOES and DOES NOT handle)

### 2. **Learn from Archived Services**

- Check `services/archived/{service-name}/` for previous implementation wisdom
- Reuse proven patterns (password hashing, token generation, etc.)
- **BUT**: Always modernize to use new shared-lib middleware and patterns

### 3. **Standard Service Structure**

```
services/{service-name}/
├── Cargo.toml
├── src/
│   ├── main.rs              # Server setup with middleware
│   ├── lib.rs               # Public exports
│   ├── handlers/            # HTTP request handlers
│   │   ├── mod.rs
│   │   └── {domain}.rs
│   ├── models/              # Request/Response types
│   │   ├── mod.rs
│   │   └── {domain}.rs
│   └── services/            # Business logic
│       ├── mod.rs
│       └── {domain}.rs
```

### 4. **Use New Middleware Stack**

```rust
// main.rs - Always use shared-lib middleware
use shared_lib::{
    AppConfig, Database, LoggingMiddleware, RequestIdMiddleware,
    SecurityHeadersMiddleware, cors, RateLimitMiddleware,
};

HttpServer::new(move || {
    App::new()
        // Priority 1 middleware
        .wrap(LoggingMiddleware::development())  // or ::production()
        .wrap(RequestIdMiddleware)
        // Priority 2 middleware
        .wrap(SecurityHeadersMiddleware::development())
        .wrap(cors::development())
        .wrap(RateLimitMiddleware::development(redis_client.clone()))
        // Routes
        .service(web::scope("/api/v1/{service}").configure(routes))
})
```

### 5. **Use Validation Extractors**

```rust
use shared_lib::ValidatedJson;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,
}

async fn create(body: ValidatedJson<CreateRequest>) -> HttpResponse {
    // body is guaranteed valid
}
```

### 6. **Inter-Service Communication**

- Use NATS for async communication between services
- **Graceful Degradation**: Handle missing services gracefully
- **Feature Flags**: Use environment variables to enable/disable inter-service calls during development
- Example: auth-service registration calls invitation-service (when available)

### 7. **Database Patterns**

- Use shared-lib Database connection
- Territory-aware queries: `territory_{code}.table_name`
- Global registries: `global.username_registry`, `global.email_registry`
- Use transactions for multi-step operations

### 8. **Error Handling**

- Use `shared_lib::AppError` for all errors
- Return `shared_lib::Result<T>` from functions
- Let middleware handle error responses automatically

### 9. **Service Scaffolding**

**CRITICAL**: When using the scaffolding script:

```bash
# ✅ CORRECT - Use base name only (script adds -service suffix)
./scripts/dev/scaffold-service.sh invitation 8004 "Description"
# Creates: invitation-service

# ❌ WRONG - Never include -service in the name
./scripts/dev/scaffold-service.sh invitation-service 8004 "..."
# Would create: invitation-service-service (WRONG!)
```

- Script location: `scripts/dev/scaffold-service.sh`
- Takes 3 arguments: `<base-name>` `<port>` `<description>`
- Automatically adds `-service` suffix to create service name
- After scaffolding, follow: `docs/guides/development/service-implementation-guide.md`

## Project Structure

- `services/` - Rust microservices workspace
  - `shared-lib/` - Shared library (config, database, error, nats) - v0.1.0-alpha.1
  - `auth-service/` - Authentication service (planned)
  - Future services: user-service, territory-service, badge-service, course-service, forum-service
- `docs/` - **Primary documentation** (consolidated, organized structure)
  - `project/` - Project definition (summary, overview, tech stack)
  - `architecture/` - System design (infrastructure, multi-pod, territory standard)
  - `guides/` - Implementation guides (deployment, development, operations)
  - `status/` - Progress tracking (roadmaps, current status)
- `docker/` - Docker configuration files and data volumes
- `pods/` - Territory pod configurations (denmark/, norway/, sweden/, europe/)
- `scripts/` - Utility scripts for development and deployment
- `temp/` - Temporary files and AI agent experiments
- `docs-archived/` - Old documentation (to be deleted)
- **Root Files**:
  - `VERSIONS.md` - Version matrix for all components
  - `CHANGELOG.md` - Platform-level change history
  - `.github/copilot-instructions.md` - This file

## Technologies Used

- Rust for backend microservices
- React with Vite for frontend development
- Docker and Docker Compose for containerization and orchestration

### 🧩 **Backend (Rust Services)**

| Component                       | Technology                                                        | Function                                                                |
| ------------------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Rust Language                   | 1.91.0                                                            | High-performance, secure system programming capabilities                |
| **HTTP API**                    | [`actix-web`](https://actix.rs/)                                  | REST API, routing, middleware, JSON handling                            |
| **WebSocket Gateway**           | [`tokio-tungstenite`](https://crates.io/crates/tokio-tungstenite) | Real-time communication between backend and frontend                    |
| **Database Layer**              | [`sqlx`](https://crates.io/crates/sqlx) + **TimescaleDB**         | Asynchronous database access and time-series storage                    |
| **Multi-tenant Isolation**      | PostgreSQL **schemas per tenant**                                 | Tenant separation in the same database (separate countries/territories) |
| **Job Queue / Message Bus**     | [`nats`](https://nats.io/)                                        | Inter-service communication and event distribution                      |
| **Authentication / SSO**        | [`openidconnect`](https://crates.io/crates/openidconnect) + JWT   | Single Sign-On (OIDC), session tokens, RBAC                             |
| **Configuration & Secrets**     | [`config`](https://crates.io/crates/config), `dotenvy`            | Environment configuration and service settings                          |
| **Logging & Metrics**           | `tracing`, `tracing-subscriber`, `opentelemetry`                  | Structured logging and observability                                    |
| **Background Jobs / Scheduler** | `tokio::task` / `cronback` / `async-cron`                         | Automations, timed scripts, policy execution                            |
| **Event Ledger (future)**       | Holochain-like module                                             | Cryptographically signed audit trail for events                         |
| **Containerization**            | **Docker Compose**                                                | Microservices, isolated environments                                    |
| **Service Routing**             | **Traefik / Linkerd (mTLS)**                                      | Service mesh + Zero-trust between containers                            |
| **Matrix Protocol**             | [`ruma`](https://crates.io/crates/ruma)                           | Decentralized communication and collaboration                           |
| **IPFS Integration**            | `ipfs-api` crate                                                  | Decentralized file storage and sharing                                  |
| **CI/CD**                       | **GitHub Actions**                                                | Automated testing, building, and deployment                             |

### Frontend Technology Stack

| Technology                    | Version             | Function                                                 |
| ----------------------------- | ------------------- | -------------------------------------------------------- |
| **Vite**                      | 5.x                 | Dev server and bundler                                   |
| **React**                     | 19.2.0              | Component-based UI (stable ecosystem, Tauri-ready)       |
| **TailwindCSS**               | 4.1.16              | Utility-first styling                                    |
| **shadcn/ui**                 | 3.5.0               | Accessible component library with theming                |
| **TanStack Router**           | 1.134.10            | Type-safe client-side routing                            |
| **TanStack Query**            | v5                  | Data fetching, caching, background refetching            |
| **Zustand**                   | latest              | State management (auth/UI state only, NOT data fetching) |
| **react-hook-form + zod**     | latest              | Form handling and validation                             |
| **TypeScript**                | latest              | Type-safe frontend logic                                 |
| **Vitest**                    | latest              | Unit testing (Vite-native)                               |
| **Testing Library**           | latest              | Component testing                                        |
| **shadcn-map**                | latest              | Interactive maps with markers (OpenStreetMap/Mapbox)     |
| **Matrix SDK**                | `matrix-js-sdk`     | Matrix protocol integration                              |
| **Holochain Client (future)** | `@holochain/client` | Holochain DNA module interaction                         |
| **Tauri (future)**            | latest              | Cross-platform desktop/mobile packaging                  |

**Stack Rationale:** React 18 chosen over React 19 for stable ecosystem during MVP phase. TanStack Query handles all server data fetching/caching. Zustand used only for auth tokens and UI state. See `docs/architecture/frontend-stack-rationale.md` for detailed decision rationale.

## Project Power/Permission Structure

The system IS visually an inverted pyramid - wide at top (users), narrow at bottom (global):

┌─────────────────────────────────────┐ ← WIDE (many users, most power)
│ USERS (Highest) │
├─────────────────────────────────────┤
│ COMMUNITIES │
├─────────────────────────────────────┤
│ TERRITORIES │
├─────────────────────────────────────┤
│ GLOBAL (Lowest) │
└─────────────────────────────────────┘ ← NARROW (few admins, least power)

This is revolutionary because:

- Traditional organizations = pyramid (narrow powerful top, wide powerless bottom)
- Your system = inverted pyramid (wide powerful top/users, narrow service-focused bottom/admins)

The term "inverted pyramid" is perfect and accurately describes:

1.  Visual structure: Inverted triangle shape
2.  Power flow: Users have the most authority
3.  Revolutionary nature: Complete reversal of traditional hierarchies

This is a user-sovereignty-first, bottom-up power structure where:

- Users (many, powerful) → top of inverted pyramid
- Global admins (few, servants) → bottom of inverted pyramid

## Natural Ecosystem Metaphor

To understand how this inverted pyramid functions as a living system, think of the platform as part of nature:

**🌰 Pod (Seed-Pod)**: Each territory deployment is like a seed pod - self-contained, capable of independent growth and reproduction.

**🌿 Roots (IT Infrastructure)**: Docker, PostgreSQL, NATS, Redis form the root system - hidden beneath the surface, providing essential nutrients and stability.

**🍄 Mycorrhizal Network (Global Level)**: The global federation layer acts like a mycorrhizal network - an underground fungal network connecting separate plants, sharing wisdom from elders through LMS teachings and exchanging knowledge between communities across different pods via forum structures.

**🌱 Stem Base (Territory Level)**: A single territory is like the base of a plant stem - managing flow between infrastructure and communities, coordinating local resources.

**🔗 Stalk Joints (Communities)**: Community structures are like the joints where branches emerge - connection points where collaboration branches out.

**🍃 Leaves (Guilds & Study Groups)**: Guilds and study groups are like leaves - where photosynthesis happens, converting knowledge into practical skills and energy.

**🌸 Flowers (Communities with People)**: Communities of active users bloom like flowers - the visible, vibrant expression where people gather, interact, and create. Users are the individual parts that make up the flower (petals, stamens, pistils) - each contributing to the whole.

**🌾 Seeds (New Knowledge)**: Flowers produce seeds of new knowledge and experience, spreading to create new pods, continuing the growth cycle.

**♻️ Energy Cycle**: Knowledge implementation manifests in communities and guilds (flowers and leaves) as shared energy returned to the soil - enriching the entire ecosystem, making all flowers grow bigger and stronger together.

This organic model emphasizes:

- **Interconnection**: Like a forest, all parts support each other
- **Sovereignty**: Each pod grows independently while benefiting from the network
- **Wisdom Flow**: Knowledge circulates like nutrients through the mycorrhizal network
- **Regeneration**: Communities create new knowledge that enriches the whole system
- **Resilience**: Distributed structure ensures ecosystem thrives even if individual parts face challenges
