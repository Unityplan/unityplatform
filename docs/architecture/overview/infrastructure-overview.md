# Platform Infrastructure Architecture

> **Note:** "unityplan.org" references in this document are examples from the test deployment only.

## 📋 Table of Contents

1. [Overview](#1-overview)
2. [System Architecture](#2-system-architecture)
3. [Microservices Architecture](#3-microservices-architecture)
4. [Multi-Territory Infrastructure](#4-multi-territory-infrastructure)
5. [Network & Security](#5-network--security)
6. [Data Layer](#6-data-layer)
7. [Matrix Protocol Integration](#7-matrix-protocol-integration)
8. [Deployment Architecture](#8-deployment-architecture)
9. [Observability](#9-observability)

---

## 1. Overview

The platform's infrastructure is designed around three core principles:

1. **Territory Sovereignty**: Each territory controls its own infrastructure and data
2. **Microservices Architecture**: Independent, scalable services with clear boundaries
3. **Progressive Decentralization**: Current centralized architecture evolves toward full decentralization

### Infrastructure Goals

| Goal | Implementation |
|------|----------------|
| **Scalability** | Independent service scaling, territory-level isolation |
| **Resilience** | No single point of failure, service redundancy |
| **Security** | Zero-trust architecture, mTLS, E2E encryption |
| **Sovereignty** | Territory-specific data residency and control |
| **Performance** | Rust microservices, optimized queries, caching |
| **Observability** | Comprehensive logging, metrics, and tracing |

---

## 2. System Architecture

### 2.1 High-Level System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        CLIENT TIER                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                │
│  │   Web App    │  │ Mobile App   │  │ Desktop App  │                │
│  │ (React/Vite) │  │   (Tauri)    │  │   (Tauri)    │                │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                │
│         │                 │                 │                          │
│         └─────────────────┴─────────────────┘                          │
│                           │                                            │
│                           │ HTTPS / WSS                                │
└───────────────────────────┼────────────────────────────────────────────┘
                            │
┌───────────────────────────┼────────────────────────────────────────────┐
│                        EDGE TIER                                        │
├───────────────────────────┼────────────────────────────────────────────┤
│                           ▼                                            │
│                   ┌───────────────┐                                    │
│                   │  Traefik      │  Reverse Proxy / Load Balancer     │
│                   │  (API Gateway)│  • TLS Termination                 │
│                   └───────┬───────┘  • Rate Limiting                   │
│                           │          • Request Routing                 │
└───────────────────────────┼────────────────────────────────────────────┘
                            │
┌───────────────────────────┼────────────────────────────────────────────┐
│                     SERVICE TIER                                        │
├───────────────────────────┼────────────────────────────────────────────┤
│                           │                                            │
│    ┌──────────────────────┼──────────────────────┐                    │
│    │                      ▼                      │                    │
│    │  ┌───────────┐  ┌────────────┐  ┌────────────┐                  │
│    │  │   Auth    │  │   User     │  │  Course    │                  │
│    │  │  Service  │  │  Service   │  │  Service   │                  │
│    │  └─────┬─────┘  └─────┬──────┘  └─────┬──────┘                  │
│    │        │              │               │                          │
│    │  ┌─────┴───────┬──────┴─────┬─────────┴─────┐                   │
│    │  │             │            │               │                   │
│    │  ▼             ▼            ▼               ▼                   │
│    │ ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐                 │
│    │ │ Badge  │  │ Forum  │  │ Matrix │  │ Trans  │                 │
│    │ │Service │  │Service │  │Gateway │  │Service │                 │
│    │ └────┬───┘  └────┬───┘  └────┬───┘  └────┬───┘                 │
│    │      │           │           │           │                      │
│    │      └───────────┴───────────┴───────────┘                      │
│    │                      │                                           │
│    │              ┌───────┴────────┐                                 │
│    │              │  NATS Message  │  Event Bus                      │
│    │              │      Bus       │  • Pub/Sub                      │
│    │              └───────┬────────┘  • Request/Reply               │
│    └──────────────────────┼───────────────────────────┘              │
│                           │                                          │
└───────────────────────────┼──────────────────────────────────────────┘
                            │
┌───────────────────────────┼──────────────────────────────────────────┐
│                      DATA TIER                                         │
├───────────────────────────┼──────────────────────────────────────────┤
│                           ▼                                            │
│    ┌────────────────────────────────────────────┐                    │
│    │         PostgreSQL + TimescaleDB           │                    │
│    │  ┌──────────┬──────────┬─────────────┐    │                    │
│    │  │  global  │territory │ territory   │... │                    │
│    │  │  schema  │   _dk    │    _ca      │    │                    │
│    │  └──────────┴──────────┴─────────────┘    │                    │
│    └────────────────────────────────────────────┘                    │
│                                                                       │
│    ┌──────────────┐       ┌────────────────┐                        │
│    │     IPFS     │       │ Matrix Synapse │                        │
│    │  (Files)     │       │  (Per Territory)│                       │
│    └──────────────┘       └────────────────┘                        │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

### 2.2 Request Flow - User Authentication

```
┌─────────┐                ┌─────────┐              ┌──────────┐
│ Client  │                │ Traefik │              │   Auth   │
│ Browser │                │ Gateway │              │  Service │
└────┬────┘                └────┬────┘              └────┬─────┘
     │                          │                        │
     │  1. GET /api/auth/login  │                        │
     ├─────────────────────────>│                        │
     │                          │                        │
     │                          │  2. Forward request    │
     │                          ├───────────────────────>│
     │                          │                        │
     │                          │                        │  3. Redirect to OIDC
     │                          │  4. 302 Redirect       │     Provider
     │<─────────────────────────┼────────────────────────┤
     │                          │                        │
     │  5. Authenticate with    │                        │
     │     OIDC Provider        │                        │
     ├──────────────────────────┼────────────────────────┼──────>
     │                          │                        │
     │  6. Callback with code   │                        │
     ├─────────────────────────>│                        │
     │                          │  7. Exchange code      │
     │                          ├───────────────────────>│
     │                          │                        │
     │                          │                        │  8. Validate token
     │                          │                        │     Create session
     │                          │                        │     Generate JWT
     │                          │  9. JWT token          │
     │  10. Set cookie + token  │<───────────────────────┤
     │<─────────────────────────┤                        │
     │                          │                        │
```

### 2.3 Request Flow - Authenticated API Call

```
┌────────┐    ┌─────────┐    ┌──────────┐    ┌────────┐    ┌──────────┐
│Client  │    │ Traefik │    │   Auth   │    │ User   │    │PostgreSQL│
│        │    │         │    │ Middleware│   │Service │    │          │
└───┬────┘    └────┬────┘    └────┬─────┘    └───┬────┘    └────┬─────┘
    │              │              │              │              │
    │ GET /api/users/profile      │              │              │
    │ Authorization: Bearer <JWT> │              │              │
    ├─────────────>│              │              │              │
    │              │              │              │              │
    │              │ Validate JWT │              │              │
    │              ├─────────────>│              │              │
    │              │              │              │              │
    │              │ JWT Valid    │              │              │
    │              │ Extract user_id, territory  │              │
    │              │<─────────────┤              │              │
    │              │              │              │              │
    │              │ Forward with user context   │              │
    │              ├────────────────────────────>│              │
    │              │              │              │              │
    │              │              │              │ SELECT FROM  │
    │              │              │              │ territory_dk.│
    │              │              │              │ user_profiles│
    │              │              │              ├─────────────>│
    │              │              │              │              │
    │              │              │              │  User data   │
    │              │              │              │<─────────────┤
    │              │              │              │              │
    │              │     200 OK + Profile data   │              │
    │<─────────────┴──────────────┴──────────────┤              │
    │              │              │              │              │
```

---

## 3. Microservices Architecture

### 3.1 Service Topology

```
┌─────────────────────────────────────────────────────────────────────┐
│                      MICROSERVICES MESH                             │
│                                                                     │
│   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐ │
│   │    Auth     │────────>│    User     │<────────│   Badge     │ │
│   │   Service   │         │   Service   │         │   Service   │ │
│   │             │         │             │         │             │ │
│   │ - Login     │         │ - Profiles  │         │ - Awards    │ │
│   │ - Register  │         │ - Privacy   │         │ - Validate  │ │
│   │ - JWT       │         │ - Settings  │         │ - Renew     │ │
│   └──────┬──────┘         └──────┬──────┘         └──────┬──────┘ │
│          │                       │                       │        │
│          │                       │                       │        │
│          └───────────────────────┼───────────────────────┘        │
│                                  │                                │
│                         ┌────────┴────────┐                       │
│                         │   NATS Bus      │                       │
│                         │  (Events)       │                       │
│                         └────────┬────────┘                       │
│                                  │                                │
│          ┌───────────────────────┼───────────────────────┐        │
│          │                       │                       │        │
│   ┌──────┴──────┐         ┌──────┴──────┐         ┌──────┴──────┐ │
│   │   Course    │         │   Forum     │         │   Matrix    │ │
│   │   Service   │         │   Service   │         │   Gateway   │ │
│   │             │         │             │         │             │ │
│   │ - LMS       │         │ - Topics    │         │ - Rooms     │ │
│   │ - Progress  │         │ - Comments  │         │ - Messages  │ │
│   │ - Badges    │         │ - Moderate  │         │ - Federation│ │
│   └──────┬──────┘         └──────┬──────┘         └──────┬──────┘ │
│          │                       │                       │        │
│          │                       │                       │        │
│   ┌──────┴──────┐         ┌──────┴──────┐         ┌──────┴──────┐ │
│   │Translation  │         │ Territory   │         │   Notify    │ │
│   │   Service   │         │   Service   │         │   Service   │ │
│   │             │         │             │         │             │ │
│   │ - Translate │         │ - Registry  │         │ - Email     │ │
│   │ - Languages │         │ - Settings  │         │ - WebSocket │ │
│   │ - Cache     │         │ - Routing   │         │ - Push      │ │
│   └─────────────┘         └─────────────┘         └─────────────┘ │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Core Services Breakdown

#### Authentication Service

```
┌───────────────────────────────────┐
│     Authentication Service         │
├───────────────────────────────────┤
│ Port: 8001                        │
│ Database: global schema           │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • User login/logout              │
│  • OIDC integration               │
│  • JWT token issuance             │
│  • Session management             │
│  • Token validation               │
│  • Refresh tokens                 │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (global.users)      │
│  → NATS (auth.* events)           │
│  → External OIDC providers        │
├───────────────────────────────────┤
│ Events Published:                 │
│  • user.logged_in                 │
│  • user.logged_out                │
│  • token.refreshed                │
│  • session.expired                │
└───────────────────────────────────┘
```

#### User Service

```
┌───────────────────────────────────┐
│         User Service              │
├───────────────────────────────────┤
│ Port: 8002                        │
│ Database: territory schemas       │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • User profiles                  │
│  • Privacy settings               │
│  • Social links                   │
│  • Language preferences           │
│  • Profile visibility             │
│  • Data export/deletion           │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (territory_*)       │
│  → Auth Service (validation)      │
│  → NATS (user.* events)           │
│  → IPFS (profile images)          │
├───────────────────────────────────┤
│ Events Published:                 │
│  • user.profile_updated           │
│  • user.deleted                   │
│  • user.privacy_changed           │
└───────────────────────────────────┘
```

#### Badge Service

```
┌───────────────────────────────────┐
│         Badge Service             │
├───────────────────────────────────┤
│ Port: 8003                        │
│ Database: global + territory      │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Award badges                   │
│  • Validate permissions           │
│  • Track expiration               │
│  • Renewal notifications          │
│  • Prerequisite checking          │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (badges)            │
│  → User Service                   │
│  → Course Service                 │
│  → NATS (badge.* events)          │
├───────────────────────────────────┤
│ Events Subscribed:                │
│  • course.completed               │
│  • badge.renewal_needed           │
├───────────────────────────────────┤
│ Events Published:                 │
│  • badge.awarded                  │
│  • badge.expired                  │
│  • badge.renewed                  │
└───────────────────────────────────┘
```

#### Course Service (LMS)

```
┌───────────────────────────────────┐
│         Course Service            │
├───────────────────────────────────┤
│ Port: 8004                        │
│ Database: territory schemas       │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Course management              │
│  • Enrollment                     │
│  • Progress tracking              │
│  • Completion verification        │
│  • Content delivery               │
│  • Quiz/assessment                │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (courses)           │
│  → Badge Service (prerequisites)  │
│  → IPFS (course materials)        │
│  → NATS (course.* events)         │
├───────────────────────────────────┤
│ Events Published:                 │
│  • course.enrolled                │
│  • course.completed               │
│  • course.progress_updated        │
│  • course.version_changed         │
└───────────────────────────────────┘
```

#### Forum Service

```
┌───────────────────────────────────┐
│         Forum Service             │
├───────────────────────────────────┤
│ Port: 8005                        │
│ Database: territory schemas       │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Forum structure                │
│  • Topic management               │
│  • Comments/replies               │
│  • Moderation                     │
│  • Voting                         │
│  • Badge-gated access             │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (forums)            │
│  → Badge Service (permissions)    │
│  → Matrix Gateway (integration)   │
│  → NATS (forum.* events)          │
├───────────────────────────────────┤
│ Events Published:                 │
│  • topic.created                  │
│  • comment.posted                 │
│  • moderation.action_taken        │
└───────────────────────────────────┘
```

#### Matrix Gateway Service

```
┌───────────────────────────────────┐
│       Matrix Gateway              │
├───────────────────────────────────┤
│ Port: 8006                        │
│ Protocol: Matrix Protocol         │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Matrix client API              │
│  • Room management                │
│  • Message routing                │
│  • Federation                     │
│  • E2E encryption bridge          │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → Matrix Synapse server          │
│  → Forum Service                  │
│  → User Service                   │
│  → NATS (matrix.* events)         │
├───────────────────────────────────┤
│ Events Published:                 │
│  • message.received               │
│  • room.created                   │
│  • federation.sync                │
└───────────────────────────────────┘
```

#### Translation Service

```
┌───────────────────────────────────┐
│      Translation Service          │
├───────────────────────────────────┤
│ Port: 8007                        │
│ Database: public schema           │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Real-time translation          │
│  • Translation memory             │
│  • Language detection             │
│  • Cache management               │
│  • Quality scoring                │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (translations)      │
│  → External translation APIs      │
│  → Redis (caching)                │
│  → NATS (translate.* events)      │
├───────────────────────────────────┤
│ Events Published:                 │
│  • translation.cached             │
│  • translation.requested          │
└───────────────────────────────────┘
```

#### Territory Service

```
┌───────────────────────────────────┐
│       Territory Service           │
├───────────────────────────────────┤
│ Port: 8008                        │
│ Database: global schema           │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Territory registry             │
│  • Database routing               │
│  • Territory settings             │
│  • Manager assignments            │
│  • Content visibility             │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (territories)       │
│  → All services (routing)         │
│  → NATS (territory.* events)      │
├───────────────────────────────────┤
│ Events Published:                 │
│  • territory.created              │
│  • territory.settings_changed     │
│  • territory.migrated             │
└───────────────────────────────────┘
```

#### Notification Service

```
┌───────────────────────────────────┐
│      Notification Service         │
├───────────────────────────────────┤
│ Port: 8009                        │
│ Database: territory schemas       │
├───────────────────────────────────┤
│ Responsibilities:                 │
│  • Email notifications            │
│  • Push notifications             │
│  • WebSocket real-time            │
│  • Notification preferences       │
│  • Delivery tracking              │
├───────────────────────────────────┤
│ Dependencies:                     │
│  → PostgreSQL (notifications)     │
│  → SMTP server                    │
│  → Push service (FCM/APNS)        │
│  → NATS (all event subscriptions) │
├───────────────────────────────────┤
│ Events Subscribed:                │
│  • *.* (all events for filtering) │
└───────────────────────────────────┘
```

### 3.3 Service Communication Patterns

#### Synchronous Communication (HTTP)

```
┌─────────┐                              ┌─────────┐
│ Client  │                              │ Service │
└────┬────┘                              └────┬────┘
     │                                        │
     │  POST /api/service/endpoint            │
     ├───────────────────────────────────────>│
     │                                        │
     │                                        │  Process request
     │                                        │  Query database
     │                                        │  Business logic
     │                                        │
     │  200 OK + Response                     │
     │<───────────────────────────────────────┤
     │                                        │
```

#### Asynchronous Communication (NATS)

```
Publisher                 NATS Bus               Subscriber(s)
┌────────┐               ┌────────┐              ┌────────┐
│Course  │               │        │              │ Badge  │
│Service │               │  NATS  │              │Service │
└───┬────┘               └───┬────┘              └───┬────┘
    │                        │                       │
    │ Publish:               │                       │
    │ "course.completed"     │                       │
    ├───────────────────────>│                       │
    │                        │                       │
    │                        │  Subscribe to:        │
    │                        │  "course.completed"   │
    │                        │<──────────────────────┤
    │                        │                       │
    │                        │  Deliver event        │
    │                        ├──────────────────────>│
    │                        │                       │
    │                        │                       │  Award badge
    │                        │                       │  Update permissions
    │                        │                       │
    │                        │  Publish:             │
    │                        │  "badge.awarded"      │
    │                        │<──────────────────────┤
    │                        │                       │
```

#### Request/Reply Pattern (RPC over NATS)

```
Requester                 NATS Bus               Responder
┌────────┐               ┌────────┐              ┌────────┐
│ Forum  │               │        │              │ Badge  │
│Service │               │  NATS  │              │Service │
└───┬────┘               └───┬────┘              └───┬────┘
    │                        │                       │
    │ Request:               │                       │
    │ "badge.check_access"   │                       │
    │ (user_id, badge_req)   │                       │
    ├───────────────────────>│                       │
    │                        │                       │
    │                        │  Forward request      │
    │                        ├──────────────────────>│
    │                        │                       │
    │                        │                       │  Check DB
    │                        │                       │  Validate
    │                        │                       │
    │                        │  Reply:               │
    │                        │  {has_access: true}   │
    │                        │<──────────────────────┤
    │                        │                       │
    │  Response              │                       │
    │<───────────────────────┤                       │
    │                        │                       │
```

### 3.4 API Gateway Routing

```
                         Traefik Routing Rules
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
         ▼                       ▼                       ▼
   /api/auth/*            /api/users/*            /api/courses/*
         │                       │                       │
         ▼                       ▼                       ▼
   ┌─────────┐            ┌─────────┐            ┌─────────┐
   │  Auth   │            │  User   │            │ Course  │
   │ Service │            │ Service │            │ Service │
   │ :8001   │            │ :8002   │            │ :8004   │
   └─────────┘            └─────────┘            └─────────┘

Traefik Configuration:
http:
  routers:
    auth-router:
      rule: "PathPrefix(`/api/auth`)"
      service: auth-service
      middlewares:
        - rate-limit
        - cors
    
    user-router:
      rule: "PathPrefix(`/api/users`)"
      service: user-service
      middlewares:
        - auth-middleware
        - rate-limit
        - cors
    
    course-router:
      rule: "PathPrefix(`/api/courses`)"
      service: course-service
      middlewares:
        - auth-middleware
        - badge-check
        - rate-limit
        - cors

  services:
    auth-service:
      loadBalancer:
        servers:
          - url: "http://auth-service:8001"
    
    user-service:
      loadBalancer:
        servers:
          - url: "http://user-service:8002"
    
    course-service:
      loadBalancer:
        servers:
          - url: "http://course-service:8004"
```

---

## 4. Multi-Territory Infrastructure

### 4.1 Territory Isolation Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    PostgreSQL Instance                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │    global    │  │    public    │  │ territory_dk │            │
│  │    schema    │  │    schema    │  │   (Denmark)  │            │
│  ├──────────────┤  ├──────────────┤  ├──────────────┤            │
│  │              │  │              │  │              │            │
│  │ • users      │  │ • trans      │  │ • profiles   │            │
│  │ • territories│  │ • categories │  │ • badges     │            │
│  │ • roles      │  │ • badge_defs │  │ • courses    │            │
│  │ • audit_log  │  │ • tool_types │  │ • forums     │            │
│  │              │  │              │  │ • communities│            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐            │
│  │ territory_ca │  │ territory_ke │  │ territory_sami│           │
│  │  (Canada)    │  │  (Kenya)     │  │ (Sámi Nation)│           │
│  ├──────────────┤  ├──────────────┤  ├──────────────┤            │
│  │              │  │              │  │              │            │
│  │ • profiles   │  │ • profiles   │  │ • profiles   │            │
│  │ • badges     │  │ • badges     │  │ • badges     │            │
│  │ • courses    │  │ • courses    │  │ • courses    │            │
│  │ • forums     │  │ • forums     │  │ • forums     │            │
│  │ • communities│  │ • communities│  │ • communities│            │
│  └──────────────┘  └──────────────┘  └──────────────┘            │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.2 Evolution Path: Single Server → Multi-Server

#### Phase 1: Initial Deployment (3-5 Territories)

```
┌──────────────────────────────────────────────────┐
│         Single PostgreSQL Server                 │
│         Location: Cloud Region (e.g., EU)        │
├──────────────────────────────────────────────────┤
│                                                  │
│  All territory schemas on one server:            │
│                                                  │
│  • territory_denmark                             │
│  • territory_norway                              │
│  • territory_sweden                              │
│  • territory_finland                             │
│  • territory_iceland                             │
│                                                  │
│  Benefits:                                       │
│  ✓ Simple infrastructure                         │
│  ✓ Low operational cost                          │
│  ✓ Easy backups                                  │
│  ✓ Straightforward migrations                    │
│                                                  │
└──────────────────────────────────────────────────┘
```

#### Phase 2: Regional Servers (10-20 Territories)

```
┌──────────────────────┐     ┌──────────────────────┐
│   EU Server          │     │  Americas Server     │
│  (Frankfurt)         │     │  (US East)           │
├──────────────────────┤     ├──────────────────────┤
│                      │     │                      │
│  Territories:        │     │  Territories:        │
│  • territory_dk      │     │  • territory_us      │
│  • territory_no      │     │  • territory_ca      │
│  • territory_se      │     │  • territory_mx      │
│  • territory_fi      │     │  • territory_br      │
│  • territory_de      │     │                      │
│  • territory_uk      │     │                      │
│                      │     │                      │
└──────────────────────┘     └──────────────────────┘

┌──────────────────────┐     ┌──────────────────────┐
│  Asia-Pacific Server │     │   Africa Server      │
│  (Singapore)         │     │  (South Africa)      │
├──────────────────────┤     ├──────────────────────┤
│                      │     │                      │
│  Territories:        │     │  Territories:        │
│  • territory_au      │     │  • territory_za      │
│  • territory_nz      │     │  • territory_ke      │
│  • territory_jp      │     │  • territory_ng      │
│  • territory_sg      │     │  • territory_gh      │
│                      │     │                      │
└──────────────────────┘     └──────────────────────┘

Benefits:
  ✓ Reduced latency for users
  ✓ Data residency compliance
  ✓ Regional autonomy
  ✓ Independent scaling
```

#### Phase 3: Territory-Specific Servers (50+ Territories)

```
Large Territories (Dedicated Servers):

┌──────────────────────┐     ┌──────────────────────┐
│  Denmark Server      │     │   Canada Server      │
│  (Copenhagen)        │     │  (Toronto)           │
├──────────────────────┤     ├──────────────────────┤
│  • territory_dk only │     │  • territory_ca only │
│                      │     │                      │
│  Full sovereignty    │     │  Full sovereignty    │
│  Dedicated resources │     │  Dedicated resources │
└──────────────────────┘     └──────────────────────┘

Small Territories (Shared Regional Servers):

┌──────────────────────────────────────────────────┐
│   Nordic Shared Server                           │
│   (Stockholm)                                    │
├──────────────────────────────────────────────────┤
│  Multiple small territories:                     │
│  • territory_faroe_islands                       │
│  • territory_greenland                           │
│  • territory_sami                                │
│  • territory_aland                               │
│                                                  │
│  Cost-effective for smaller populations          │
└──────────────────────────────────────────────────┘
```

### 4.3 Cross-Server Territory Management

```
┌─────────────────────────────────────────────────────────────────┐
│                     Territory Manager UI                        │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ Territory Switcher:  [ Denmark ▼ ]                     │   │
│  │                      │                                  │   │
│  │                      ├── Denmark (EU Server)            │   │
│  │                      ├── Canada (Americas Server)       │   │
│  │                      └── Kenya (Africa Server)          │   │
│  └────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      API Gateway (Traefik)                      │
│                                                                 │
│  Request: GET /api/users/profile                               │
│  Header: X-Territory-ID: denmark                               │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Territory Registry Service                   │
│                                                                 │
│  1. Lookup territory "denmark"                                 │
│  2. Find database server: "postgres://eu-server:5432"          │
│  3. Cache mapping (TTL: 5 min)                                 │
│  4. Return connection pool                                     │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│               Connection Pool Manager                           │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   EU Pool    │  │Americas Pool │  │ Africa Pool  │         │
│  │              │  │              │  │              │         │
│  │ Max: 20      │  │ Max: 20      │  │ Max: 20      │         │
│  │ Active: 5    │  │ Active: 3    │  │ Active: 2    │         │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘         │
│         │                 │                 │                  │
└─────────┼─────────────────┼─────────────────┼──────────────────┘
          │                 │                 │
          ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  EU Server   │  │Americas Server│ │Africa Server │
│ (Frankfurt)  │  │  (US East)   │  │(South Africa)│
└──────────────┘  └──────────────┘  └──────────────┘
```

### 4.4 Territory Migration Process

```
Migration: territory_denmark from Shared → Dedicated Server

Step 1: Export Territory Data
┌──────────────────────────────────────────────────┐
│  Source: EU Shared Server                        │
│                                                  │
│  $ pg_dump --host=eu-shared.db \                │
│    --schema=territory_denmark \                  │
│    --format=custom \                             │
│    --file=denmark_backup.dump \                  │
│    platform_db                                   │
│                                                  │
│  Output: denmark_backup.dump (compressed)        │
└──────────────────────────────────────────────────┘
         │
         ▼
Step 2: Create New Server & Restore
┌──────────────────────────────────────────────────┐
│  Target: Denmark Dedicated Server                │
│                                                  │
│  1. Provision new PostgreSQL instance            │
│  2. Create database: platform_db                 │
│  3. Restore backup:                              │
│                                                  │
│  $ pg_restore --host=dk-dedicated.db \           │
│    --dbname=platform_db \                        │
│    --schema=territory_denmark \                  │
│    denmark_backup.dump                           │
│                                                  │
└──────────────────────────────────────────────────┘
         │
         ▼
Step 3: Update Territory Registry
┌──────────────────────────────────────────────────┐
│  Territory Registry Service                      │
│                                                  │
│  UPDATE territories                              │
│  SET database_server =                           │
│    'postgres://dk-dedicated.db:5432/platform'    │
│  WHERE territory_id = 'denmark';                 │
│                                                  │
│  ✓ Registry updated                              │
│  ✓ Cache invalidated                             │
└──────────────────────────────────────────────────┘
         │
         ▼
Step 4: Verify & Switch Traffic
┌──────────────────────────────────────────────────┐
│  Traffic Routing                                 │
│                                                  │
│  1. Health check new server                      │
│  2. Test queries                                 │
│  3. Gradual traffic shift:                       │
│     • 10% → Denmark Dedicated                    │
│     • 50% → Denmark Dedicated                    │
│     • 100% → Denmark Dedicated                   │
│  4. Monitor for issues                           │
│                                                  │
│  ✓ Migration complete                            │
│  ✓ Old schema can be archived                    │
└──────────────────────────────────────────────────┘
```

### 4.5 Data Residency & Compliance

```
┌─────────────────────────────────────────────────────────────────┐
│              Territory Data Residency Requirements              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  EU Territories (GDPR)                                         │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  • Data stored in EU region                             │  │
│  │  • No cross-border transfers without consent            │  │
│  │  • Right to deletion enforcement                        │  │
│  │  • Data portability support                             │  │
│  │  • Audit logs for all access                            │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Canadian Territories (PIPEDA)                                 │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  • Data stored in Canada or with adequate protection    │  │
│  │  • Consent tracking                                     │  │
│  │  • Breach notification procedures                       │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Indigenous Territories (Sovereignty)                          │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  • Data sovereignty respected                           │  │
│  │  • Community control over data sharing                  │  │
│  │  • Cultural considerations in data handling             │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

Implementation:
┌──────────────────────────────────────────────────┐
│  Territory Settings Table                        │
├──────────────────────────────────────────────────┤
│                                                  │
│  territory_settings {                            │
│    territory_id: UUID,                           │
│    data_residency_region: String,                │
│    compliance_frameworks: [GDPR, PIPEDA, ...],   │
│    allow_cross_border: Boolean,                  │
│    encryption_required: Boolean,                 │
│    audit_retention_days: Integer,                │
│    backup_location: String,                      │
│  }                                               │
│                                                  │
└──────────────────────────────────────────────────┘
```

### 4.6 Territory Federation (Matrix Protocol)

```
┌─────────────────────────────────────────────────────────────────┐
│                   Cross-Territory Communication                 │
│                    (Matrix Federation)                          │
└─────────────────────────────────────────────────────────────────┘

Denmark Territory                    Canada Territory
┌──────────────────┐                ┌──────────────────┐
│ Matrix Synapse   │                │ Matrix Synapse   │
│ matrix.dk.unity  │                │ matrix.ca.unity  │
│                  │                │                  │
│ Users:           │                │ Users:           │
│ @user:dk.unity   │                │ @user:ca.unity   │
└────────┬─────────┘                └────────┬─────────┘
         │                                   │
         │      Federation Protocol          │
         │   (Server-to-Server API)          │
         └───────────────┬───────────────────┘
                         │
              Cross-Territory Room:
         !global-forum:dk.unity

┌─────────────────────────────────────────────────────┐
│  Room: #global-climate-action                       │
│  Server: matrix.dk.unity                            │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Participants from multiple territories:            │
│  • @alice:dk.unity (Denmark)                        │
│  • @bob:ca.unity (Canada)                           │
│  • @carol:ke.unity (Kenya)                          │
│  • @david:sami.unity (Sámi Nation)                  │
│                                                     │
│  Messages are federated across servers              │
│  Each territory retains its own copy                │
│  End-to-end encryption available                    │
│                                                     │
└─────────────────────────────────────────────────────┘

Federation Flow:
@alice:dk.unity sends message in room !xyz:dk.unity

  1. Client → Denmark Matrix Server
     POST /_matrix/client/r0/rooms/!xyz:dk.unity/send

  2. Denmark Server processes message
     - Stores in local database
     - Identifies remote participants

  3. Denmark Server → Canada Server (federation)
     PUT /_matrix/federation/v1/send/<txn_id>

  4. Canada Server receives and stores
     - Validates signature
     - Stores for local users
     - Delivers to @bob:ca.unity

  5. Kenya Server receives and stores
     - Same process for @carol:ke.unity

Result: Message available on all territory servers
```

---

## 5. Network & Security

### 5.1 Zero-Trust Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Security Layers                              │
└─────────────────────────────────────────────────────────────────┘

Layer 1: Edge Security (Traefik)
┌──────────────────────────────────────────────────────┐
│  • TLS 1.3 termination                               │
│  • Rate limiting (100 req/min per IP)                │
│  • DDoS protection                                   │
│  • WAF (Web Application Firewall)                    │
│  • CORS policy enforcement                           │
└──────────────────────────────────────────────────────┘
                        │
                        ▼
Layer 2: Authentication
┌──────────────────────────────────────────────────────┐
│  • JWT validation                                    │
│  • Token expiration check (15 min)                   │
│  • Refresh token rotation                            │
│  • Session tracking                                  │
└──────────────────────────────────────────────────────┘
                        │
                        ▼
Layer 3: Service Mesh (mTLS)
┌──────────────────────────────────────────────────────┐
│  • Mutual TLS between all services                   │
│  • Certificate rotation (7 days)                     │
│  • Service identity verification                     │
│  • Encrypted service-to-service communication        │
└──────────────────────────────────────────────────────┘
                        │
                        ▼
Layer 4: Authorization
┌──────────────────────────────────────────────────────┐
│  • Badge-based permissions                           │
│  • Territory access control                          │
│  • Role validation                                   │
│  • Resource-level permissions                        │
└──────────────────────────────────────────────────────┘
                        │
                        ▼
Layer 5: Data Layer Security
┌──────────────────────────────────────────────────────┐
│  • PostgreSQL Row-Level Security (RLS)               │
│  • Encrypted connections (SSL)                       │
│  • Schema isolation                                  │
│  • Audit logging                                     │
└──────────────────────────────────────────────────────┘
```

### 5.2 Service Mesh with mTLS (Linkerd)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Linkerd Service Mesh                         │
└─────────────────────────────────────────────────────────────────┘

Service A                  Linkerd Proxy              Service B
┌──────────┐              ┌────────────┐             ┌──────────┐
│          │              │            │             │          │
│  Auth    │──────────┐   │  Linkerd   │   ┌─────────│  User    │
│  Service │          │   │  Data      │   │         │  Service │
│          │          └──>│  Plane     │<──┘         │          │
│  :8001   │              │            │             │  :8002   │
└──────────┘              └────────────┘             └──────────┘
     │                          │                          │
     │ 1. Initiate request      │                          │
     │    (plaintext internally)│                          │
     └─────────────────────────>│                          │
                                │                          │
                                │ 2. mTLS handshake        │
                                │    - Verify cert         │
                                │    - Establish session   │
                                ├─────────────────────────>│
                                │                          │
                                │ 3. Encrypted channel     │
                                │<─────────────────────────┤
                                │                          │
                                │ 4. Forward request       │
                                ├─────────────────────────>│
                                │                          │
                                │ 5. Response              │
                                │<─────────────────────────┤
                                │                          │
     │ 6. Deliver response      │                          │
     │<─────────────────────────┤                          │

Features:
┌──────────────────────────────────────────────────────┐
│  • Automatic mTLS between services                   │
│  • Certificate lifecycle management                  │
│  • Service discovery                                 │
│  • Load balancing                                    │
│  • Circuit breaking                                  │
│  • Retry budgets                                     │
│  • Request tracing                                   │
│  • Metrics collection                                │
└──────────────────────────────────────────────────────┘
```

### 5.3 TLS Termination & Certificate Management

```
┌─────────────────────────────────────────────────────────────────┐
│                 TLS Certificate Flow                            │
└─────────────────────────────────────────────────────────────────┘

Client                    Traefik                  Let's Encrypt
┌────────┐               ┌─────────┐               ┌────────────┐
│Browser │               │         │               │  ACME CA   │
└───┬────┘               └────┬────┘               └─────┬──────┘
    │                         │                          │
    │ https://unityplan.org   │                          │
    ├────────────────────────>│                          │
    │                         │                          │
    │                         │ No valid cert, request   │
    │                         ├─────────────────────────>│
    │                         │                          │
    │                         │ ACME Challenge           │
    │                         │<─────────────────────────┤
    │                         │                          │
    │ /.well-known/acme/...   │                          │
    ├────────────────────────>│                          │
    │                         │                          │
    │ Challenge response      │                          │
    │<────────────────────────┤                          │
    │                         │                          │
    │                         │ Validation success       │
    │                         │<─────────────────────────┤
    │                         │                          │
    │                         │ Issue certificate        │
    │                         │<─────────────────────────┤
    │                         │                          │
    │ TLS Handshake           │                          │
    │ (with new cert)         │                          │
    │<────────────────────────┤                          │
    │                         │                          │
    │ Encrypted HTTPS         │                          │
    │<───────────────────────>│                          │
    │                         │                          │

Certificate Storage:
┌──────────────────────────────────────────────────────┐
│  /etc/traefik/certs/                                 │
│    ├── unityplan.org.crt                             │
│    ├── unityplan.org.key                             │
│    ├── matrix.unityplan.org.crt                      │
│    ├── matrix.unityplan.org.key                      │
│    └── api.unityplan.org.crt                         │
│                                                      │
│  Auto-renewal: 30 days before expiration             │
│  Backup: Daily to secure storage                     │
└──────────────────────────────────────────────────────┘
```

### 5.4 Network Segmentation

```
┌─────────────────────────────────────────────────────────────────┐
│                     Docker Networks                             │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  public-network (exposed to internet)                           │
│                                                                 │
│  ┌──────────┐                                                   │
│  │ Traefik  │  Port 80, 443                                     │
│  └──────────┘                                                   │
└─────────────────────────────────────────────────────────────────┘
         │
         │ (bridge)
         ▼
┌─────────────────────────────────────────────────────────────────┐
│  service-network (internal services)                            │
│                                                                 │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐           │
│  │  Auth   │  │  User   │  │ Course  │  │  Badge  │           │
│  │ Service │  │ Service │  │ Service │  │ Service │           │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘           │
│                                                                 │
│  No direct internet access                                      │
└─────────────────────────────────────────────────────────────────┘
         │
         │ (bridge)
         ▼
┌─────────────────────────────────────────────────────────────────┐
│  data-network (databases only)                                  │
│                                                                 │
│  ┌──────────────┐  ┌──────────┐  ┌──────────┐                 │
│  │  PostgreSQL  │  │   NATS   │  │   IPFS   │                 │
│  └──────────────┘  └──────────┘  └──────────┘                 │
│                                                                 │
│  Isolated from services network                                 │
│  Only specific services have access                             │
└─────────────────────────────────────────────────────────────────┘

Docker Compose Network Configuration:
networks:
  public-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16
  
  service-network:
    driver: bridge
    internal: true
    ipam:
      config:
        - subnet: 172.21.0.0/16
  
  data-network:
    driver: bridge
    internal: true
    ipam:
      config:
        - subnet: 172.22.0.0/16
```

### 5.5 Security Best Practices Implementation

```
┌─────────────────────────────────────────────────────────────────┐
│                    Security Hardening                           │
└─────────────────────────────────────────────────────────────────┘

Container Security:
┌──────────────────────────────────────────────────────┐
│  • Run as non-root user (UID 1000)                   │
│  • Read-only root filesystem where possible          │
│  • No privileged containers                          │
│  • Resource limits (CPU, memory)                     │
│  • Security scanning (Trivy, Snyk)                   │
│  • Minimal base images (Alpine, Distroless)          │
└──────────────────────────────────────────────────────┘

Secret Management:
┌──────────────────────────────────────────────────────┐
│  • Environment variables for non-sensitive config    │
│  • Docker secrets for sensitive data                 │
│  • Vault for production secrets                      │
│  • Never commit secrets to Git                       │
│  • Rotate secrets regularly                          │
└──────────────────────────────────────────────────────┘

Database Security:
┌──────────────────────────────────────────────────────┐
│  • Row-Level Security (RLS) policies                 │
│  • Prepared statements (SQL injection prevention)    │
│  • Connection pooling with max limits                │
│  • SSL-only connections                              │
│  • Principle of least privilege                      │
│  • Regular security patches                          │
└──────────────────────────────────────────────────────┘

API Security:
┌──────────────────────────────────────────────────────┐
│  • Input validation on all endpoints                 │
│  • Output encoding                                   │
│  • Rate limiting per user/IP                         │
│  • CSRF protection                                   │
│  • XSS prevention                                    │
│  • Content Security Policy (CSP)                     │
└──────────────────────────────────────────────────────┘
```

---

## 6. Data Layer

### 6.1 PostgreSQL Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│              PostgreSQL + TimescaleDB Instance                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Connection Pools (per territory):                             │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Territory: Denmark                                      │  │
│  │  Pool Size: 20 connections                               │  │
│  │  Idle Timeout: 10 minutes                                │  │
│  │  Max Lifetime: 30 minutes                                │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  Schemas:                                                       │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  global (shared):                                        │  │
│  │    - users (authentication)                              │  │
│  │    - territories (registry)                              │  │
│  │    - audit_logs (immutable)                              │  │
│  │                                                          │  │
│  │  public (reference data):                                │  │
│  │    - translations                                        │  │
│  │    - badge_definitions                                   │  │
│  │    - tool_types                                          │  │
│  │                                                          │  │
│  │  territory_{id} (per territory):                         │  │
│  │    - user_profiles                                       │  │
│  │    - courses                                             │  │
│  │    - forums                                              │  │
│  │    - badges_earned                                       │  │
│  │    - communities                                         │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  TimescaleDB Hypertables:                                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  • user_activity (partitioned by time)                   │  │
│  │  • system_metrics (partitioned by time)                  │  │
│  │  • audit_events (partitioned by time)                    │  │
│  │                                                          │  │
│  │  Retention: 90 days detailed, 2 years aggregated         │  │
│  │  Compression: Automatic after 7 days                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Database Schema Overview

> **📚 Detailed Schemas:** See service-specific documentation:
>
> - [auth-service/DATABASE.md](../services/auth-service/DATABASE.md) - Global user identities
> - [user-service/DATABASE.md](../services/user-service/DATABASE.md) - Territory user profiles
> - [territory-service/DATABASE.md](../services/territory-service/DATABASE.md) - Territory management
> - [badge-service/DATABASE.md](../services/badge-service/DATABASE.md) - Badge definitions and awards
> - [course-service/DATABASE.md](../services/course-service/DATABASE.md) - Course content
> - [storage-service/DATABASE.md](../services/storage-service/DATABASE.md) - IPFS file uploads

**Global Schema (Authentication & Registry):**

- `global.user_identities` - Minimal user identities (username, territory, public key hash)
- `global.territories` - Territory registry (code, name, pod assignment)
- `global.audit_logs` - System-wide audit trail

**Territory Schema Template** (`territory_{code}`):

- `territory_{code}.users` - Personal user data and profiles
- `territory_{code}.user_badges` - Badge awards per user
- `territory_{code}.courses` - Course content specific to territory
- `territory_{code}.user_languages` - Language proficiency data
- `territory_{code}.invitation_tokens` - Invitation system

**TimescaleDB Hypertables (Time-Series Data):**

- `user_activity` - User activity logs (partitioned by time)
- `system_metrics` - System performance metrics
- `audit_events` - Event auditing with time-series optimization

**Data Sovereignty Model:**

- Global schema: Only coordination data (identities, territories, audit)
- Territory schemas: All personal data (profiles, badges, courses, languages)
- User chooses territory during signup
- All personal data stored in chosen territory pod

### 6.3 IPFS Integration

```
┌─────────────────────────────────────────────────────────────────┐
│                   IPFS Architecture                             │
└─────────────────────────────────────────────────────────────────┘

┌──────────────┐                           ┌──────────────┐
│   Frontend   │                           │  Backend     │
│              │                           │  Services    │
└──────┬───────┘                           └──────┬───────┘
       │                                          │
       │ 1. Upload file                           │
       ├─────────────────────────────────────────>│
       │                                          │
       │                                          │ 2. Upload to IPFS
       │                                          ├────────────┐
       │                                          │            ▼
       │                                          │     ┌──────────────┐
       │                                          │     │  IPFS Node   │
       │                                          │     │              │
       │                                          │     │  • Store     │
       │                                          │     │  • Pin       │
       │                                          │     │  • Return CID│
       │                                          │     └──────────────┘
       │                                          │            │
       │                                          │<───────────┘
       │                                          │ 3. CID received
       │                                          │
       │                                          │ 4. Store CID in DB
       │                                          ├────────────┐
       │                                          │            ▼
       │                                          │     ┌──────────────┐
       │                                          │     │  PostgreSQL  │
       │                                          │     │              │
       │                                          │     │ file_uploads │
       │                                          │     └──────────────┘
       │                                          │
       │ 5. Return CID to client                  │
       │<─────────────────────────────────────────┤
       │                                          │
       │ 6. Request file via CID                  │
       ├─────────────────────────────────────────>│
       │                                          │
       │                                          │ 7. Fetch from IPFS
       │                                          ├────────────┐
       │                                          │            ▼
       │                                          │     ┌──────────────┐
       │                                          │     │  IPFS Node   │
       │                                          │     │              │
       │                                          │     │  • Cat CID   │
       │                                          │     └──────────────┘
       │                                          │            │
       │                                          │<───────────┘
       │                                          │
       │ 8. Stream file content                   │
       │<─────────────────────────────────────────┤
       │                                          │

**File Storage Schema:**

> **📚 Detailed Schema:** See [storage-service/DATABASE.md](../services/storage-service/DATABASE.md)

**Table:** `file_uploads`

**Key Fields:**
- `id` - UUID PRIMARY KEY
- `cid` - TEXT UNIQUE - IPFS Content ID (hash of file content)
- `filename` - VARCHAR(255) - Original filename
- `mime_type` - VARCHAR(100) - File MIME type
- `size_bytes` - BIGINT - File size
- `uploaded_by` - UUID - User who uploaded file
- `uploaded_at` - TIMESTAMPTZ - Upload timestamp
- `pinned` - BOOLEAN - Whether file is pinned on IPFS node
- `description` - TEXT - Optional file description

**IPFS Integration:**
- Files stored on IPFS node (content-addressable)
- CID (Content ID) stored in database
- Automatic pinning ensures file availability
- Deduplication by content hash
```

### 6.4 Backup & Disaster Recovery

```
┌─────────────────────────────────────────────────────────────────┐
│                    Backup Strategy                              │
└─────────────────────────────────────────────────────────────────┘

Daily Backups (Automated):
┌──────────────────────────────────────────────────────┐
│  Cron: 2:00 AM UTC daily                             │
│                                                      │
│  For each territory schema:                          │
│    pg_dump --schema=territory_{id} \                │
│      --format=custom \                               │
│      --file=/backups/territory_{id}_$(date).dump    │
│                                                      │
│  Retention:                                          │
│    • Daily: 7 days                                   │
│    • Weekly: 4 weeks                                 │
│    • Monthly: 12 months                              │
│                                                      │
│  Storage:                                            │
│    • Local: /backups (encrypted volume)              │
│    • Remote: S3-compatible (versioned, encrypted)    │
│                                                      │
└──────────────────────────────────────────────────────┘

Point-in-Time Recovery (PITR):
┌──────────────────────────────────────────────────────┐
│  • WAL (Write-Ahead Log) archiving enabled           │
│  • Continuous archiving to remote storage            │
│  • Recovery target: any point in last 30 days        │
│                                                      │
│  Recovery Process:                                   │
│    1. Restore base backup                            │
│    2. Apply WAL files up to target time              │
│    3. Promote to primary                             │
│                                                      │
└──────────────────────────────────────────────────────┘

Replication:
┌──────────────────────────────────────────────────────┐
│  Primary ─────> Standby Replica                      │
│                                                      │
│  • Streaming replication (synchronous)               │
│  • Automatic failover with health checks             │
│  • Read replicas for reporting queries               │
│                                                      │
└──────────────────────────────────────────────────────┘
```

---

## 7. Matrix Protocol Integration

### 7.1 Matrix Server Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│           Matrix Homeserver per Territory                       │
└─────────────────────────────────────────────────────────────────┘

Territory: Denmark
┌──────────────────────────────────────────────────────┐
│  Matrix Synapse Server                               │
│  Domain: matrix.dk.unityplan.org                     │
│                                                      │
│  ┌────────────────────────────────────────────────┐ │
│  │  Client-Server API (:8008)                     │ │
│  │  • Registration                                │ │
│  │  • Login                                       │ │
│  │  • Rooms                                       │ │
│  │  • Messages                                    │ │
│  │  • E2E encryption                              │ │
│  └────────────────────────────────────────────────┘ │
│                                                      │
│  ┌────────────────────────────────────────────────┐ │
│  │  Server-Server API (:8448)                     │ │
│  │  • Federation                                  │ │
│  │  • Event exchange                              │ │
│  │  • Key distribution                            │ │
│  └────────────────────────────────────────────────┘ │
│                                                      │
│  Database: PostgreSQL (matrix_dk schema)             │
│  Media Store: IPFS integration                       │
└──────────────────────────────────────────────────────┘

Territory: Canada
┌──────────────────────────────────────────────────────┐
│  Matrix Synapse Server                               │
│  Domain: matrix.ca.unityplan.org                     │
│  (Same structure as Denmark)                         │
└──────────────────────────────────────────────────────┘

           Federation Connection
Denmark ←──────────────────────→ Canada
```

### 7.2 Matrix Integration with Forum Service

```
┌─────────────────────────────────────────────────────────────────┐
│         Forum Service ↔ Matrix Integration                      │
└─────────────────────────────────────────────────────────────────┘

Forum Topic Created
│
▼
┌────────────────────────────────────────────────────────┐
│  Forum Service                                         │
│                                                        │
│  1. User creates forum topic                           │
│     POST /api/forums/topics                            │
│     {                                                  │
│       title: "Climate Action Discussion",             │
│       category_id: "...",                              │
│       territory_id: "denmark"                          │
│     }                                                  │
│                                                        │
│  2. Store in PostgreSQL                                │
│     INSERT INTO territory_dk.forum_topics ...          │
│                                                        │
│  3. Create corresponding Matrix room                   │
│     POST /_matrix/client/r0/createRoom                 │
│     {                                                  │
│       name: "Climate Action Discussion",               │
│       topic: "Forum topic ID: abc-123",                │
│       visibility: "public",                            │
│       preset: "public_chat"                            │
│     }                                                  │
│                                                        │
│  4. Link forum topic to Matrix room                    │
│     UPDATE territory_dk.forum_topics                   │
│     SET matrix_room_id = "!xyz:dk.unityplan.org"       │
│     WHERE id = "abc-123"                               │
└────────────────────────────────────────────────────────┘
│
▼
┌────────────────────────────────────────────────────────┐
│  Matrix Server                                         │
│                                                        │
│  Room created: !xyz:dk.unityplan.org                   │
│  All messages sync bidirectionally                     │
└────────────────────────────────────────────────────────┘

Message Flow:
User posts comment → Forum Service → Matrix room
Matrix room message → Forum Service → PostgreSQL
```

### 7.3 End-to-End Encryption (E2EE)

```
┌─────────────────────────────────────────────────────────────────┐
│              Matrix E2EE for Private Messages                   │
└─────────────────────────────────────────────────────────────────┘

Alice (@alice:dk.unity)          Bob (@bob:ca.unity)
┌─────────────┐                 ┌─────────────┐
│             │                 │             │
│ 1. Generate │                 │ 1. Generate │
│    device   │                 │    device   │
│    keys     │                 │    keys     │
│             │                 │             │
│ 2. Upload   │                 │ 2. Upload   │
│    to DK    │                 │    to CA    │
│    server   │                 │    server   │
└──────┬──────┘                 └──────┬──────┘
       │                               │
       │  3. Start DM conversation     │
       ├──────────────────────────────>│
       │                               │
       │  4. Exchange keys via server  │
       │<─────────────────────────────>│
       │     (signed, verified)        │
       │                               │
       │  5. Establish Olm session     │
       │     (Double Ratchet)          │
       │<─────────────────────────────>│
       │                               │
       │  6. Encrypted message         │
       │     [encrypted payload]       │
       ├──────────────────────────────>│
       │                               │
       │                               │  7. Decrypt locally
       │                               │     with device key
       │                               │
       │  8. Reply (encrypted)         │
       │<──────────────────────────────┤
       │                               │
   9. Decrypt                          │
      locally                          │

Key Points:
• Server never sees plaintext
• Keys stored only on user devices
• Forward secrecy with ratcheting
• Verification via safety numbers
```

---

## 8. Deployment Architecture

### 8.1 Docker Compose Structure

```yaml
# docker-compose.yml
version: '3.8'

services:
  # === EDGE TIER ===
  traefik:
    image: traefik:v3.0
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - ./traefik/traefik.yml:/traefik.yml:ro
      - ./traefik/certs:/certs
    networks:
      - public-network
      - service-network
    labels:
      - "traefik.enable=true"

  # === SERVICE TIER ===
  auth-service:
    build:
      context: ./services/auth
      dockerfile: ../../docker/Dockerfile.rust-service
    environment:
      - DATABASE_URL=postgres://postgres:password@postgres:5432/platform
      - NATS_URL=nats://nats:4222
      - OIDC_ISSUER=https://auth.unityplan.org
    networks:
      - service-network
      - data-network
    depends_on:
      - postgres
      - nats
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.auth.rule=PathPrefix(`/api/auth`)"
      - "traefik.http.services.auth.loadbalancer.server.port=8001"

  user-service:
    build:
      context: ./services/user
      dockerfile: ../../docker/Dockerfile.rust-service
    environment:
      - DATABASE_URL=postgres://postgres:password@postgres:5432/platform
      - NATS_URL=nats://nats:4222
    networks:
      - service-network
      - data-network
    depends_on:
      - postgres
      - nats
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.user.rule=PathPrefix(`/api/users`)"

  course-service:
    build:
      context: ./services/course
      dockerfile: ../../docker/Dockerfile.rust-service
    environment:
      - DATABASE_URL=postgres://postgres:password@postgres:5432/platform
      - NATS_URL=nats://nats:4222
      - IPFS_URL=http://ipfs:5001
    networks:
      - service-network
      - data-network

  # === DATA TIER ===
  postgres:
    image: timescale/timescaledb:latest-pg16
    environment:
      - POSTGRES_DB=platform
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql
    networks:
      - data-network
    ports:
      - "5432:5432" # Expose for migrations only

  nats:
    image: nats:2.10-alpine
    command: "-js -sd /data"
    volumes:
      - nats-data:/data
    networks:
      - service-network
    ports:
      - "4222:4222"
      - "8222:8222" # Monitoring

  ipfs:
    image: ipfs/kubo:latest
    volumes:
      - ipfs-data:/data/ipfs
    networks:
      - data-network
    ports:
      - "4001:4001" # Swarm
      - "5001:5001" # API
      - "8080:8080" # Gateway

  matrix-synapse:
    image: matrixdotorg/synapse:latest
    environment:
      - SYNAPSE_SERVER_NAME=matrix.unityplan.org
      - SYNAPSE_REPORT_STATS=no
    volumes:
      - matrix-data:/data
      - ./matrix/homeserver.yaml:/data/homeserver.yaml
    networks:
      - service-network
      - data-network
    depends_on:
      - postgres

  # === OBSERVABILITY ===
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    networks:
      - service-network
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-data:/var/lib/grafana
      - ./grafana/dashboards:/etc/grafana/provisioning/dashboards
    networks:
      - service-network
    ports:
      - "3000:3000"

  jaeger:
    image: jaegertracing/all-in-one:latest
    environment:
      - COLLECTOR_OTLP_ENABLED=true
    networks:
      - service-network
    ports:
      - "16686:16686" # UI
      - "4317:4317"   # OTLP gRPC
      - "4318:4318"   # OTLP HTTP

networks:
  public-network:
    driver: bridge
  service-network:
    driver: bridge
    internal: true
  data-network:
    driver: bridge
    internal: true

volumes:
  postgres-data:
  nats-data:
  ipfs-data:
  matrix-data:
  prometheus-data:
  grafana-data:
```

### 8.2 CI/CD Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│                  GitHub Actions Workflow                        │
└─────────────────────────────────────────────────────────────────┘

Trigger: Push to main / Pull Request

┌────────────────────┐
│  1. Code Checkout  │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  2. Lint & Format  │
│  • cargo clippy    │
│  • cargo fmt       │
│  • eslint          │
│  • prettier        │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  3. Run Tests      │
│  • Unit tests      │
│  • Integration     │
│  • E2E tests       │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  4. Security Scan  │
│  • cargo audit     │
│  • Trivy           │
│  • Snyk            │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  5. Build Images   │
│  • Docker build    │
│  • Multi-stage     │
│  • Tag: SHA + ver  │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  6. Push to        │
│     Registry       │
│  • GHCR / ECR      │
│  • Signed images   │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  7. Deploy         │
│  • Update compose  │
│  • Rolling update  │
│  • Health checks   │
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│  8. Verify         │
│  • Smoke tests     │
│  • Monitor metrics │
└────────────────────┘
```

### 8.3 Environment Management

```
┌─────────────────────────────────────────────────────────────────┐
│                    Environment Strategy                         │
└─────────────────────────────────────────────────────────────────┘

Development
┌──────────────────────────────────────────────────────┐
│  • Local Docker Compose                              │
│  • Hot reload enabled                                │
│  • Mock external services                            │
│  • Seed data for testing                             │
│  • Debug logging                                     │
└──────────────────────────────────────────────────────┘

Staging
┌──────────────────────────────────────────────────────┐
│  • Production-like setup                             │
│  • Real integrations (OIDC, etc.)                    │
│  • Performance testing                               │
│  • UAT (User Acceptance Testing)                     │
│  • Automated E2E tests                               │
└──────────────────────────────────────────────────────┘

Production
┌──────────────────────────────────────────────────────┐
│  • High availability                                 │
│  • Backup & disaster recovery                        │
│  • Monitoring & alerting                             │
│  • Auto-scaling                                      │
│  • Blue-green deployment                             │
└──────────────────────────────────────────────────────┘
```

---

## 9. Observability

### 9.1 Logging Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Logging Pipeline                             │
└─────────────────────────────────────────────────────────────────┘

Application Logs
│
├─ Auth Service     ──┐
├─ User Service     ──┤
├─ Course Service   ──┤
├─ Forum Service    ──┤ Structured JSON logs
├─ Badge Service    ──┤ (tracing crate)
└─ Matrix Gateway   ──┘
         │
         ▼
┌──────────────────────────────────────────────────────┐
│  Log Aggregator (Vector / Fluentd)                   │
│                                                      │
│  • Parse JSON                                        │
│  • Add metadata (service, host, time)                │
│  • Filter & route                                    │
└──────────────────┬───────────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────────┐
│  Log Storage (Loki / Elasticsearch)                  │
│                                                      │
│  • Index by service, level, timestamp                │
│  • Retention: 30 days                                │
│  • Full-text search                                  │
└──────────────────┬───────────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────────┐
│  Visualization (Grafana)                             │
│                                                      │
│  • Log dashboards                                    │
│  • Real-time tail                                    │
│  • Alert on patterns                                 │
└──────────────────────────────────────────────────────┘

Log Format (JSON):
{
  "timestamp": "2025-11-04T10:30:00Z",
  "level": "INFO",
  "service": "auth-service",
  "trace_id": "abc-123-def-456",
  "span_id": "xyz-789",
  "user_id": "user-uuid",
  "message": "User logged in successfully",
  "metadata": {
    "ip": "192.168.1.1",
    "method": "POST",
    "path": "/api/auth/login"
  }
}
```

### 9.2 Metrics Collection

```
┌─────────────────────────────────────────────────────────────────┐
│                 Metrics Architecture                            │
└─────────────────────────────────────────────────────────────────┘

Application Metrics (OpenTelemetry)
│
├─ HTTP request duration       ──┐
├─ HTTP request count           ──┤
├─ Database query duration      ──┤
├─ NATS message count           ──┤ Prometheus format
├─ Badge validations/sec        ──┤
└─ Active users gauge           ──┘
         │
         ▼
┌──────────────────────────────────────────────────────┐
│  Prometheus (Scraping)                               │
│                                                      │
│  Scrape interval: 15s                                │
│  Retention: 30 days                                  │
│                                                      │
│  Targets:                                            │
│  • auth-service:9091/metrics                         │
│  • user-service:9091/metrics                         │
│  • course-service:9091/metrics                       │
│  • postgres_exporter:9187/metrics                    │
│  • nats:8222/metrics                                 │
└──────────────────┬───────────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────────┐
│  Grafana (Visualization)                             │
│                                                      │
│  Dashboards:                                         │
│  • System Overview                                   │
│  • Service Health                                    │
│  • Database Performance                              │
│  • User Activity                                     │
│  • Error Rates                                       │
└──────────────────┬───────────────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────────────┐
│  Alerting (Prometheus Alertmanager)                  │
│                                                      │
│  Rules:                                              │
│  • High error rate (>1%)                             │
│  • Database connection pool exhausted                │
│  • Disk usage >80%                                   │
│  • Service down                                      │
│                                                      │
│  Notifications: Slack, Email, PagerDuty              │
└──────────────────────────────────────────────────────┘
```

### 9.3 Distributed Tracing

```
┌─────────────────────────────────────────────────────────────────┐
│              Distributed Tracing (Jaeger)                       │
└─────────────────────────────────────────────────────────────────┘

Request: POST /api/courses/enroll

Trace ID: abc-123-def-456
│
├─ Span: HTTP POST /api/courses/enroll
│  │ Service: course-service
│  │ Duration: 145ms
│  │
│  ├─ Span: Check badge prerequisites
│  │  │ Service: course-service
│  │  │ Duration: 15ms
│  │  │
│  │  └─ Span: NATS request to badge-service
│  │     │ Service: badge-service
│  │     │ Duration: 10ms
│  │     │
│  │     └─ Span: PostgreSQL SELECT from user_badges
│  │        Duration: 5ms
│  │
│  ├─ Span: Create enrollment record
│  │  │ Service: course-service
│  │  │ Duration: 25ms
│  │  │
│  │  └─ Span: PostgreSQL INSERT into enrollments
│  │     Duration: 20ms
│  │
│  └─ Span: Publish enrollment event
│     │ Service: course-service
│     │ Duration: 5ms
│     │
│     └─ Span: NATS publish to course.enrolled
│        Duration: 2ms
│
└─ Total Duration: 145ms

Jaeger UI shows:
• Waterfall view of all spans
• Service dependencies
• Bottleneck identification
• Error propagation
```

### 9.4 Health Checks & Monitoring

```
┌─────────────────────────────────────────────────────────────────┐
│                  Service Health Checks                          │
└─────────────────────────────────────────────────────────────────┘

Each Service Exposes:
GET /health
{
  "status": "healthy",
  "timestamp": "2025-11-04T10:30:00Z",
  "version": "1.2.3",
  "checks": {
    "database": {
      "status": "up",
      "latency_ms": 5
    },
    "nats": {
      "status": "up",
      "latency_ms": 2
    },
    "dependencies": {
      "badge-service": "up",
      "user-service": "up"
    }
  }
}

GET /ready
{
  "ready": true,
  "checks": {
    "database_migrations": "complete",
    "cache_warm": true,
    "configuration_loaded": true
  }
}

Traefik Health Check Configuration:
services:
  auth-service:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8001/health"]
      interval: 10s
      timeout: 3s
      retries: 3
      start_period: 30s
```

---

## 10. Summary

### 10.1 Infrastructure Characteristics

| Aspect | Implementation |
|--------|----------------|
| **Architecture** | Microservices with event-driven communication |
| **Language** | Rust for backend services |
| **Database** | PostgreSQL + TimescaleDB with schema-per-territory |
| **Communication** | NATS for async, HTTP for sync, Matrix for federation |
| **Storage** | IPFS for decentralized file storage |
| **Security** | mTLS, JWT, E2EE, zero-trust |
| **Deployment** | Docker Compose, containerized services |
| **Observability** | Prometheus, Grafana, Jaeger, structured logging |
| **Scalability** | Independent service scaling, territory-level isolation |

### 10.2 Key Design Decisions

1. **Schema-based multi-tenancy**: Easy migration, strong isolation, independent backups
2. **Microservices over monolith**: Independent scaling, technology flexibility, fault isolation
3. **NATS for events**: Lightweight, high-performance, built-in persistence
4. **Matrix Protocol**: Open standard, federation-ready, E2EE support
5. **Rust for services**: Memory safety, performance, fearless concurrency
6. **IPFS preparation**: Future-proof for decentralization migration
7. **Observability-first**: Comprehensive monitoring from day one

### 10.3 Future Evolution

```
Phase 1: MVP (Current)              Phase 2: Scale              Phase 3: Decentralize
┌─────────────────────┐            ┌─────────────────────┐     ┌─────────────────────┐
│ • Single server     │            │ • Regional servers  │     │ • Holochain DNAs    │
│ • Docker Compose    │    ───>    │ • Kubernetes        │ ───>│ • Peer-to-peer      │
│ • Shared infra      │            │ • Auto-scaling      │     │ • User data on      │
│ • 3-5 territories   │            │ • 20+ territories   │     │   devices           │
└─────────────────────┘            └─────────────────────┘     └─────────────────────┘
```

---

*This infrastructure is designed to start simple, scale gracefully, and evolve toward full decentralization while maintaining user sovereignty and territory autonomy at every stage.*
