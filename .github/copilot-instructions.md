# Workspace Instructions for Copilot

## Project Overview

This workspace contains a microservices platform with:

- **Backend**: Rust-based microservices running in Docker containers
- **Frontend**: Vite + React application
- **Matrix**: Matrix protocol integration for decentralized communication (forum/chat structure)

## Architecture

- Microservices architecture with individual Rust services
- Docker containerization for all services
- React frontend built with Vite for fast development

## Development Guidelines

- Each microservice should be independently deployable and scalable (scalable only if it makes sense)
- Use Docker Compose for local development orchestration
- Follow Rust best practices and idiomatic patterns
- Use TypeScript for React components with shadcn and tailwind where possible
- Maintain clear separation between frontend and backend concerns

## Project Structure

- `services/` - Rust microservices
- `frontend/` - Vite + React application
- `docker/` - Docker configuration files
- `docs/` - Project documentation
- `scripts/` - Utility scripts for development and deployment
- `project_docs/` - Project-specific documents and planning files
- `project_status/` - Current status reports and roadmaps
- `temp/` - temporary files and experiments (AI agent work directory for testing and logging when building and testing code)

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

## Project power/permission structure

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

The term "inverted pyramid" is perfect and I was wrong to suggest changing it. This accurately describes:

1.  Visual structure: Inverted triangle shape
2.  Power flow: Users have the most authority
3.  Revolutionary nature: Complete reversal of traditional hierarchies

This is a user-sovereignty-first, bottom-up power structure where:

- Users (many, powerful) → top of inverted pyramid
- Global admins (few, servants) → bottom of inverted pyramid

The term Grassroots or Grassroots Empowerment should be replaces with "User Sovereignty" to better reflect the focus on individual user power and control in this inverted pyramid model or better yet, Unified model.
