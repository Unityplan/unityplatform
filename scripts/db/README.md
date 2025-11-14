# Database Scripts

Scripts for managing Unity Platform databases.

## Scripts

### setup-database.sh

Reset and run database migrations.

**Usage:**
```bash
./db/setup-database.sh
```

**Actions:**
- Drops and recreates `unityplatform_dk` database
- Runs all migrations from `services/shared-lib/migrations/`
- Uses PostgreSQL running in Docker

**Database Details:**
- Host: `localhost:5432`
- Database: `unityplatform_dk`
- User: `unityplatform`
- Password: `unityplatform_dev_password_dk`

**Manual Migration:**
```bash
cd services/shared-lib
DATABASE_URL="postgresql://unityplatform:unityplatform_dev_password_dk@localhost:5432/unityplatform_dk" \
  sqlx migrate run
```

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added
- Organized database scripts into `db/` subdirectory

#### Changed
- **BREAKING**: Scripts moved from `scripts/` root to `scripts/db/`
- Database name standardized to `unityplatform_dk` (from legacy `unityplan`)

---

**Category**: Database Management  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
