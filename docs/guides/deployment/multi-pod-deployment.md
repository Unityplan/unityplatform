# Multi-Pod Deployment Guide

**Purpose:** Step-by-step guide for deploying UnityPlan in multi-pod configuration  
**Target:** Phase 2 Development (Q1 2026)  
**Prerequisites:** Docker, Docker Compose v2+

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Single-Host Multi-Pod Setup](#single-host-multi-pod-setup)
3. [Multi-Host Setup (Geographic Distribution)](#multi-host-setup)
4. [Network Configuration](#network-configuration)
5. [NATS Clustering](#nats-clustering)
6. [Monitoring Setup](#monitoring-setup)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Option 1: Single-Pod (Phase 1 MVP - Current)

```bash
# Use the original single docker-compose.yml
docker compose up -d
```

### Option 2: Multi-Pod Local Development (Phase 2)

```bash
# 1. Create mesh network (shared across all stacks)
docker network create unityplan-mesh-network

# 2. Start development tools
docker compose -f docker-compose.dev.yml up -d

# 3. Start monitoring stack
docker compose -f docker-compose.monitoring.yml up -d

# 4. Start Denmark pod
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d

# 5. Start Norway pod
docker compose -f docker-compose.pod.yml -p pod-no \
  --env-file pods/norway/.env up -d

# 6. Start Sweden pod (optional)
docker compose -f docker-compose.pod.yml -p pod-se \
  --env-file pods/sweden/.env up -d
```

### Verify Deployment

```bash
# Check all running containers
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# Check NATS cluster status
curl http://192.168.60.133:8222/varz | jq '.cluster'
curl http://192.168.60.133:8223/varz | jq '.cluster'

# Check network connectivity
docker network inspect unityplan-mesh-network

# Access services
# - Dev Dashboard: http://192.168.60.133:8888
# - Grafana: http://192.168.60.133:3001
# - Prometheus: http://192.168.60.133:9090
# - Jaeger: http://192.168.60.133:16686
```

---

## Single-Host Multi-Pod Setup

### Architecture

```
Host: 192.168.60.133
├── Dev Stack (global)
│   ├── dev-dashboard (8888)
│   ├── dev-adminer (8080)
│   ├── dev-mailhog (8025)
│   └── dev-redis-commander (8082)
│
├── Monitoring Stack (global)
│   ├── monitoring-prometheus (9090)
│   ├── monitoring-grafana (3001)
│   ├── monitoring-jaeger (16686)
│   └── reverse-proxy-traefik (80, 443, 8083)
│
├── Pod Denmark
│   ├── service-postgres-dk (5432)
│   ├── service-redis-dk (6379)
│   ├── service-nats-dk (4222, 6222, 8222)
│   ├── service-ipfs-dk (5001, 8081, 4001)
│   ├── service-matrix-dk (8008)
│   └── exporters (9187, 9121, 7777, 9100, 8089)
│
├── Pod Norway
│   ├── service-postgres-no (5433)
│   ├── service-redis-no (6380)
│   ├── service-nats-no (4223, 6223, 8223)
│   ├── service-ipfs-no (5002, 8082, 4002)
│   ├── service-matrix-no (8009)
│   └── exporters (9188, 9122, 7778)
│
└── Pod Sweden
    ├── service-postgres-se (5434)
    ├── service-redis-se (6381)
    ├── service-nats-se (4224, 6224, 8224)
    ├── service-ipfs-se (5003, 8083, 4003)
    ├── service-matrix-se (8010)
    └── exporters (9189, 9123, 7779)
```

### Step-by-Step Deployment

#### 1. Prepare Environment

```bash
# Navigate to project root
cd /home/henrik/code/data/projects/unityplan_platform/workspace

# Create mesh network
docker network create unityplan-mesh-network

# Verify network
docker network ls | grep unityplan
```

#### 2. Start Development Stack

```bash
# Start dev tools
docker compose -f docker-compose.dev.yml up -d

# Verify
docker compose -f docker-compose.dev.yml ps

# Expected output:
# NAME                STATUS      PORTS
# dev-dashboard       running     0.0.0.0:8888->80/tcp
# dev-adminer         running     0.0.0.0:8080->8080/tcp
# dev-mailhog         running     0.0.0.0:1025->1025/tcp, 0.0.0.0:8025->8025/tcp
# dev-redis-commander running     0.0.0.0:8082->8081/tcp
```

#### 3. Start Monitoring Stack

```bash
# Start monitoring services
docker compose -f docker-compose.monitoring.yml up -d

# Verify
docker compose -f docker-compose.monitoring.yml ps

# Access Grafana
# URL: http://192.168.60.133:3001
# User: admin / admin
```

#### 4. Start Pod Denmark

```bash
# Start Denmark pod
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d

# Wait for postgres to be healthy
docker compose -f docker-compose.pod.yml -p pod-dk ps

# Check NATS logs
docker logs service-nats-dk | grep "Server is ready"
```

#### 5. Start Pod Norway

```bash
# Start Norway pod
docker compose -f docker-compose.pod.yml -p pod-no \
  --env-file pods/norway/.env up -d

# Verify NATS cluster formation
curl http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Should show: ["nats://service-nats-no:6223"]
```

#### 6. Start Pod Sweden (Optional)

```bash
# Start Sweden pod
docker compose -f docker-compose.pod.yml -p pod-se \
  --env-file pods/sweden/.env up -d

# Verify 3-node NATS cluster
curl http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Should show: ["nats://service-nats-no:6223", "nats://service-nats-se:6224"]
```

#### 7. Configure Redis Commander for Multi-Pod

```bash
# Edit docker-compose.dev.yml
# Update redis-commander environment:
# REDIS_HOSTS=dk:service-redis-dk:6379,no:service-redis-no:6380,se:service-redis-se:6381

# Restart redis-commander
docker compose -f docker-compose.dev.yml restart redis-commander
```

---

## Multi-Host Setup (Geographic Distribution)

### Prerequisites

- 3+ VPS instances (e.g., Hetzner, DigitalOcean)
- Ubuntu 22.04+ or Debian 12+
- Docker and Docker Compose installed
- WireGuard installed

### Server Allocation

```
Server 1 (Copenhagen): 45.123.45.67
├── Pod Denmark
└── Development Stack
└── Monitoring Stack

Server 2 (Oslo): 89.234.56.78
└── Pod Norway

Server 3 (Stockholm): 91.234.12.34
└── Pod Sweden
```

### WireGuard VPN Mesh Setup

#### Server 1 (Copenhagen - 10.0.1.1)

```bash
# Install WireGuard
sudo apt update
sudo apt install wireguard

# Generate keys
wg genkey | tee /etc/wireguard/privatekey | wg pubkey > /etc/wireguard/publickey

# Create /etc/wireguard/wg0.conf
[Interface]
PrivateKey = <server1-private-key>
Address = 10.0.1.1/24
ListenPort = 51820

[Peer]  # Server 2 (Oslo)
PublicKey = <server2-public-key>
Endpoint = 89.234.56.78:51820
AllowedIPs = 10.0.2.0/24
PersistentKeepalive = 25

[Peer]  # Server 3 (Stockholm)
PublicKey = <server3-public-key>
Endpoint = 91.234.12.34:51820
AllowedIPs = 10.0.3.0/24
PersistentKeepalive = 25

# Start WireGuard
sudo systemctl enable wg-quick@wg0
sudo systemctl start wg-quick@wg0

# Verify
sudo wg show
ping 10.0.2.1  # Oslo
ping 10.0.3.1  # Stockholm
```

#### Server 2 (Oslo - 10.0.2.1)

```bash
# /etc/wireguard/wg0.conf
[Interface]
PrivateKey = <server2-private-key>
Address = 10.0.2.1/24
ListenPort = 51820

[Peer]  # Server 1 (Copenhagen)
PublicKey = <server1-public-key>
Endpoint = 45.123.45.67:51820
AllowedIPs = 10.0.1.0/24
PersistentKeepalive = 25

[Peer]  # Server 3 (Stockholm)
PublicKey = <server3-public-key>
Endpoint = 91.234.12.34:51820
AllowedIPs = 10.0.3.0/24
PersistentKeepalive = 25

# Start WireGuard
sudo systemctl enable wg-quick@wg0
sudo systemctl start wg-quick@wg0
```

#### Server 3 (Stockholm - 10.0.3.1)

```bash
# /etc/wireguard/wg0.conf
[Interface]
PrivateKey = <server3-private-key>
Address = 10.0.3.1/24
ListenPort = 51820

[Peer]  # Server 1 (Copenhagen)
PublicKey = <server1-public-key>
Endpoint = 45.123.45.67:51820
AllowedIPs = 10.0.1.0/24
PersistentKeepalive = 25

[Peer]  # Server 2 (Oslo)
PublicKey = <server2-public-key>
Endpoint = 89.234.56.78:51820
AllowedIPs = 10.0.2.0/24
PersistentKeepalive = 25

# Start WireGuard
sudo systemctl enable wg-quick@wg0
sudo systemctl start wg-quick@wg0
```

### Deploy Pods on Multi-Host

#### Server 1 (Copenhagen)

```bash
# Deploy dev and monitoring stacks
docker compose -f docker-compose.dev.yml up -d
docker compose -f docker-compose.monitoring.yml up -d

# Deploy Pod Denmark
docker compose -f docker-compose.pod.yml -p pod-dk \
  --env-file pods/denmark/.env up -d
```

#### Server 2 (Oslo)

```bash
# Update NATS routes to use VPN IPs
# Edit pods/norway/.env:
# NATS_ROUTES=nats://10.0.1.1:6222,nats://10.0.3.1:6224

# Deploy Pod Norway
docker compose -f docker-compose.pod.yml -p pod-no \
  --env-file pods/norway/.env up -d
```

#### Server 3 (Stockholm)

```bash
# Update NATS routes to use VPN IPs
# Edit pods/sweden/.env:
# NATS_ROUTES=nats://10.0.1.1:6222,nats://10.0.2.1:6223

# Deploy Pod Sweden
docker compose -f docker-compose.pod.yml -p pod-se \
  --env-file pods/sweden/.env up -d
```

---

## Network Configuration

### Network Topology

```
┌─────────────────────────────────────┐
│     unityplan-global-net            │
│  (Dev + Monitoring services)        │
│  - dev-dashboard                    │
│  - monitoring-prometheus            │
│  - monitoring-grafana               │
└──────────────┬──────────────────────┘
               │
               ├─────────────────┬────────────────┐
               │                 │                │
┌──────────────▼──────┐ ┌────────▼──────┐ ┌──────▼────────────┐
│  pod-dk-net         │ │  pod-no-net   │ │  pod-se-net       │
│  (Pod DK internal)  │ │  (Pod NO)     │ │  (Pod SE)         │
│  - postgres-dk      │ │  - postgres-no│ │  - postgres-se    │
│  - redis-dk         │ │  - redis-no   │ │  - redis-se       │
│  - nats-dk          │ │  - nats-no    │ │  - nats-se        │
└──────────────┬──────┘ └────────┬──────┘ └──────┬────────────┘
               │                 │                │
               └────────┬────────┴────────┬───────┘
                        │                 │
               ┌────────▼─────────────────▼────────┐
               │   unityplan-mesh-network          │
               │ (Cross-pod communication)         │
               │  - NATS clustering                │
               │  - IPFS swarm                     │
               │  - Matrix federation              │
               │  - Prometheus scraping            │
               └───────────────────────────────────┘
```

### Network Commands

```bash
# Create all networks
docker network create unityplan-mesh-network
docker network create unityplan-global-net

# Inspect network
docker network inspect unityplan-mesh-network

# List containers on mesh network
docker network inspect unityplan-mesh-network --format '{{range .Containers}}{{.Name}} {{end}}'

# Test connectivity between pods
docker exec service-nats-dk ping service-nats-no
docker exec service-redis-dk redis-cli -h service-redis-no ping
```

---

## NATS Clustering

### Cluster Formation

NATS nodes automatically discover and connect to cluster routes specified in `NATS_ROUTES`.

#### Verification

```bash
# Check cluster status on each node
curl http://192.168.60.133:8222/varz | jq '.cluster'
curl http://192.168.60.133:8223/varz | jq '.cluster'
curl http://192.168.60.133:8224/varz | jq '.cluster'

# Expected output:
{
  "cluster_name": "unityplan-global",
  "addr": "0.0.0.0:6222",
  "cluster_port": 6222,
  "urls": [
    "nats://service-nats-no:6223",
    "nats://service-nats-se:6224"
  ]
}
```

### JetStream Configuration

```bash
# Connect to NATS Denmark
nats context add dk --server=nats://192.168.60.133:4222

# Create global stream (replicated across all pods)
nats stream add GLOBAL_EVENTS \
  --subjects="global.*" \
  --storage=file \
  --replicas=3 \
  --retention=limits \
  --max-age=7d \
  --max-msgs=-1 \
  --max-bytes=-1

# Create territory-specific stream
nats stream add TERRITORY_DK \
  --subjects="territory.dk.*" \
  --storage=file \
  --replicas=1 \
  --retention=limits \
  --max-age=30d

# List streams
nats stream ls

# Stream info
nats stream info GLOBAL_EVENTS
```

### Testing Cross-Pod Messaging

```bash
# Terminal 1: Subscribe on Pod Norway
nats sub --server=nats://192.168.60.133:4223 "global.test"

# Terminal 2: Publish from Pod Denmark
nats pub --server=nats://192.168.60.133:4222 "global.test" "Hello from Denmark!"

# Terminal 1 should receive the message
```

---

## Monitoring Setup

### Prometheus Federation

Create `docker/prometheus/prometheus-central.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'unityplan-central'

# No federation in single-host mode - directly scrape exporters
scrape_configs:
  # Pod Denmark exporters
  - job_name: 'postgres-dk'
    static_configs:
      - targets: ['monitoring-postgres-exporter-dk:9187']
        labels:
          pod: 'denmark'
          territory: 'dk'
  
  - job_name: 'redis-dk'
    static_configs:
      - targets: ['monitoring-redis-exporter-dk:9121']
        labels:
          pod: 'denmark'
          territory: 'dk'
  
  - job_name: 'nats-dk'
    static_configs:
      - targets: ['monitoring-nats-exporter-dk:7777']
        labels:
          pod: 'denmark'
          territory: 'dk'
  
  # Pod Norway exporters
  - job_name: 'postgres-no'
    static_configs:
      - targets: ['monitoring-postgres-exporter-no:9188']
        labels:
          pod: 'norway'
          territory: 'no'
  
  - job_name: 'redis-no'
    static_configs:
      - targets: ['monitoring-redis-exporter-no:9122']
        labels:
          pod: 'norway'
          territory: 'no'
  
  - job_name: 'nats-no'
    static_configs:
      - targets: ['monitoring-nats-exporter-no:7778']
        labels:
          pod: 'norway'
          territory: 'no'
  
  # Add more pod exporters as needed...
```

### Grafana Datasource

Grafana is already configured to use Prometheus via provisioning. No additional setup needed.

---

## Troubleshooting

### Issue: NATS Cluster Not Forming

**Symptoms:**
```bash
curl http://192.168.60.133:8222/varz | jq '.cluster.urls'
# Returns: []
```

**Solutions:**

1. **Check mesh network connectivity:**
   ```bash
   docker exec service-nats-dk ping service-nats-no
   docker exec service-nats-dk nslookup service-nats-no
   ```

2. **Verify NATS routes configuration:**
   ```bash
   docker logs service-nats-dk | grep "routes"
   docker logs service-nats-no | grep "routes"
   ```

3. **Ensure correct network attachment:**
   ```bash
   docker inspect service-nats-dk | jq '.[0].NetworkSettings.Networks'
   # Should show both pod-dk-net and mesh-network
   ```

4. **Restart NATS services in order:**
   ```bash
   docker compose -f docker-compose.pod.yml -p pod-dk restart nats
   docker compose -f docker-compose.pod.yml -p pod-no restart nats
   ```

---

### Issue: Port Conflicts on Single Host

**Symptoms:**
```
Error: Bind for 0.0.0.0:9100 failed: port is already allocated
```

**Solutions:**

1. **Disable duplicate host-level exporters:**
   - Only run `node-exporter` and `cadvisor` in ONE pod (pod-dk)
   - Comment out in other pods' .env files or docker-compose.pod.yml

2. **Use different ports:**
   - Already configured in pod .env files
   - Pod DK: 9100, 8089
   - Pod NO: 9101, 8090
   - Pod SE: 9102, 8091

---

### Issue: Database Connection Failed

**Symptoms:**
```
FATAL: database "unityplan_dk" does not exist
```

**Solutions:**

1. **Initialize database schema:**
   ```bash
   # Connect to pod database
   docker exec -it service-postgres-dk psql -U unityplan -d unityplan_dk
   
   # Run initialization script manually if not auto-executed
   \i /docker-entrypoint-initdb.d/init.sql
   ```

2. **Check postgres logs:**
   ```bash
   docker logs service-postgres-dk
   ```

---

### Issue: Prometheus Not Scraping Exporters

**Symptoms:**
```
Targets in Prometheus UI show as "down"
```

**Solutions:**

1. **Verify exporter accessibility:**
   ```bash
   curl http://192.168.60.133:9187/metrics | head
   curl http://192.168.60.133:9121/metrics | head
   ```

2. **Check Prometheus config:**
   ```bash
   docker exec monitoring-prometheus cat /etc/prometheus/prometheus.yml
   ```

3. **Reload Prometheus:**
   ```bash
   curl -X POST http://192.168.60.133:9090/-/reload
   ```

---

## Next Steps

1. **Test multi-pod setup locally**
2. **Implement cross-pod API calls**
3. **Configure Prometheus federation**
4. **Set up automated backups**
5. **Plan WireGuard VPN for multi-host**
6. **Create deployment automation (Ansible/Terraform)**

---

**Document Version:** 1.0  
**Last Updated:** November 5, 2025  
**Maintainer:** UnityPlan Platform Team
