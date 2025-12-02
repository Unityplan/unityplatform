# Task Scheduler Service

**Port:** 8015  
**Version:** 0.1.0-alpha.1  
**Status:** ✅ Complete (2/2 endpoints)  
**Type:** Infrastructure Service

Background task scheduler for automated cleanup jobs and GDPR-compliant user deletion.

---

## Overview

The Task Scheduler Service handles:
- **Automated Cleanup Jobs** - Scheduled cron jobs for system maintenance
- **User Data Deletion** - GDPR-compliant hard deletion of user data after 30 days
- **NATS Event Monitoring** - Subscribes to platform events for audit logging
- **Manual Admin Operations** - On-demand cleanup triggers for administrators

### Key Features

- ✅ Weekly automated cleanup (Sundays at 2:00 AM UTC)
- ✅ Transaction-safe deletion across all services
- ✅ Dry-run mode for testing and statistics
- ✅ NATS event subscription for user deletion tracking
- ✅ Admin-only endpoints with badge-based authorization
- ✅ Comprehensive audit logging

---

## Architecture

### Deletion Flow

```
User Requests Deletion (auth-service)
  ↓
┌─────────────────────────────────────────┐
│ 1. Soft Delete                          │
│    - auth-service marks user deleted    │
│    - Publishes user.deleted NATS event  │
│    - Sets deleted_at timestamp          │
└─────────────────────────────────────────┘
  ↓
┌─────────────────────────────────────────┐
│ 2. Services Receive Event               │
│    - All services subscribe to event    │
│    - Soft-delete their user data        │
│    - User cannot login anymore          │
└─────────────────────────────────────────┘
  ↓
Days 1-29: Waiting Period
  ↓
┌─────────────────────────────────────────┐
│ 3. Scheduled Cleanup (Day 30+)          │
│    - Cron job: Sunday 2:00 AM UTC       │
│    - Query global.registry_username     │
│    - Find users with deleted_at > 30d   │
└─────────────────────────────────────────┘
  ↓
┌─────────────────────────────────────────┐
│ 4. Hard Deletion (Transaction-Safe)     │
│    - user-service: 8 tables             │
│    - badge-service: 2 tables            │
│    - community-service: 3 tables        │
│    - auth-service: 2 tables             │
│    - global registries: 2 tables        │
└─────────────────────────────────────────┘
  ↓
✅ Complete GDPR-Compliant Deletion
```

### Cron Schedule

| Job | Schedule | Description |
|-----|----------|-------------|
| User Cleanup | `0 0 2 * * Sun` | Every Sunday at 2:00 AM UTC - Hard delete users soft-deleted >30 days ago |

---

## API Endpoints

### 1. Manual Cleanup Trigger

**Endpoint:** `POST /api/v1/cleanup/users`  
**Auth:** Required (JWT + `task-admin` badge)  
**Description:** Manually trigger user cleanup job

**Request Body:**
```json
{
  "force": false,     // Optional: bypass time checks
  "dryRun": true      // Optional: test without deleting
}
```

**Response:**
```json
{
  "success": true,
  "stats": {
    "softDeletedUsers": 5,
    "eligibleForDeletion": 3,
    "deleted": 3,
    "territoriesProcessed": ["dk", "no"]
  },
  "executionTimeMs": 1250
}
```

**Use Cases:**
- Test cleanup logic before scheduled run
- Immediate cleanup for compliance requirements
- Generate statistics about pending deletions

---

### 2. Get Cleanup Statistics

**Endpoint:** `GET /api/v1/cleanup/stats`  
**Auth:** Required (JWT + `task-admin` badge)  
**Description:** View cleanup statistics without deleting (dry-run mode)

**Response:**
```json
{
  "success": true,
  "stats": {
    "softDeletedUsers": 5,
    "eligibleForDeletion": 3,
    "deleted": 0,
    "territoriesProcessed": ["dk", "no"]
  },
  "executionTimeMs": 450
}
```

**Use Cases:**
- Monitor pending deletions
- Audit soft-deleted users
- Compliance reporting

---

## NATS Events

### Subscribed Events

| Event | Subject | Purpose |
|-------|---------|---------|
| User Deleted | `user.deleted` | Audit logging of user deletion requests |

**Event Payload:**
```json
{
  "userId": "uuid",
  "territory": "dk",
  "deletedAt": "2025-12-02T10:30:00Z"
}
```

**Behavior:**
- Logs event for audit trail
- Does NOT perform deletion (services handle their own soft-delete)
- Hard deletion occurs via scheduled cron job after 30 days

---

## Security

### Required Permissions

All endpoints require:
- ✅ Valid JWT token
- ✅ `task-admin` badge

### Badge Assignment

```bash
# Grant task-admin badge to a user (requires badge-admin)
POST /api/v1/badges/award
{
  "userId": "user-uuid",
  "badgeSlug": "task-admin"
}
```

---

## Data Deletion Details

### Services & Tables Cleaned

**user-service (8 tables):**
- `user_users_profiles`
- `user_users_profile_links`
- `user_users_profile_language_proficiency`
- `user_users_connections` (both user_id and target_user_id)
- `user_users_data_exports`
- `user_users_account_deletion_requests`
- `user_users_settings`

**badge-service (2 tables):**
- `badge_users_badges`
- `badge_users_progress`

**community-service (3 tables):**
- `community_communities_members`
- `community_communities_managers`
- `community_communities` (created_by set to NULL)

**invitation-service (1 table):**
- `invitation_invitations_tokens` (created_by set to NULL)

**auth-service (2 tables):**
- `auth_refresh_tokens`
- `auth_users_core`

**Global Registries (2 tables):**
- `global.registry_username`
- `global.registry_email`

### Transaction Safety

All deletions occur within a PostgreSQL transaction:
- ✅ All-or-nothing deletion
- ✅ Rollback on any error
- ✅ Maintains referential integrity
- ✅ No orphaned data

---

## Configuration

**Environment Variables:**

```bash
# Database (required)
DATABASE_URL=postgresql://user:pass@localhost:5432/unityplatform_dk

# NATS (required)
NATS_URL=nats://localhost:4222
NATS_CLUSTER_NAME=unityplatform-cluster

# Server (optional)
SERVER_HOST=0.0.0.0
SERVER_PORT=8015

# Redis (required for rate limiting)
REDIS_URL=redis://localhost:6379
```

---

## Development

### Running Locally

```bash
# Start infrastructure
docker-compose -f docker-compose.dev.yml up -d

# Run service
cd services/task-scheduler-service
cargo run
```

### Testing

```bash
# Run tests
cargo test -p task-scheduler-service

# Check cleanup stats (requires task-admin badge)
curl -H "Authorization: Bearer $JWT_TOKEN" \
  http://localhost:8015/api/v1/cleanup/stats

# Trigger dry-run cleanup
curl -X POST -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"dryRun": true}' \
  http://localhost:8015/api/v1/cleanup/users
```

### Swagger UI

Access interactive API documentation:
```
http://localhost:8015/swagger-ui/
```

---

## Monitoring

### Logs

Service logs all operations with structured logging:

```
2025-12-02T02:00:00Z INFO Running scheduled user cleanup job
2025-12-02T02:00:01Z INFO Found 3 users eligible for hard deletion
2025-12-02T02:00:02Z INFO Successfully hard-deleted user user_id=abc territory=dk
2025-12-02T02:00:05Z INFO Cleanup job completed deleted=3 eligible=3
```

### Metrics

**TODO:** Prometheus metrics for:
- Cleanup job execution time
- Number of users deleted per run
- Failed deletion attempts
- NATS event processing rate

---

## Troubleshooting

### Common Issues

**1. "Requires task-admin badge"**
- Solution: Award task-admin badge via badge-service
- Admin must have `badge-admin` badge to assign

**2. "NATS subscription failed"**
- Check NATS_URL is correct
- Verify NATS container is running
- Check network connectivity

**3. "Transaction rollback"**
- Check logs for specific table error
- Verify database schema is up to date
- Ensure no foreign key violations

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug cargo run
```

---

## Roadmap

### Completed
- ✅ Automated cleanup cron job
- ✅ Manual cleanup endpoints
- ✅ NATS event subscription
- ✅ Transaction-safe deletion
- ✅ Dry-run mode
- ✅ Admin authorization

### Planned
- ⏳ Prometheus metrics
- ⏳ Configurable cleanup schedule
- ⏳ Email notifications for admins
- ⏳ Cleanup history tracking
- ⏳ Integration tests for event flow

---

## References

- **Implementation Guide:** `docs/guides/development/FK-REMOVAL-IMPLEMENTATION.md`
- **NATS Events:** `services/shared-lib/src/events/`
- **Database Schema:** `services/shared-lib/migrations/`
- **Service Independence:** `docs/architecture/MIGRATIONS-MASTER.md`
