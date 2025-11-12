# Archived Services - Pre-Middleware Architecture

**Archived Date:** November 12, 2025  
**Reason:** Complete architecture redesign with comprehensive middleware patterns

---

## What Was Archived

This directory contains all service code created before the comprehensive middleware architecture redesign documented in:

- `docs/architecture/services/shared-lib/MIDDLEWARE.md`
- `docs/architecture/services/INTER-SERVICE-COMMUNICATION.md`
- `docs/architecture/services/CACHING-STRATEGY.md`
- `docs/architecture/services/ERROR-HANDLING.md`
- `docs/architecture/MIGRATIONS-MASTER.md`

### Archived Services

1. **auth-service/** - Authentication service (old architecture)
2. **user-service/** - User profile service (old architecture)
3. **user-service.old/** - Previous version of user service
4. **badge-service/** - Badge system (scaffolded from old auth-service)
5. **community-service/** - Community management (scaffolded from old auth-service)
6. **course-service/** - Learning management (scaffolded from old auth-service)
7. **event-service/** - Event management (scaffolded from old auth-service)
8. **forum-service/** - Forum system (scaffolded from old auth-service)
9. **invitation-service/** - Invitation system (scaffolded from old auth-service)
10. **ipfs-service/** - IPFS integration (scaffolded from old auth-service)
11. **notification-service/** - Notifications (scaffolded from old auth-service)
12. **settings-service/** - User settings (scaffolded from old auth-service)
13. **territory-service/** - Territory management (scaffolded from old auth-service)
14. **translation-service/** - Translation service (scaffolded from old auth-service)

### Why Archived

**Old Architecture Issues:**

- ❌ No comprehensive middleware patterns
- ❌ Inconsistent error handling
- ❌ No request ID tracking
- ❌ Missing security headers
- ❌ No rate limiting
- ❌ Incomplete CORS configuration
- ❌ No circuit breakers
- ❌ Inconsistent logging structure
- ❌ Missing health check standards
- ❌ No graceful shutdown patterns

**New Architecture Benefits:**

- ✅ 12 comprehensive middleware patterns
- ✅ Standardized error handling across all services
- ✅ Request ID tracking for distributed tracing
- ✅ Security headers (CSP, HSTS, etc.)
- ✅ Rate limiting with Redis
- ✅ Proper CORS for multi-pod deployment
- ✅ Circuit breakers for resilience
- ✅ Structured logging with context
- ✅ Health checks and metrics
- ✅ Graceful shutdown with connection draining
- ✅ Event schemas and validation
- ✅ Phase 1 (development) vs Phase 2 (production) configurations

### Migration Notes

**Database:** Old migrations also archived in `shared-lib/migrations-old-20251112/`

**New Migration Strategy:** Service-specific migrations with dependency management per `MIGRATIONS-MASTER.md`

### Reference

These services may contain useful reference code for:

- API endpoint structures
- Handler patterns
- Model definitions
- Business logic

However, all new services should follow the comprehensive middleware patterns documented in `docs/architecture/services/`.

---

## Restoration

If you need to reference old code:

```bash
# View archived services
ls -la /path/to/workspace/services/archived/

# Compare old vs new
diff -r archived/auth-service/ auth-service/
```

**DO NOT** restore these services directly - they lack the new middleware architecture.
