# Test & Seeding Scripts

Scripts for creating test data and seeding the Unity Platform database.

## Scripts

### create-test-user.sh

Create a test user directly in the database (bypasses invitation system).

**⚠️ DEVELOPMENT ONLY** - DO NOT use in production!

**Usage:**

```bash
./test/create-test-user.sh <territory> [email] [username] [password] [full_name]
```

**Example:**

```bash
./test/create-test-user.sh dk test@example.com testuser TestPass123! "Test User"
```

**Parameters:**

- `territory` - Territory code (dk, no, se, etc.)
- `email` - User email address
- `username` - Username (3-50 characters)
- `password` - Password (min 8 characters)
- `full_name` - Full name

---

### create-bootstrap-invitation.sh

Create bootstrap invitation token for territory managers.

**Usage:**

```bash
./test/create-bootstrap-invitation.sh <territory> <email> [days]
```

**Example:**

```bash
./test/create-bootstrap-invitation.sh dk admin@unityplatform.dk 365
```

**Parameters:**

- `territory` - Territory code (dk, no, se, etc.)
- `email` - Email address for invitation
- `days` - Validity period in days (default: 30)

---

### register-platform-badges.sh

Register platform-wide badges in the badge system.

**Usage:**

```bash
./test/register-platform-badges.sh
```

**Creates:**

- Platform service badges (Auth, User, Territory, Badge, etc.)
- Standard user achievement badges
- Community participation badges

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added

- Organized test and seeding scripts into `test/` subdirectory

#### Changed

- **BREAKING**: Scripts moved from `scripts/` root to `scripts/test/`
- Territory parameter now required for all user creation scripts

---

**Category**: Testing & Data Seeding  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
