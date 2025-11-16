# utility-service Changelog

All notable changes to the utility-service will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned

- QR code generation endpoint (Phase 2)
- Image optimization endpoint (resize, compress)
- IPFS integration for decentralized storage
- Additional caching strategies (LRU, size-based eviction)
- Rate limiting per user (currently global)
- Metrics for cache hit/miss ratio
- Favicon fallback icons (letter avatars for domains without icons)

---

## [0.1.0-alpha.1] - 2025-11-16

**Release Stage:** Alpha (MVP Phase 1 - Production Ready)

### Added - Favicon Service (1 endpoint)

- GET /api/v1/utility/favicon - Fetch and cache website favicons
  - Query parameters: `url` (required), `size` (optional, 16|32|64|128)
  - Supports multiple favicon sources (/favicon.ico, /favicon.png, Google S2 API)
  - Image processing: resize with Lanczos3 filter, PNG output
  - Default fallback: 200x200x200 gray square
  - Returns: image/png with cache headers

### Added - Caching

- Redis-based caching with 7-day TTL
- Cache key format: `favicon:{domain}:{size}`
- Empty data validation (prevents caching failed fetches)
- ETag support for HTTP 304 Not Modified responses
- X-Cache-Status header (HIT/MISS) for monitoring

### Added - Security

- JWT authentication required (via AuthUser extractor)
- URL validation (RFC 3986 compliant)
- SSRF protection (blocks localhost, private IPs, link-local addresses)
- Rate limiting via Redis
- 1MB maximum file size limit
- 5-second fetch timeout

### Added - Health Endpoints

- GET /api/v1/health - Service health status with Redis connectivity
- GET /api/v1/ready - Readiness probe for Kubernetes/Docker
- GET /api/v1/metrics - Prometheus metrics export

### Added - Documentation

- OpenAPI 3.0 specification via utoipa
- Swagger UI at /swagger-ui/
- Bearer token authentication in Swagger UI
- Complete API documentation with examples

### Infrastructure

- All 5 required middleware components:
  1. LoggingMiddleware (request/response logging)
  2. RequestIdMiddleware (correlation tracking)
  3. SecurityHeadersMiddleware (CSP, HSTS, etc.)
  4. CORS (development mode)
  5. RateLimitMiddleware (Redis-backed)
- Shared-lib integration for consistency
- ValidatedQuery for automatic request validation
- AppError for unified error handling

### Performance

- Redis connection pooling with multiplexed async connections
- Efficient blob URL cleanup in frontend
- Image processing optimizations (Lanczos3 downscaling)
- Cache hit rate tracking via X-Cache-Status header
- Typical performance: MISS ~100ms, HIT ~5ms (20x faster)

### Configuration

- Environment-based configuration via .env
- Configurable cache TTL, timeouts, size limits
- Development/production mode support
- Logging levels (info, debug, trace)

---

## Architecture Compliance

**Fully compliant with [Architecture Standards](../../docs/architecture/README.md)**

✅ Shared Library Integration (all 5 middleware)  
✅ Code & Naming Conventions (snake_case Rust, camelCase JSON)  
✅ API Endpoint Structure (health, ready, metrics, versioned paths)  
✅ Error Handling (AppError, Result<T>, automatic JSON errors)  
✅ Validation (ValidatedQuery, validator crate, custom validators)  

---

**Category**: Infrastructure Service  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-16
