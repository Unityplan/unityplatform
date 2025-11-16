# Utility Service API

**Version:** 0.1.0-alpha.1  
**Base URL:** `http://localhost:8014/api/v1/utility`  
**Authentication:** Required (JWT Bearer token)

---

## Table of Contents

- [Authentication](#authentication)
- [Health & Monitoring](#health--monitoring)
- [Favicon Utilities](#favicon-utilities)
- [Error Responses](#error-responses)

---

## Authentication

All endpoints require a valid JWT token in the Authorization header:

```http
Authorization: Bearer <token>
```

**Exception**: Health endpoints (`/health`, `/ready`, `/metrics`) are public.

---

## Health & Monitoring

### GET /api/v1/health

Health check endpoint for service status.

**Authentication:** None (public)

**Response:**

```json
{
  "status": "healthy",
  "service": "utility-service",
  "version": "0.1.0-alpha.1",
  "uptime": 3600,
  "redis": {
    "connected": true,
    "latency_ms": 2.3
  }
}
```

**Status Codes:**

- `200 OK` - Service is healthy
- `503 Service Unavailable` - Service is unhealthy

---

### GET /api/v1/ready

Readiness probe for service startup.

**Authentication:** None (public)

**Response:**

```json
{
  "ready": true,
  "service": "utility-service"
}
```

**Status Codes:**

- `200 OK` - Service is ready
- `503 Service Unavailable` - Service is not ready

---

### GET /api/v1/metrics

Prometheus metrics endpoint.

**Authentication:** None (public)

**Response:** Prometheus text format

```prometheus
# HELP utility_service_info Service information
# TYPE utility_service_info gauge
utility_service_info{version="0.1.0-alpha.1"} 1

# HELP utility_service_http_requests_total Total HTTP requests
# TYPE utility_service_http_requests_total counter
utility_service_http_requests_total{method="GET",path="/api/v1/utility/favicon",status="200"} 1523

# HELP utility_service_favicon_cache_hits_total Total favicon cache hits
# TYPE utility_service_favicon_cache_hits_total counter
utility_service_favicon_cache_hits_total 1203

# HELP utility_service_favicon_cache_misses_total Total favicon cache misses
# TYPE utility_service_favicon_cache_misses_total counter
utility_service_favicon_cache_misses_total 320
```

---

## Favicon Utilities

### GET /api/v1/utility/favicon

Fetch and cache a website's favicon.

**Authentication:** Required

**Query Parameters:**

| Parameter | Type   | Required | Default | Description                                   |
| --------- | ------ | -------- | ------- | --------------------------------------------- |
| `url`     | string | Yes      | -       | Website URL (e.g., `https://github.com`)      |
| `size`    | number | No       | 32      | Icon size in pixels (16, 32, 64, 128)         |

**Request Example:**

```http
GET /api/v1/utility/favicon?url=https://github.com&size=32
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response:** Image binary data (PNG format)

**Headers:**

```http
Content-Type: image/png
Cache-Control: public, max-age=604800
ETag: "a1b2c3d4e5f6"
X-Cache-Status: HIT
```

**Cache Status Values:**

- `HIT` - Served from Redis cache
- `MISS` - Fetched from website and cached
- `BYPASS` - Served from fallback (fetch failed)

**Status Codes:**

- `200 OK` - Favicon returned successfully
- `400 Bad Request` - Invalid URL or size parameter
- `401 Unauthorized` - Missing or invalid authentication token
- `404 Not Found` - Favicon not found, default icon returned
- `429 Too Many Requests` - Rate limit exceeded
- `500 Internal Server Error` - Server error
- `503 Service Unavailable` - External fetch timeout or failure

---

## Error Responses

All errors follow the standard `AppError` format from shared-lib:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid URL format",
    "details": {
      "field": "url",
      "reason": "Must be a valid HTTP or HTTPS URL"
    }
  },
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Common Error Codes

| Code                  | HTTP Status | Description                        |
| --------------------- | ----------- | ---------------------------------- |
| `VALIDATION_ERROR`    | 400         | Invalid request parameters         |
| `UNAUTHORIZED`        | 401         | Missing or invalid authentication  |
| `RATE_LIMIT_EXCEEDED` | 429         | Too many requests                  |
| `INTERNAL_ERROR`      | 500         | Server error                       |
| `SERVICE_UNAVAILABLE` | 503         | Redis unavailable or fetch timeout |

### Validation Rules

**URL Validation:**

- Must be valid HTTP or HTTPS URL
- Must not be localhost or internal IP address
- Must not exceed 2048 characters
- Domain must be valid

**Size Validation:**

- Must be one of: 16, 32, 64, 128
- Default: 32 pixels

---

## Rate Limiting

Default limits (configured via `RateLimitMiddleware`):

- **Authenticated users**: 100 requests per minute per user
- **Per endpoint**: 1000 requests per minute total

**Headers:**

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1700000000
```

---

## Caching Behavior

### Redis Cache

- **Key Format**: `favicon:{domain}:{size}`
- **TTL**: 7 days (604800 seconds)
- **Expiration**: Automatic (Redis TTL)

### Client-Side Caching

```http
Cache-Control: public, max-age=604800
ETag: "hash-of-content"
```

**Recommended client behavior:**

1. Cache response for 7 days
2. Use ETag for conditional requests
3. Validate on stale cache

---

## Future Endpoints (Phase 2)

### QR Code Generation

```http
GET /api/v1/utility/qr?data={text}&size={pixels}
```

### Image Optimization

```http
POST /api/v1/utility/optimize
Content-Type: multipart/form-data

{
  "image": <file>,
  "format": "webp",
  "quality": 85,
  "max_width": 1920
}
```

### Thumbnail Generation

```http
POST /api/v1/utility/thumbnail
Content-Type: multipart/form-data

{
  "image": <file>,
  "width": 150,
  "height": 150,
  "crop": "center"
}
```

---

**Last Updated:** November 16, 2025  
**Maintained By:** Development Team
