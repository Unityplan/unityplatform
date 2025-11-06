# Workspace Instructions for Copilot

## Project Overview

**Platform:** UnityPlan - User sovereignty-first learning and collaboration platform  
**Version:** 0.1.0-alpha.1 (MVP Phase 1 - Early Development)  
**Status:** Infrastructure complete, backend services in development

This workspace contains a microservices platform with:

- **Backend**: Rust-based microservices running in Docker containers
- **Frontend**: Vite + React application (not yet started)
- **Matrix**: Matrix protocol integration for decentralized communication (planned)
- **Multi-Pod Architecture**: Territory-based pod deployment (Denmark pod operational)

## Quick References

- **Version Matrix**: See `VERSIONS.md` for all component versions
- **Documentation**: `docs/` directory (consolidated structure)
- **Status**: `docs/status/current/phase-1-status.md` (18% complete)
- **Database Schema**: `services/shared-lib/migrations/` (version: 20251105000001)

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

| Technology                    | Version             | Function                          |
| ----------------------------- | ------------------- | --------------------------------- |
| **Vite**                      | latest              | Dev server and bundler            |
| **React**                     | 19                  | Component-based UI                |
| **TailwindCSS**               | 4.1.16              | Utility-first styling             |
| **ShadCN**                    | 3.5.0               | Prebuilt components, theme system |
| **TanStack Router**           | 1.134.10            | Client-side routing               |
| **TypeScript**                | latest              | Type-safe frontend logic          |
| **Matrix SDK**                | `matrix-js-sdk`     | Matrix protocol integration       |
| **Holochain Client (future)** | `@holochain/client` | Holochain DNA module interaction  |
| **Tauri (future)**            | latest              | Mobile application packaging      |

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
