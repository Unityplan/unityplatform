# Docker Configuration Files

This directory contains **configuration files only** for Docker services. All runtime data is stored in Docker named volumes or excluded from version control.

## 📋 Directory Structure

```
docker/
├── README.md (this file)
├── grafana/
│   └── provisioning/          # Grafana dashboards and datasources
│       ├── dashboards/
│       │   ├── dashboards.yml
│       │   ├── multi-pod-overview.json
│       │   └── pod-denmark-overview.json
│       └── datasources/
│           └── datasources.yml
├── placeholder-pages/
│   └── frontend.html          # Placeholder page template
├── postgres/
│   └── init.sql               # Database initialization script
├── prometheus/
│   ├── prometheus.yml         # Main Prometheus config
│   └── prometheus-central.yml # Central Prometheus config (multi-pod)
└── traefik/
    ├── traefik.yml            # Traefik reverse proxy config
    ├── error-pages/
    │   └── 503.html           # Service unavailable page
    └── landing-page/
        └── index.html         # Landing page template
```

## 🎯 Purpose

**What's in git (this directory):**
- ✅ Configuration files (YAML, JSON, SQL)
- ✅ Template files (HTML pages)
- ✅ Provisioning scripts
- ✅ Static resources needed for services

**What's NOT in git (runtime data):**
- ❌ Database files (`postgres-data/`, etc.)
- ❌ Service state (`grafana-data/`, `prometheus-data/`)
- ❌ Cache directories
- ❌ Log files
- ❌ Any `*-data/` directories

Runtime data is stored in **Docker named volumes** managed by docker-compose.

## 📦 Configuration Files

### Grafana

**Location:** `grafana/provisioning/`

**Dashboards:**
- `multi-pod-overview.json` - Global view of all territory pods
- `pod-denmark-overview.json` - Denmark pod metrics

**Datasources:**
- `datasources.yml` - Prometheus datasource configuration

**Usage:** Automatically provisioned when Grafana container starts.

---

### PostgreSQL

**Location:** `postgres/init.sql`

**Purpose:** Database initialization script for pod deployments.

**Contains:**
- Database creation
- Schema setup
- Initial permissions

**Usage:** Executed once when PostgreSQL container is first created.

---

### Prometheus

**Location:** `prometheus/`

**Files:**
- `prometheus.yml` - Single-pod monitoring configuration
- `prometheus-central.yml` - Multi-pod federation configuration

**Usage:** Mounted as config in Prometheus containers.

---

### Traefik

**Location:** `traefik/`

**Files:**
- `traefik.yml` - Main reverse proxy configuration
- `error-pages/503.html` - Service unavailable page
- `landing-page/index.html` - Default landing page

**Usage:** Traefik routes and error handling.

---

### Placeholder Pages

**Location:** `placeholder-pages/`

**Files:**
- `frontend.html` - Placeholder shown before frontend is deployed

**Usage:** Served by Traefik when frontend service is unavailable.

## 🔧 Runtime Data Storage

Runtime data for Docker services is stored in **named volumes** defined in docker-compose files:

**Development Services (`docker-compose.dev.yml`):**
- `forgejo-data` - Git repository data (⚠️ IMPORTANT: Contains all git repos!)
- `registry-data` - Docker registry images

**Monitoring Services (`docker-compose.monitoring.yml`):**
- `prometheus-data` - Metrics time-series database
- `grafana-data` - Grafana dashboards and settings
- `traefik-data` - Traefik certificates and state

**Pod Services (`docker-compose.pod.yml`):**
- `postgres-data` - PostgreSQL databases
- `redis-data` - Redis cache
- `nats-data` - NATS JetStream data
- `ipfs-data` - IPFS distributed storage
- `matrix-data` - Matrix homeserver data

**To list all volumes:**
```bash
docker volume ls | grep unityplatform
```

**To backup a volume:**
```bash
docker run --rm -v unityplatform-dev_forgejo-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/forgejo-backup.tar.gz -C /data .
```

## 🚀 Related Files

**Docker Compose Files (in project root):**
- `docker-compose.dev.yml` - Development tools (Forgejo, Adminer, etc.)
- `docker-compose.monitoring.yml` - Monitoring stack (Prometheus, Grafana)
- `docker-compose.pod.yml` - Single-territory pod
- `docker-compose.multi-territory-pod.yml` - Multi-territory pod

**Configuration Docs:**
- See `../scripts/README.md` for management scripts
- See `../MULTI-POD-README.md` for multi-pod deployment
- See `../monitoring/README.md` for dashboard documentation

## ⚠️ Important Notes

**Do NOT commit runtime data:**
- All `*-data/` directories are gitignored
- Runtime data is stored in Docker volumes
- Forgejo volume contains all git repository data - back up regularly!

**Modifying configs:**
1. Edit config files in this directory
2. Restart affected services: `docker compose -f <file> restart <service>`
3. Commit config changes to git

**Volume management:**
- List volumes: `docker volume ls`
- Inspect volume: `docker volume inspect <volume-name>`
- Remove unused volumes: `docker volume prune` (⚠️ careful!)

## 📚 See Also

- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Grafana Provisioning](https://grafana.com/docs/grafana/latest/administration/provisioning/)
- [Prometheus Configuration](https://prometheus.io/docs/prometheus/latest/configuration/configuration/)
- [Traefik Configuration](https://doc.traefik.io/traefik/)

---

**Last Updated:** 2025-11-14  
**Maintainer:** Unity Platform Team
