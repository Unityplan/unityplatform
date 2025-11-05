# NATS Clustering Configuration

**Purpose:** Complete NATS clustering setup for UnityPlan multi-pod architecture  
**Scope:** Cross-pod messaging, JetStream replication, topic design  
**Status:** Production-ready configuration

---

## Table of Contents

1. [Overview](#overview)
2. [Cluster Architecture](#cluster-architecture)
3. [Configuration](#configuration)
4. [JetStream Setup](#jetstream-setup)
5. [Topic Design](#topic-design)
6. [Security](#security)
7. [Operations](#operations)

---

## Overview

UnityPlan uses NATS clustering to enable:

- **Cross-pod communication**: Messages between territories
- **Event distribution**: Global events replicated to all pods
- **Service discovery**: Microservices find each other via NATS
- **Real-time sync**: WebSocket events, notifications, state changes

### Cluster Properties

```
Cluster Name:     unityplan-global
Topology:         Full mesh (all nodes connected)
JetStream:        Enabled on all nodes
Replication:      R3 for global streams, R1 for territory streams
```

---

## Cluster Architecture

### Single-Host Development

```
┌─────────────────────────────────────────────────────────────┐
│                   unityplan-mesh-network                    │
│                     (Docker Network)                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│  │  NATS DK     │◄────►│  NATS NO     │◄────►│  NATS SE     │
│  │              │      │              │      │              │
│  │ Client: 4222 │      │ Client: 4223 │      │ Client: 4224 │
│  │ Cluster: 6222│      │ Cluster: 6223│      │ Cluster: 6224│
│  │ Monitor: 8222│      │ Monitor: 8223│      │ Monitor: 8224│
│  │              │      │              │      │              │
│  │ JetStream: ✓ │      │ JetStream: ✓ │      │ JetStream: ✓ │
│  └──────────────┘      └──────────────┘      └──────────────┘
│                                                             │
└─────────────────────────────────────────────────────────────┘

Routes Configuration:
- DK: --routes=nats://service-nats-no:6223,nats://service-nats-se:6224
- NO: --routes=nats://service-nats-dk:6222,nats://service-nats-se:6224
- SE: --routes=nats://service-nats-dk:6222,nats://service-nats-no:6223
```

### Multi-Host Production

```
┌────────────────────────────────────────────────────────────────┐
│                   WireGuard VPN Mesh Network                   │
│              (10.0.1.0/24, 10.0.2.0/24, 10.0.3.0/24)          │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  │  Copenhagen     │    │  Oslo           │    │  Stockholm      │
│  │  45.123.45.67   │    │  89.234.56.78   │    │  91.234.12.34   │
│  │  VPN: 10.0.1.1  │    │  VPN: 10.0.2.1  │    │  VPN: 10.0.3.1  │
│  │                 │    │                 │    │                 │
│  │  NATS DK        │◄──►│  NATS NO        │◄──►│  NATS SE        │
│  │  Client: 4222   │    │  Client: 4222   │    │  Client: 4222   │
│  │  Cluster: 6222  │    │  Cluster: 6222  │    │  Cluster: 6222  │
│  │  Monitor: 8222  │    │  Monitor: 8222  │    │  Monitor: 8222  │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘
│                                                                │
└────────────────────────────────────────────────────────────────┘

Routes Configuration (via VPN):
- DK: --routes=nats://10.0.2.1:6222,nats://10.0.3.1:6222
- NO: --routes=nats://10.0.1.1:6222,nats://10.0.3.1:6222
- SE: --routes=nats://10.0.1.1:6222,nats://10.0.2.1:6222
```

---

## Configuration

### Docker Compose Configuration

From `docker-compose.pod.yml`:

```yaml
nats:
  image: nats:2.10-alpine
  container_name: service-nats-${POD_ID}
  hostname: service-nats-${POD_ID}
  command:
    - "--cluster_name=unityplan-global"
    - "--cluster=nats://0.0.0.0:6222"
    - "--routes=${NATS_ROUTES}"
    - "--http_port=8222"
    - "--jetstream"
    - "--store_dir=/data"
    - "--max_payload=8MB"
  ports:
    - "${NATS_CLIENT_PORT}:4222"    # Client connections
    - "${NATS_CLUSTER_PORT}:6222"   # Cluster routes
    - "${NATS_MONITOR_PORT}:8222"   # HTTP monitoring
  volumes:
    - nats-data:/data
  networks:
    - pod-${POD_ID}-net
    - mesh-network
  restart: unless-stopped
```

### Environment Variables

From `pods/denmark/.env`:

```bash
# NATS Configuration
NATS_CLIENT_PORT=4222
NATS_CLUSTER_PORT=6222
NATS_MONITOR_PORT=8222
NATS_ROUTES=nats://service-nats-no:6223,nats://service-nats-se:6224
```

### Command-Line Flags Explained

| Flag | Value | Purpose |
|------|-------|---------|
| `--cluster_name` | `unityplan-global` | Shared cluster identifier |
| `--cluster` | `nats://0.0.0.0:6222` | Listen address for cluster traffic |
| `--routes` | `nats://service-nats-no:6223,...` | Seed routes to other nodes |
| `--http_port` | `8222` | Monitoring endpoint |
| `--jetstream` | (flag) | Enable JetStream persistence |
| `--store_dir` | `/data` | JetStream storage location |
| `--max_payload` | `8MB` | Maximum message size |

---

## JetStream Setup

### Enable JetStream on All Nodes

JetStream is enabled via `--jetstream` flag in docker-compose.

### Create Streams

#### Global Stream (Replicated R3)

```bash
# Connect to any node
nats context add dk --server=nats://192.168.60.133:4222

# Create global stream with 3 replicas
nats stream add GLOBAL_EVENTS \
  --subjects="global.*" \
  --storage=file \
  --replicas=3 \
  --retention=limits \
  --max-age=7d \
  --discard=old \
  --max-msgs-per-subject=-1

# Verify replication
nats stream info GLOBAL_EVENTS
```

**Output:**
```
Information for Stream GLOBAL_EVENTS

Configuration:

             Subjects: global.*
     Acknowledgements: true
            Retention: File - Limits
             Replicas: 3
       Discard Policy: Old
     Duplicate Window: 2m0s
     Maximum Messages: unlimited
        Maximum Bytes: unlimited
          Maximum Age: 7d0h0m0s
 Maximum Message Size: 8MB
    Maximum Consumers: unlimited

Cluster Information:

                 Name: unityplan-global
               Leader: service-nats-dk
              Replica: service-nats-no, current, seen 0.00s ago
              Replica: service-nats-se, current, seen 0.00s ago
```

#### Territory Stream (Single Replica R1)

```bash
# Create territory-specific stream (only stored in DK pod)
nats stream add TERRITORY_DK \
  --subjects="territory.dk.*" \
  --storage=file \
  --replicas=1 \
  --retention=limits \
  --max-age=30d \
  --discard=old

# Create similar streams for other territories
nats stream add TERRITORY_NO --subjects="territory.no.*" --replicas=1 --max-age=30d
nats stream add TERRITORY_SE --subjects="territory.se.*" --replicas=1 --max-age=30d
```

#### Cross-Territory Stream (R3 for important events)

```bash
nats stream add CROSS_TERRITORY \
  --subjects="cross.*.*.*" \
  --storage=file \
  --replicas=3 \
  --retention=limits \
  --max-age=14d \
  --discard=old

# Example subjects:
# cross.dk.no.transfer      (Denmark → Norway event)
# cross.*.*.collaboration   (Any territory pair)
```

### Create Consumers

```bash
# Durable consumer for global events
nats consumer add GLOBAL_EVENTS global-processor \
  --filter="global.*" \
  --ack=explicit \
  --replay=instant \
  --deliver=all \
  --max-deliver=-1

# Territory-specific consumer
nats consumer add TERRITORY_DK dk-local-processor \
  --filter="territory.dk.*" \
  --ack=explicit \
  --replay=instant \
  --deliver=all
```

---

## Topic Design

### Naming Convention

```
{scope}.{origin}.{domain}.{action}

Scopes:
  global.*           - Global events (replicated R3)
  territory.{id}.*   - Territory-local events (R1)
  cross.{from}.{to}.* - Cross-territory events (R3)

Examples:
  global.user.registered
  global.content.published
  territory.dk.user.login
  territory.dk.group.created
  cross.dk.no.collaboration_invite
  cross.*.*.learning_shared
```

### Topic Hierarchy

```
global.*
├── global.user.*
│   ├── global.user.registered
│   ├── global.user.suspended
│   └── global.user.profile_updated
├── global.content.*
│   ├── global.content.published
│   ├── global.content.flagged
│   └── global.content.archived
├── global.system.*
│   ├── global.system.maintenance
│   ├── global.system.alert
│   └── global.system.config_changed

territory.{dk|no|se}.*
├── territory.dk.user.*
│   ├── territory.dk.user.login
│   ├── territory.dk.user.logout
│   └── territory.dk.user.session_expired
├── territory.dk.group.*
│   ├── territory.dk.group.created
│   ├── territory.dk.group.member_added
│   └── territory.dk.group.deleted
├── territory.dk.data.*
│   ├── territory.dk.data.export_requested
│   └── territory.dk.data.backup_completed

cross.{from}.{to}.*
├── cross.dk.no.collaboration_invite
├── cross.dk.*.learning_shared
├── cross.*.*.knowledge_exchange
└── cross.*.*.federation_event
```

### Subject Wildcards

```bash
# Subscribe to all global events
nats sub "global.*"

# Subscribe to all user events across all territories
nats sub "*.user.*"

# Subscribe to all Denmark events
nats sub "territory.dk.>"

# Subscribe to cross-territory events from Denmark
nats sub "cross.dk.>"
```

---

## Security

### Authentication (Future Enhancement)

#### JWT-based Authentication

```bash
# Generate operator key
nsc add operator UnityPlan

# Create accounts for each territory
nsc add account DK
nsc add account NO
nsc add account SE

# Create users
nsc add user --account DK dk-api-service
nsc add user --account NO no-api-service

# Export JWT config
nsc generate config --sys-account SYS > nats-auth.conf
```

#### NATS Configuration with Auth

```yaml
# nats-auth.conf (future)
operator: /nats/operator.jwt

system_account: SYS

resolver: {
  type: full
  dir: '/nats/jwt'
}
```

### TLS Encryption (Production)

#### Generate Certificates

```bash
# Generate CA certificate
openssl genrsa -out ca-key.pem 2048
openssl req -new -x509 -sha256 -key ca-key.pem -out ca.pem -days 3650

# Generate server certificate for each node
openssl genrsa -out server-key-dk.pem 2048
openssl req -new -sha256 -key server-key-dk.pem -out server-dk.csr -subj "/CN=service-nats-dk"
openssl x509 -req -in server-dk.csr -CA ca.pem -CAkey ca-key.pem -CAcreateserial -out server-cert-dk.pem -days 365
```

#### NATS TLS Configuration

```yaml
nats:
  command:
    - "--tls"
    - "--tlscert=/nats/certs/server-cert.pem"
    - "--tlskey=/nats/certs/server-key.pem"
    - "--tlscacert=/nats/certs/ca.pem"
  volumes:
    - ./nats-certs:/nats/certs:ro
```

---

## Operations

### Health Checks

#### HTTP Monitoring Endpoint

```bash
# Get server stats
curl http://192.168.60.133:8222/varz | jq

# Get connection info
curl http://192.168.60.133:8222/connz | jq

# Get subscription routing
curl http://192.168.60.133:8222/subsz | jq

# Get cluster routes
curl http://192.168.60.133:8222/routez | jq

# JetStream info
curl http://192.168.60.133:8222/jsz | jq
```

#### Key Metrics

```bash
# Cluster health
curl http://192.168.60.133:8222/varz | jq '{
  cluster_name: .cluster.name,
  cluster_port: .cluster.port,
  routes: .cluster.urls | length,
  connections: .connections,
  in_msgs: .in_msgs,
  out_msgs: .out_msgs,
  in_bytes: .in_bytes,
  out_bytes: .out_bytes
}'
```

### CLI Operations

#### Install NATS CLI

```bash
# Linux/macOS
curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sh

# Or via package manager
brew install nats-io/nats-tools/nats  # macOS
```

#### Create Contexts

```bash
# Add contexts for each pod
nats context add dk --server=nats://192.168.60.133:4222 --description="Pod Denmark"
nats context add no --server=nats://192.168.60.133:4223 --description="Pod Norway"
nats context add se --server=nats://192.168.60.133:4224 --description="Pod Sweden"

# Select context
nats context select dk

# List contexts
nats context ls
```

#### Stream Management

```bash
# List all streams
nats stream ls

# Stream details
nats stream info GLOBAL_EVENTS

# View messages
nats stream view GLOBAL_EVENTS

# Purge stream (delete all messages)
nats stream purge GLOBAL_EVENTS --force

# Delete stream
nats stream rm GLOBAL_EVENTS
```

#### Consumer Management

```bash
# List consumers
nats consumer ls GLOBAL_EVENTS

# Consumer info
nats consumer info GLOBAL_EVENTS global-processor

# Get next message
nats consumer next GLOBAL_EVENTS global-processor

# Consume continuously
nats consumer sub GLOBAL_EVENTS global-processor
```

#### Testing Messaging

```bash
# Publish message
nats pub global.user.registered '{"user_id": 123, "email": "user@example.com"}'

# Subscribe to topic
nats sub global.user.registered

# Request-reply pattern
nats reply "api.user.get" '{"status": "ok", "user": {...}}'
nats request "api.user.get" '{"user_id": 123}'

# Benchmark
nats bench global.test --msgs=10000 --size=1024 --pub=10 --sub=10
```

### Monitoring with Prometheus

#### Prometheus Scrape Config

```yaml
# docker/prometheus/prometheus.yml
scrape_configs:
  - job_name: 'nats-dk'
    static_configs:
      - targets: ['monitoring-nats-exporter-dk:7777']
        labels:
          pod: 'denmark'
          cluster: 'unityplan-global'
  
  - job_name: 'nats-no'
    static_configs:
      - targets: ['monitoring-nats-exporter-no:7778']
        labels:
          pod: 'norway'
          cluster: 'unityplan-global'
```

#### Key Metrics

```promql
# Total messages in/out
rate(gnatsd_varz_in_msgs[5m])
rate(gnatsd_varz_out_msgs[5m])

# Bytes in/out
rate(gnatsd_varz_in_bytes[5m])
rate(gnatsd_varz_out_bytes[5m])

# Active connections
gnatsd_varz_connections

# Cluster size
gnatsd_varz_routes

# JetStream storage usage
gnatsd_jsz_total_store
gnatsd_jsz_total_memory

# Stream message count
nats_stream_messages{stream="GLOBAL_EVENTS"}
```

### Backup and Recovery

#### Backup JetStream Data

```bash
# Backup stream
nats stream backup GLOBAL_EVENTS /backup/global_events_$(date +%Y%m%d).tar.gz

# Backup all streams
for stream in $(nats stream ls -n); do
  nats stream backup $stream /backup/${stream}_$(date +%Y%m%d).tar.gz
done
```

#### Restore Stream

```bash
# Restore stream
nats stream restore GLOBAL_EVENTS /backup/global_events_20251105.tar.gz
```

#### Manual Volume Backup

```bash
# Stop NATS container
docker compose -f docker-compose.pod.yml -p pod-dk stop nats

# Backup volume
docker run --rm -v unityplan_nats-data:/data -v $(pwd)/backup:/backup \
  alpine tar czf /backup/nats-data-dk-$(date +%Y%m%d).tar.gz -C /data .

# Restart NATS
docker compose -f docker-compose.pod.yml -p pod-dk start nats
```

### Scaling Considerations

#### Adding New Pod (e.g., Finland)

1. **Create pod environment file:**
   ```bash
   # pods/finland/.env
   POD_ID=fi
   POD_NAME=finland
   NATS_CLIENT_PORT=4225
   NATS_CLUSTER_PORT=6225
   NATS_MONITOR_PORT=8225
   NATS_ROUTES=nats://service-nats-dk:6222,nats://service-nats-no:6223,nats://service-nats-se:6224
   ```

2. **Update existing pods' routes:**
   ```bash
   # Add to DK, NO, SE .env files:
   NATS_ROUTES=...,nats://service-nats-fi:6225
   ```

3. **Deploy new pod:**
   ```bash
   docker compose -f docker-compose.pod.yml -p pod-fi \
     --env-file pods/finland/.env up -d
   ```

4. **Verify cluster:**
   ```bash
   curl http://192.168.60.133:8222/varz | jq '.cluster.urls'
   # Should now show 3 routes (NO, SE, FI)
   ```

#### Remove Pod from Cluster

1. **Stop pod:**
   ```bash
   docker compose -f docker-compose.pod.yml -p pod-se down
   ```

2. **Update remaining pods' routes:**
   ```bash
   # Remove SE from DK and NO .env files
   NATS_ROUTES=nats://service-nats-no:6223  # DK
   NATS_ROUTES=nats://service-nats-dk:6222  # NO
   ```

3. **Restart remaining pods:**
   ```bash
   docker compose -f docker-compose.pod.yml -p pod-dk restart nats
   docker compose -f docker-compose.pod.yml -p pod-no restart nats
   ```

---

## Best Practices

1. **Topic Design:**
   - Use hierarchical naming for easier filtering
   - Replicate only what's necessary (R3 for critical global, R1 for local)
   - Use wildcards wisely (avoid `>` on high-volume topics)

2. **JetStream:**
   - Set appropriate retention policies (7d for global, 30d for territory)
   - Use explicit acknowledgments for critical messages
   - Monitor storage usage (`gnatsd_jsz_total_store`)

3. **Clustering:**
   - Always use full mesh topology for 3-5 nodes
   - Use seed routes (--routes) instead of auto-discovery
   - Monitor cluster health (`routez` endpoint)

4. **Security:**
   - Enable TLS in production
   - Use JWT authentication for multi-tenant isolation
   - Rotate credentials regularly

5. **Operations:**
   - Regular backups of JetStream data
   - Monitor Prometheus metrics for anomalies
   - Test cluster recovery procedures

---

**Document Version:** 1.0  
**Last Updated:** November 5, 2025  
**Maintainer:** UnityPlan Platform Team
