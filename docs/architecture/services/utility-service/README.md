# Utility Service

**Version:** 0.1.0-alpha.1  
**Port:** 8014  
**Phase:** Phase 1 (Infrastructure Service)  
**Status:** 🚧 In Development

---

## Overview

The **utility-service** is a lightweight infrastructure service that provides utility functions for other services and the frontend. It handles tasks that don't fit into core business services, such as favicon fetching, image optimization, QR code generation, and other helper functions.

### Key Characteristics

- **Stateless Design**: No database required
- **Redis Caching**: Fast retrieval with automatic cache management
- **Multi-Pod Ready**: Cache-first design, future IPFS integration for cross-pod sharing
- **High Performance**: Optimized for speed and scalability

---

## Responsibilities

### What This Service DOES

- ✅ **Favicon Fetching**: Retrieve and cache website favicons
- ✅ **Image Caching**: Store and serve frequently accessed images
- ✅ **Future Utilities**: QR codes, image resizing, PDF generation (Phase 2)

### What This Service DOES NOT Do

- ❌ **Business Logic**: No user data, badges, territories, etc.
- ❌ **Data Storage**: No database (Redis cache only)
- ❌ **Event Publishing**: No NATS events (stateless utilities)

---

## Dependencies

### Infrastructure Dependencies

- **Redis** (required): Caching layer
  - Cache TTL: 7 days for favicons
  - Automatic expiration and cleanup

### Service Dependencies

- **None**: This is a foundational infrastructure service
- **Future**: IPFS service (Phase 2) for cross-pod asset sharing

### Database

- **None**: Stateless service with Redis cache only

### NATS Events

- **None**: Not applicable for utility functions
- **Future**: Phase 2 may add cross-pod synchronization via IPFS

---

## API Endpoints

See [API.md](API.md) for complete specifications.

### Health & Monitoring

- `GET /api/v1/health` - Health check
- `GET /api/v1/ready` - Readiness probe
- `GET /api/v1/metrics` - Prometheus metrics

### Utilities

- `GET /api/v1/utility/favicon` - Fetch website favicon
- **Future**: QR codes, image optimization, etc.

---

## Configuration

### Environment Variables

```env
# Service Configuration
SERVICE_NAME=utility-service
SERVICE_PORT=8014
SERVICE_HOST=0.0.0.0
LOG_LEVEL=info

# Redis (from shared-lib AppConfig)
APP__REDIS__URL=redis://localhost:6379

# Utility Settings
UTILITY__FAVICON__CACHE_TTL=604800      # 7 days in seconds
UTILITY__FAVICON__STORAGE_PATH=uploads/favicons
UTILITY__FAVICON__DEFAULT_SIZE=32
UTILITY__FAVICON__MAX_SIZE=128
UTILITY__FAVICON__MAX_FILE_SIZE=1048576  # 1MB
UTILITY__FAVICON__FETCH_TIMEOUT=5        # 5 seconds
```

### Security Settings

- **URL Validation**: Blocks localhost, internal IPs, and invalid URLs
- **Rate Limiting**: Via shared-lib `RateLimitMiddleware`
- **Size Limits**: Max 1MB per favicon download
- **Timeout**: 5-second timeout on external fetches
- **Authentication**: Requires valid JWT token

---

## Architecture

### Caching Strategy

```text
Request Flow:
1. Check Redis cache (key: favicon:{domain}:{size})
2. If HIT → return cached image
3. If MISS → fetch from website
4. Store in Redis (7-day TTL)
5. Return image

Fallback:
- If fetch fails → return default icon
- If Redis fails → direct fetch (degraded mode)
```

### Storage Structure

```text
uploads/favicons/
├── github.com.32.png
├── twitter.com.32.png
├── linkedin.com.64.png
└── default.png (fallback icon)
```

**Future (Phase 2)**: IPFS integration for cross-pod asset sharing

---

## Deployment

### Docker Configuration

**Service Name**: `service-utility-dk`  
**Port**: 8014  
**Network**: `unityplatform-pod-dk-net`

### Health Checks

- **Health endpoint**: `GET /health`
- **Ready endpoint**: `GET /ready`
- **Startup probe**: 10 seconds
- **Liveness probe**: Every 30 seconds

---

## Standards Compliance

### ✅ Required Components (Phase 1)

- ✅ ALL 5 core middleware (Logging, RequestId, Security, CORS, RateLimit)
- ✅ `ValidatedQuery` for request validation
- ✅ `AuthUser` for JWT authentication
- ✅ `AppError` and `Result<T>` for error handling
- ✅ Health endpoints: `/api/v1/health`, `/api/v1/ready`
- ✅ Metrics endpoint: `/api/v1/metrics` (Prometheus format)
- ✅ API naming: `/api/v1/utility/{function}`
- ✅ OpenAPI/Swagger documentation
- ✅ Graceful shutdown
- ✅ Workspace dependencies

### ⚠️ Special Cases (Infrastructure Service)

- ⚠️ **No Database**: Stateless design with Redis cache only
- ⚠️ **No NATS**: Not applicable for utility functions (Phase 1)

---

## Metrics

### Standard Prometheus Metrics

```prometheus
utility_service_info{version="0.1.0-alpha.1"} 1
utility_service_http_requests_total{method, path, status}
utility_service_http_request_duration_seconds{method, path}
utility_service_errors_total{type}
```

### Service-Specific Metrics

```prometheus
utility_service_favicon_cache_hits_total
utility_service_favicon_cache_misses_total
utility_service_favicon_fetch_duration_seconds
utility_service_favicon_fetch_failures_total{reason}
```

---

## Development Status

### Phase 1 (Current)

- 🚧 **Favicon fetching and caching** - In Development
- 🚧 **Redis cache integration** - In Development
- 🚧 **URL validation and security** - In Development

### Phase 2 (Planned)

- ⏳ **IPFS integration** for cross-pod asset sharing
- ⏳ **QR code generation**
- ⏳ **Image optimization/resizing**
- ⏳ **PDF generation**
- ⏳ **Thumbnail generation**

---

## Related Documentation

- [API.md](API.md) - Complete API specifications
- [../../README.md](../../README.md) - Architecture overview
- [../shared-lib/MIDDLEWARE.md](../shared-lib/MIDDLEWARE.md) - Middleware documentation

---

**Last Updated:** November 16, 2025  
**Maintained By:** Development Team
