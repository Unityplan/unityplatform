# Multi-Pod Architecture Design

**Document Version:** 1.0  
**Last Updated:** November 5, 2025  
**Target Phase:** Phase 2 (Scale & Federation)

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Principles](#architecture-principles)
3. [Pod Structure](#pod-structure)
4. [Stack Organization](#stack-organization)
5. [Network Architecture](#network-architecture)
6. [Data Architecture](#data-architecture)
7. [Cross-Pod Communication](#cross-pod-communication)
8. [Monitoring & Observability](#monitoring--observability)
9. [Deployment Topologies](#deployment-topologies)
10. [Migration Path](#migration-path)

---

## Overview

The UnityPlan platform is designed to scale globally while maintaining **user sovereignty** and **data locality**. The multi-pod architecture enables:

- **Geographic distribution** - Each country/territory operates its own pod
- **Data sovereignty** - User data stays within territorial boundaries
- **Fault isolation** - Pod failures don't cascade globally
- **Scalable growth** - Add new territories without disrupting existing ones

### Pod Definition

A **pod** is a self-contained set of services serving a specific **territory** (country, region, or administrative area). Each pod:

- Maintains its own database with territory-specific schemas
- Handles local user traffic and data storage
- Communicates with other pods via NATS messaging
- Shares authentication through global federation layer

---

## Architecture Principles

### 1. **Territory-First Design**

```
User (Denmark) → Pod Denmark → territory_DK schema
User (Norway)  → Pod Norway  → territory_NO schema
User (Sweden)  → Pod Sweden  → territory_SE schema
```

### 2. **Global Services + Local Pods**

```
┌─────────────────────────────────────────────────────────────┐
│                     GLOBAL LAYER                            │
│  - Central Monitoring (Prometheus Federation, Grafana)     │
│  - Development Tools (Adminer, MailHog, Dashboard)         │
│  - Global Auth Service (SSO, JWT validation)               │
│  - NATS Cluster Coordinator                                │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│   Pod DK     │◄────►│   Pod NO     │◄────►│   Pod SE     │
│ (Denmark)    │ NATS │ (Norway)     │ NATS │ (Sweden)     │
│              │      │              │      │              │
│ - Postgres   │      │ - Postgres   │      │ - Postgres   │
│ - Redis      │      │ - Redis      │      │ - Redis      │
│ - NATS       │      │ - NATS       │      │ - NATS       │
│ - IPFS       │      │ - IPFS       │      │ - IPFS       │
│ - Matrix     │      │ - Matrix     │      │ - Matrix     │
└──────────────┘      └──────────────┘      └──────────────┘
```

### 3. **Federation Over Centralization**

- No single point of failure
- Each pod operates independently
- Global state synchronized via NATS JetStream
- Matrix federation for cross-pod communication

### 4. **User Sovereignty**

- Users choose their home territory
- Data stored in user's preferred jurisdiction
- Right to data portability between territories
- Transparent governance per territory

---

## Pod Structure

### Service Categories per Pod

#### **Core Services (Per Territory)**

```yaml
# Pod Denmark Example
services:
  service-postgres-dk:
    # PostgreSQL 16 + TimescaleDB
    # Contains: territory_DK schema, global schema (read-only replicas)
    
  service-redis-dk:
    # Territory-specific cache
    # Session storage, query cache
    
  service-nats-dk:
    # NATS node in global cluster
    # Routes to other pods: nats-no, nats-se, etc.
    
  service-ipfs-dk:
    # IPFS node in global swarm
    # Pins territory-specific content
    
  service-matrix-dk:
    # Matrix homeserver
    # Federated with other territory homeservers
```

#### **Pod-Local Exporters**

```yaml
  monitoring-postgres-exporter-dk:
  monitoring-redis-exporter-dk:
  monitoring-nats-exporter-dk:
  monitoring-node-exporter-dk:   # Host metrics
  monitoring-cadvisor-dk:        # Container metrics
```

#### **Optional Pod-Local Services**

```yaml
  service-api-gateway-dk:     # Territory-specific API endpoint
  service-websocket-dk:       # Real-time connections
  service-search-dk:          # Meilisearch for territory data
```

---

### Pod Deployment Models

UnityPlan supports two pod deployment models:

#### **Model 1: Single-Territory Pod** (Dedicated Infrastructure)

**Use Case:** Large territories with high traffic (USA, India, Germany)

```yaml
Pod Denmark (DK):
  - PostgreSQL: unityplan_dk database
  - Redis: dk:* key namespace
  - NATS: territory.dk.* topics
  - User Base: 1M+ users
  - Resource: Dedicated VPS/Cloud instance

Pod Norway (NO):
  - PostgreSQL: unityplan_no database
  - Redis: no:* key namespace
  - NATS: territory.no.* topics
  - User Base: 500K+ users
  - Resource: Dedicated VPS/Cloud instance
```

**Benefits:**
- Full resource isolation
- Independent scaling
- Simple to manage
- Clear cost attribution per territory

**Challenges:**
- Higher cost for small territories
- Underutilized resources for low-traffic territories

---

#### **Model 2: Multi-Territory Pod** (Shared Infrastructure)

**Use Case:** Small/medium territories in same geographic region

```yaml
Pod Europe (EU):
  - PostgreSQL: 
      * unityplan_de (Germany)
      * unityplan_fr (France)
      * unityplan_es (Spain)
  - Redis: de:*, fr:*, es:* key namespaces
  - NATS: territory.de.*, territory.fr.*, territory.es.* topics
  - Combined User Base: 300K users
  - Resource: Single VPS/Cloud instance in EU region

Pod Asia-Pacific (AP):
  - PostgreSQL:
      * unityplan_sg (Singapore)
      * unityplan_my (Malaysia)
      * unityplan_th (Thailand)
  - Combined User Base: 200K users
  - Resource: Single VPS/Cloud instance in Singapore
```

**Benefits:**
- Cost efficient for smaller territories
- Low latency for geographically close countries
- Shared infrastructure costs
- Easy to split into dedicated pods when needed

**Data Isolation:**
- **PostgreSQL**: Separate databases with schema-per-territory
- **Redis**: Key namespace prefixing (`de:user:123`, `fr:user:456`)
- **NATS**: Topic-based routing (`territory.de.*`, `territory.fr.*`)
- **IPFS**: Content-addressed (naturally shared, saves storage)

**Migration Path:**
```
Start: 3 territories on shared pod (EU)
  ↓
Growth: Germany outgrows shared pod
  ↓
Split: Germany → dedicated pod-de
       France + Spain remain on pod-eu
  ↓
Future: Each territory gets dedicated pod as needed
```

**Example: Europe Multi-Territory Pod**

```yaml
# pods/europe/.env
POD_ID=eu
TERRITORY_CODES=DE,FR,ES  # Multiple territories

# Single PostgreSQL with 3 databases
POSTGRES_DB_DE=unityplan_de
POSTGRES_DB_FR=unityplan_fr
POSTGRES_DB_ES=unityplan_es

# Shared Redis (key prefixing)
# Keys: de:session:abc, fr:cache:xyz, es:user:123

# Shared NATS cluster node
# Topics: territory.de.*, territory.fr.*, territory.es.*
```

**Territory-Specific Configuration:**

```bash
# Germany
TERRITORY_DE_TIMEZONE=Europe/Berlin
TERRITORY_DE_LOCALE=de_DE.UTF-8
TERRITORY_DE_LANGUAGE=de

# France
TERRITORY_FR_TIMEZONE=Europe/Paris
TERRITORY_FR_LOCALE=fr_FR.UTF-8
TERRITORY_FR_LANGUAGE=fr

# Spain
TERRITORY_ES_TIMEZONE=Europe/Madrid
TERRITORY_ES_LOCALE=es_ES.UTF-8
TERRITORY_ES_LANGUAGE=es
```

**Database Schema Example:**

```sql
-- unityplan_de database
CREATE SCHEMA global;        -- Replicated data
CREATE SCHEMA territory_DE;  -- Germany-specific data

-- unityplan_fr database
CREATE SCHEMA global;        -- Replicated data
CREATE SCHEMA territory_FR;  -- France-specific data

-- unityplan_es database
CREATE SCHEMA global;        -- Replicated data
CREATE SCHEMA territory_ES;  -- Spain-specific data
```

**Deployment:**

```bash
# Deploy multi-territory pod
docker compose -f docker-compose.multi-territory-pod.yml -p pod-eu \
  --env-file pods/europe/.env up -d

# Creates:
# - 3 PostgreSQL databases (de, fr, es)
# - Shared Redis, NATS, IPFS
# - All exporters labeled with pod=europe
```

---

### Choosing the Right Model

| Factor | Single-Territory Pod | Multi-Territory Pod |
|--------|---------------------|---------------------|
| **User Base** | 500K+ users | < 500K users combined |
| **Geographic Spread** | Any location | Same region/continent |
| **Cost** | Higher | Lower (shared resources) |
| **Complexity** | Simple | Moderate (multi-DB setup) |
| **Scalability** | Independent | Grouped scaling |
| **Best For** | Major countries/regions | Smaller territories nearby |

**Real-World Examples:**

**Single-Territory Pods:**
- United States (pod-us)
- India (pod-in)
- Brazil (pod-br)
- China (pod-cn)

**Multi-Territory Pods:**
- Europe: Germany, France, Spain (pod-eu)
- Nordics: Iceland, Greenland (pod-nordic)
- Caribbean: Jamaica, Trinidad, Barbados (pod-caribbean)
- Pacific: Fiji, Samoa, Tonga (pod-pacific)

---

## Stack Organization

### Three-Stack Architecture

#### **1. Development Stack (Global - Single Instance)**

**Purpose:** Developer productivity and debugging  
**Scope:** Connects to ALL pods  
**Deployment:** Once per development environment

```yaml
# docker-compose.dev.yml
services:
  dev-dashboard:
    # Landing page with links to all tools
    
  dev-adminer:
    # Database UI - can connect to all pod databases
    # Connections: postgres-dk, postgres-no, postgres-se
    
  dev-mailhog:
    # Email testing for all pods
    
  dev-redis-commander:
    # Redis UI - can connect to all pod Redis instances
```

**Access Pattern:**
- Developers use single dashboard to access any pod
- Debugging tools have global visibility
- No production traffic touches dev stack

#### **2. Monitoring Stack (Global - Single Instance with Federation)**

**Purpose:** Unified observability across all pods  
**Scope:** Aggregates metrics/traces from all pods  
**Deployment:** Once per environment

```yaml
# docker-compose.monitoring.yml
services:
  monitoring-prometheus:
    # Central Prometheus with federation
    # Scrapes from all pod-local Prometheus instances
    
  monitoring-grafana:
    # Dashboards for all pods
    # Data source: Federated Prometheus
    
  monitoring-jaeger:
    # Central trace collector
    # All pods send traces here
    
  monitoring-alertmanager:
    # Alert routing and deduplication
```

**Metrics Flow:**
```
Pod DK exporters → Pod DK Prometheus → Central Prometheus → Grafana
Pod NO exporters → Pod NO Prometheus ────────┘
Pod SE exporters → Pod SE Prometheus ────────┘
```

#### **3. Service Stack (Per Territory Pod)**

**Purpose:** Serve territory-specific users and data  
**Scope:** Isolated per territory  
**Deployment:** One per territory/country

```yaml
# docker-compose.pod.yml (template)
services:
  service-postgres-${POD_ID}:
    environment:
      POSTGRES_DB: unityplan_${POD_ID}
      
  service-redis-${POD_ID}:
    # Territory cache
    
  service-nats-${POD_ID}:
    # Clustered with other pods
    command:
      - "--cluster=nats://0.0.0.0:6222"
      - "--routes=nats://nats-dk:6222,nats://nats-no:6222"
```

---

## Network Architecture

### Development Environment (Single Host)

```
┌────────────────────────────────────────────────────────────┐
│                Host: 192.168.60.133                        │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │ global-net   │  │ pod-dk-net   │  │ pod-no-net   │   │
│  │ (bridge)     │  │ (bridge)     │  │ (bridge)     │   │
│  │              │  │              │  │              │   │
│  │ - Dashboard  │  │ - Postgres   │  │ - Postgres   │   │
│  │ - Prometheus │  │ - Redis      │  │ - Redis      │   │
│  │ - Grafana    │  │ - NATS       │  │ - NATS       │   │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘   │
│         │                 │                 │            │
│         └─────────────────┴─────────────────┘            │
│                  mesh-network (bridge)                   │
│            (cross-pod communication)                     │
└────────────────────────────────────────────────────────────┘
```

**Networks:**
- `global-net` - Dev and monitoring stack
- `pod-dk-net` - Denmark pod internal
- `pod-no-net` - Norway pod internal
- `mesh-network` - Cross-pod NATS clustering, IPFS swarm

### Production Environment (Multi-Host)

```
┌─────────────────────────┐         ┌─────────────────────────┐
│ Server DK (Copenhagen)  │         │ Server NO (Oslo)        │
│ Public: 45.123.45.67   │         │ Public: 89.234.56.78   │
│ VPN: 10.0.1.1          │◄───────►│ VPN: 10.0.2.1          │
│                         │WireGuard│                         │
│ Pod DK Services         │         │ Pod NO Services         │
│ - postgres-dk:5432     │         │ - postgres-no:5432     │
│ - nats-dk:4222,6222    │         │ - nats-no:4222,6222    │
└─────────────────────────┘         └─────────────────────────┘
            │                                   │
            └───────────────┬───────────────────┘
                            │
                ┌───────────▼───────────┐
                │ Central Monitoring    │
                │ Server (Global)       │
                │ - Prometheus          │
                │ - Grafana             │
                │ - Jaeger              │
                └───────────────────────┘
```

**WireGuard VPN Mesh:**
- Each pod server has VPN peer connection to others
- Private 10.x.x.x network for pod-to-pod traffic
- Public IPs only for user-facing services
- Encrypted tunnels over internet

---

## Data Architecture

### PostgreSQL Schema-Per-Territory

```sql
-- Pod Denmark Database
unityplan_dk
├── public (extensions, shared functions)
├── global (read-only replica from central)
│   ├── territories
│   ├── badge_definitions
│   └── global_policies
└── territory_DK (read-write, Denmark data)
    ├── users
    ├── communities
    ├── posts
    ├── projects
    └── local_policies

-- Pod Norway Database
unityplan_no
├── public
├── global (read-only replica)
└── territory_NO (read-write, Norway data)
    ├── users
    ├── communities
    └── ...
```

### Territory ID Format Standard

#### 🆔 Territory ID Format

UnityPlan uses a standardized territory identification system that respects sovereignty for both countries and First Nations.

##### **Countries** (ISO 3166-1 Alpha-2)

Uses standard two-letter country codes:

| ID | Name | Notes |
|----|------|-------|
| `US` | United States | ISO 3166-1 |
| `CA` | Canada | ISO 3166-1 |
| `AU` | Australia | ISO 3166-1 |
| `NZ` | New Zealand | ISO 3166-1 |
| `MX` | Mexico | ISO 3166-1 |
| `GB` | United Kingdom | ISO 3166-1 |
| `FR` | France | ISO 3166-1 |
| `DK` | Denmark | ISO 3166-1 |
| `NO` | Norway | ISO 3166-1 |
| `SE` | Sweden | ISO 3166-1 |

**Total**: 249 countries (ISO 3166-1 standard)

##### **First Nations** ({NAME}-FN-{COUNTRY})

Format prioritizes First Nation name, followed by FN marker, followed by country code for **geographic context only**:

**Format**: `{NAME}-FN-{COUNTRY}`

**CRITICAL**: The country code in a First Nation ID is **GEOGRAPHIC METADATA ONLY** and does **NOT** create a parent-child relationship. First Nations are **top-level sovereign territories** with `parent_territory = None`.

**Examples**:

| ID | Name | Country Context | parent_territory | Notes |
|----|------|-----------------|------------------|-------|
| `HAIDA-FN-CA` | Haida Nation | Canada (geographic) | `None` | Independent, Pacific Northwest |
| `NAVAJO-FN-US` | Navajo Nation | United States (geographic) | `None` | Independent, largest US tribe |
| `CREE-FN-CA` | Cree Nation | Canada (geographic) | `None` | Independent, largest FN in Canada |
| `CHEROKEE-FN-US` | Cherokee Nation | United States (geographic) | `None` | Independent, second largest US tribe |
| `YOLNGU-FN-AU` | Yolngu people | Australia (geographic) | `None` | Independent, Northern Territory |
| `MAORI-FN-NZ` | Māori | New Zealand (geographic) | `None` | Independent, Indigenous Polynesian |
| `ZAPOTEC-FN-MX` | Zapotec people | Mexico (geographic) | `None` | Independent, Oaxaca region |
| `INUIT-FN-CA` | Inuit | Canada (geographic) | `None` | Independent, Arctic regions |
| `SAMI-FN-NO` | Sámi people | Norway (geographic) | `None` | Independent (also in SE, FI, RU) |

**Sovereignty Principles**:
- ✅ First Nation name comes **first** (respects sovereignty)
- ✅ `FN` marker clearly identifies as First Nation
- ✅ Country code provides **geographic context** (prevents name collisions)
- ✅ **`parent_territory = None`** (top-level, equal to countries)
- ✅ First Nations control their own communities (e.g., `HAIDA-FN-CA-MASSETT`)
- ✅ Self-identification respected (registered name is authoritative)

**Why Keep Country Code?**
1. **Prevents name collisions**: `EAGLE-FN-CA` vs `EAGLE-FN-US` are distinct
2. **Geographic context**: Helps users understand location
3. **No power implication**: Code is metadata, NOT hierarchy
4. **Practical**: Matches how First Nations often identify themselves internationally

**What This Means for Governance**:
- TeacherRegistrar for `CA` **CANNOT** manage `HAIDA-FN-CA` (separate hierarchies)
- TeacherRegistrar for `HAIDA-FN-CA` **CANNOT** manage `CA` (separate hierarchies)
- Each is sovereign within their own hierarchy
- Platform respects Indigenous self-determination

##### **Communities** ({PARENT}-{NAME})

Communities are nested within countries or First Nations:

**Format**: `{PARENT_ID}-{COMMUNITY_NAME}`

**Examples**:

| ID | Name | Parent | Type |
|----|------|--------|------|
| `US-CA-SF` | San Francisco | United States → California | City |
| `HAIDA-FN-CA-MASSETT` | Massett | Haida Nation (Canada) | Village |
| `CA-BC-VANCOUVER` | Vancouver | Canada → British Columbia | City |
| `NAVAJO-FN-US-WINDOW-ROCK` | Window Rock | Navajo Nation (US) | Capital |
| `AU-NSW-SYDNEY` | Sydney | Australia → New South Wales | City |
| `DK-COPENHAGEN` | Copenhagen | Denmark | City |
| `NO-OSLO` | Oslo | Norway | City |
| `SE-STOCKHOLM` | Stockholm | Sweden | City |

**Hierarchy Depth**: Unlimited (communities can nest within communities)

**Database Implementation**:

```sql
-- territories table
CREATE TABLE global.territories (
  id VARCHAR(100) PRIMARY KEY,  -- e.g., 'DK', 'HAIDA-FN-CA', 'DK-COPENHAGEN'
  name VARCHAR(255) NOT NULL,
  type VARCHAR(50) NOT NULL,    -- 'country', 'first_nation', 'community'
  parent_territory VARCHAR(100), -- NULL for top-level (countries & First Nations)
  pod_id VARCHAR(10),           -- Which pod serves this territory
  created_at TIMESTAMPTZ DEFAULT NOW(),
  
  FOREIGN KEY (parent_territory) REFERENCES global.territories(id),
  CHECK (
    -- Top-level sovereignty: countries and First Nations have no parent
    (type IN ('country', 'first_nation') AND parent_territory IS NULL)
    OR
    -- Communities must have a parent
    (type = 'community' AND parent_territory IS NOT NULL)
  )
);

-- Example data
INSERT INTO global.territories (id, name, type, parent_territory, pod_id) VALUES
  ('DK', 'Denmark', 'country', NULL, 'dk'),
  ('NO', 'Norway', 'country', NULL, 'no'),
  ('CA', 'Canada', 'country', NULL, 'ca'),
  ('HAIDA-FN-CA', 'Haida Nation', 'first_nation', NULL, 'haida-fn-ca'),
  ('NAVAJO-FN-US', 'Navajo Nation', 'first_nation', NULL, 'navajo-fn-us'),
  ('DK-COPENHAGEN', 'Copenhagen', 'community', 'DK', 'dk'),
  ('HAIDA-FN-CA-MASSETT', 'Massett', 'community', 'HAIDA-FN-CA', 'haida-fn-ca');
```

### Data Residency Rules

1. **User Home Territory**
   - User chooses home territory during signup
   - All personal data stored in home territory pod
   - User can migrate to different territory (data export/import)

2. **Cross-Territory Visibility**
   - Public posts visible globally via NATS events
   - Cached in local Redis for performance
   - Foreign data accessed via API calls to origin pod

3. **Global Data (Replicated)**
   - Badge definitions
   - Global policies
   - Territory metadata
   - Synchronized via NATS JetStream

### Replication Strategy

```yaml
# Option 1: Logical Replication (PostgreSQL)
# Central DB → Read-only replicas to each pod
global_db:
  replication: logical
  subscribers:
    - pod-dk (global schema)
    - pod-no (global schema)
    - pod-se (global schema)

# Option 2: NATS-based Event Sourcing
# Changes to global data published as events
# Each pod consumes events and updates local copy
```

---

## Cross-Pod Communication

### NATS Clustering Configuration

#### Pod Denmark NATS

```yaml
service-nats-dk:
  image: nats:latest
  command:
    - "-js"                                    # Enable JetStream
    - "-m=8222"                               # Monitoring port
    - "--store_dir=/data"                     # JetStream storage
    - "--cluster=nats://0.0.0.0:6222"        # Cluster port
    - "--cluster_name=unityplan-global"      # Cluster name
    - "--routes=nats://service-nats-no:6222,nats://service-nats-se:6222"
  ports:
    - "4222:4222"   # Client
    - "6222:6222"   # Cluster
    - "8222:8222"   # Monitoring
```

#### Pod Norway NATS

```yaml
service-nats-no:
  image: nats:latest
  command:
    - "-js"
    - "-m=8222"
    - "--store_dir=/data"
    - "--cluster=nats://0.0.0.0:6222"
    - "--cluster_name=unityplan-global"
    - "--routes=nats://service-nats-dk:6222,nats://service-nats-se:6222"
  ports:
    - "4223:4222"   # Different host port (same host dev)
    - "6223:6222"
    - "8223:8222"
```

### NATS Topic Design

```
# Territory-specific events
territory.dk.users.created
territory.dk.posts.published
territory.no.community.joined

# Global events (broadcast to all pods)
global.auth.session.validated
global.badge.awarded
global.policy.updated

# Cross-territory events
cross.dk.no.message.sent   # DK user → NO user
cross.*.*.*.search.query   # Search across all territories
```

### JetStream Streams

```bash
# Create global policy stream (replicated across all pods)
nats stream add GLOBAL_POLICIES \
  --subjects="global.policy.*" \
  --storage=file \
  --replicas=3 \
  --max-age=7d

# Create territory-specific stream
nats stream add TERRITORY_DK \
  --subjects="territory.dk.*" \
  --storage=file \
  --replicas=1 \
  --max-age=30d
```

### IPFS Swarm

```yaml
service-ipfs-dk:
  environment:
    - IPFS_SWARM_KEY=/key/swarm/psk/1.0.0/.../unityplan-private-swarm
    - LIBP2P_FORCE_PNET=1
  # Peers automatically discover each other
  # Territory-specific content pinned locally
  # Global content replicated across pods
```

---

## Monitoring & Observability

### Prometheus Federation

**Central Prometheus Configuration:**

```yaml
# /etc/prometheus/prometheus.yml (central)
scrape_configs:
  # Federate from Pod Denmark
  - job_name: 'federate-pod-dk'
    honor_labels: true
    metrics_path: '/federate'
    params:
      'match[]':
        - '{job=~"postgres|redis|nats|node|cadvisor"}'
    static_configs:
      - targets: ['prometheus-pod-dk:9090']
        labels:
          pod: 'denmark'
          territory: 'dk'

  # Federate from Pod Norway
  - job_name: 'federate-pod-no'
    honor_labels: true
    metrics_path: '/federate'
    params:
      'match[]':
        - '{job=~"postgres|redis|nats|node|cadvisor"}'
    static_configs:
      - targets: ['prometheus-pod-no:9090']
        labels:
          pod: 'norway'
          territory: 'no'
```

**Pod-Local Prometheus (Example: Denmark):**

```yaml
# /etc/prometheus/prometheus.yml (pod-dk)
global:
  external_labels:
    pod: 'denmark'
    territory: 'dk'

scrape_configs:
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter-dk:9187']
  
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter-dk:9121']
  
  - job_name: 'nats'
    static_configs:
      - targets: ['nats-exporter-dk:7777']
  
  - job_name: 'node'
    static_configs:
      - targets: ['node-exporter-dk:9100']
  
  - job_name: 'cadvisor'
    static_configs:
      - targets: ['cadvisor-dk:8080']
```

### Grafana Dashboard Organization

```
Grafana (Central)
├── Home Dashboard (all pods overview)
├── Pod Dashboards/
│   ├── Pod Denmark Overview
│   ├── Pod Norway Overview
│   └── Pod Sweden Overview
├── Service Dashboards/
│   ├── PostgreSQL (all pods)
│   ├── Redis (all pods)
│   ├── NATS Cluster (global)
│   └── IPFS Swarm (global)
└── Territory Analytics/
    ├── User Activity by Territory
    └── Cross-Territory Interactions
```

### Distributed Tracing

```yaml
# All pods send traces to central Jaeger
services:
  service-api-gateway-dk:
    environment:
      JAEGER_AGENT_HOST: jaeger-central
      JAEGER_AGENT_PORT: 6831
      JAEGER_SERVICE_NAME: api-gateway-dk
      JAEGER_TAGS: pod=denmark,territory=dk
```

**Trace Context Propagation:**
- User request → Pod DK API
- Pod DK → NATS message → Pod NO
- Pod NO processes → response
- Full trace visible in Jaeger with pod tags

---

## Deployment Topologies

### Topology 1: Single-Host Development

**Use Case:** Local development, testing  
**Setup:** All pods on 192.168.60.133

```bash
# Start global services
docker compose -f docker-compose.dev.yml up -d
docker compose -f docker-compose.monitoring.yml up -d

# Start territory pods
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d

docker compose -f docker-compose.pod.yml -p pod-no \
  --env-file pods/norway/.env up -d
```

**Pros:**
- Simple setup
- Fast iteration
- No network complexity

**Cons:**
- No real latency testing
- Resource intensive on single machine

---

### Topology 2: Multi-VM Simulation

**Use Case:** Testing cross-pod communication, latency  
**Setup:** VMs on same host or cloud

```
VM1 (10.0.1.10): Pod Denmark + Dev Tools
VM2 (10.0.2.10): Pod Norway
VM3 (10.0.3.10): Pod Sweden
VM4 (10.0.4.10): Central Monitoring
```

**Network Simulation:**
```bash
# Add latency between VMs (Linux tc)
sudo tc qdisc add dev eth0 root netem delay 50ms 10ms distribution normal
```

**Pros:**
- Realistic network conditions
- True isolation
- Scalability testing

---

### Topology 3: Geographic Distribution (Production)

**Use Case:** Real multi-region deployment  
**Setup:** Physical servers or VPS in different countries

```
Copenhagen Server (45.x.x.x):  Pod Denmark
Oslo Server (89.x.x.x):        Pod Norway
Stockholm Server (91.x.x.x):   Pod Sweden
Frankfurt Server (3.x.x.x):    Central Monitoring
```

**Requirements:**
- WireGuard VPN mesh
- DNS for service discovery
- CDN for static assets
- Load balancers per region

**WireGuard Configuration:**

```ini
# /etc/wireguard/wg0.conf (Pod Denmark)
[Interface]
PrivateKey = <dk-private-key>
Address = 10.0.1.1/24
ListenPort = 51820

[Peer]  # Pod Norway
PublicKey = <no-public-key>
Endpoint = 89.x.x.x:51820
AllowedIPs = 10.0.2.0/24

[Peer]  # Pod Sweden
PublicKey = <se-public-key>
Endpoint = 91.x.x.x:51820
AllowedIPs = 10.0.3.0/24
```

---

## Migration Path

### Phase 1: MVP (Current - Nov 2025)
**Status:** Single docker-compose, all services  
**Action:** Complete MVP features, test schema-per-territory

```
✅ Single docker-compose.yml
✅ PostgreSQL with territory_DK, territory_NO schemas
✅ NATS JetStream (single node)
✅ Monitoring stack
```

---

### Phase 2: Split Stacks (Q1 2026)
**Goal:** Prepare for multi-pod deployment  
**Action:** Split into dev/monitoring/pod compose files

**Deliverables:**
- `docker-compose.dev.yml`
- `docker-compose.monitoring.yml`
- `docker-compose.pod.yml` (template)
- `pods/denmark/.env`, `pods/norway/.env`

**Testing:**
- Run 2-3 pod simulation on single host
- Validate NATS clustering
- Test Prometheus federation
- Cross-pod data access patterns

---

### Phase 3: Multi-Host Staging (Q2 2026)
**Goal:** Test geographic distribution  
**Action:** Deploy to 2-3 VPS instances

**Infrastructure:**
- Hetzner/DigitalOcean VPS in EU regions
- WireGuard VPN between servers
- DNS setup (pod-dk.unityplan.org, pod-no.unityplan.org)
- Monitoring central location

**Validation:**
- Latency testing (Copenhagen ↔ Oslo)
- Failover scenarios
- Data consistency checks
- User experience from different territories

---

### Phase 4: Production Multi-Region (Q3 2026)
**Goal:** Launch with 3-5 territory pods  
**Action:** Production deployment

**Pods:**
- Denmark (Copenhagen)
- Norway (Oslo)
- Sweden (Stockholm)
- Germany (Frankfurt) - Optional
- Poland (Warsaw) - Optional

**Production Checklist:**
- [ ] SSL certificates (Let's Encrypt)
- [ ] Database backups (per pod)
- [ ] NATS JetStream replication
- [ ] Monitoring alerts
- [ ] Incident response runbooks
- [ ] User documentation (territory selection)
- [ ] GDPR compliance verification

---

## Operational Considerations

### Pod Health Monitoring

```yaml
# Healthcheck endpoints per pod
GET /health/live   → 200 if pod is running
GET /health/ready  → 200 if pod can serve traffic
GET /health/cluster → NATS cluster status

# Critical Alerts
- Pod Postgres down → Page on-call
- NATS partition detected → Warning
- Redis memory > 80% → Warning
- Disk space < 10% → Page on-call
```

### Disaster Recovery

**Scenario 1: Single Pod Failure**
- Users routed to nearest healthy pod
- Data restored from backups
- NATS replays missed events from JetStream

**Scenario 2: Network Partition**
- Pods operate independently (AP in CAP theorem)
- Eventual consistency via NATS
- Conflict resolution on partition heal

**Scenario 3: Data Center Loss**
- Daily backups to object storage (S3/Backblaze)
- Restore pod in new location
- Rejoin NATS cluster

### Scaling Strategy

**Vertical Scaling (Per Pod):**
- Increase PostgreSQL resources
- Add Redis memory
- More CPU for NATS

**Horizontal Scaling (Add Pods):**
- New territory pod in 30 minutes
- Automated setup via terraform/ansible
- Self-register with NATS cluster

---

## Summary

This multi-pod architecture provides:

✅ **User Sovereignty** - Data stays in user's chosen territory  
✅ **Scalability** - Add territories independently  
✅ **Resilience** - Pod failures are isolated  
✅ **Performance** - Local data, low latency  
✅ **Federation** - Cross-pod communication via NATS  
✅ **Observability** - Centralized monitoring with federation  

**Next Steps:**
1. Create docker-compose splits (Phase 2 prep)
2. Document NATS clustering setup
3. Test 2-pod simulation locally
4. Plan WireGuard VPN mesh for multi-host

---

**Document Maintainer:** UnityPlan Platform Team  
**Review Schedule:** Quarterly or after major architecture changes
