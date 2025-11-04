# Phase 1 MVP - Step-by-Step Implementation Checklist

## Overview
This document provides a detailed, dependency-ordered checklist for implementing Phase 1 MVP.
Each step must be completed before moving to the next dependent step.

---

## STAGE 1: Foundation & Infrastructure Setup

### Step 1.1: Repository & Project Structure
```
✅ Initialize Git repository
  └─ Create .gitignore for Rust, Node, Docker
  └─ Create README.md with project overview
  └─ Set up branch protection (main, develop)
  └─ Configure Git LFS for large files (if needed)

✅ Create workspace directory structure
  /workspace
    ├── services/          # Rust microservices
    ├── frontend/          # React/Vite app
    ├── docker/            # Docker configs
    ├── docs/              # Documentation
    ├── scripts/           # Utility scripts
    └── .github/           # GitHub workflows
```

### Step 1.2: Docker Infrastructure Setup
```
☐ Create docker-compose.yml (development)
  └─ PostgreSQL 16 service
     └─ Port: 5432
     └─ Volume: ./docker/postgres-data
     └─ Environment: POSTGRES_DB=unityplan_dev
     └─ Health check configured
  
  └─ TimescaleDB extension enabled
     └─ Init script: ./docker/postgres/init.sql
  
  └─ NATS service
     └─ Port: 4222 (client), 8222 (management)
     └─ Enable JetStream
     └─ Volume: ./docker/nats-data
  
  └─ Redis service (for caching)
     └─ Port: 6379
     └─ Volume: ./docker/redis-data
  
  └─ Adminer (database UI for development)
     └─ Port: 8080
     └─ Connected to PostgreSQL

☐ Create Docker network
  └─ Name: unityplan-network
  └─ Driver: bridge

☐ Test infrastructure
  └─ Run: docker-compose up -d
  └─ Verify PostgreSQL connection
  └─ Verify NATS connection
  └─ Verify Redis connection
  └─ Check Adminer access at localhost:8080

☐ Create database initialization scripts
  └─ docker/postgres/init.sql
     └─ Create TimescaleDB extension
     └─ Create initial schemas: public, global
     └─ Set up database roles
```

### Step 1.3: Rust Backend Foundation
```
☐ Create Rust workspace (services/Cargo.toml)
  [workspace]
  members = [
    "auth-service",
    "user-service",
    "territory-service",
    "badge-service",
    "course-service",
    "forum-service",
    "ipfs-service",
    "translation-service",
    "matrix-gateway"
  ]

☐ Create shared library crate (services/shared-lib)
  └─ Common types and utilities
  └─ Database connection pool management
  └─ NATS client wrapper
  └─ Error handling types
  └─ Configuration structs
  └─ Logging setup (tracing)
  
  Dependencies:
    - sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
    - tokio = { version = "1", features = ["full"] }
    - serde = { version = "1", features = ["derive"] }
    - tracing = "0.1"
    - tracing-subscriber = "0.3"
    - async-nats = "0.33"
    - config = "0.14"
    - dotenvy = "0.15"

☐ Create configuration system
  └─ services/shared-lib/src/config.rs
     └─ Database URL from env
     └─ NATS URL from env
     └─ Redis URL from env
     └─ Service-specific configs
     └─ Load from .env file
  
  └─ .env.example file
     DATABASE_URL=postgresql://user:pass@localhost:5432/unityplan_dev
     NATS_URL=nats://localhost:4222
     REDIS_URL=redis://localhost:6379
     LOG_LEVEL=debug

☐ Create database connection module
  └─ services/shared-lib/src/db.rs
     └─ PostgreSQL connection pool (SQLx)
     └─ Pool configuration (min: 5, max: 50)
     └─ Health check function
     └─ Migration runner setup
     └─ Schema switching capability (for multi-tenancy)

☐ Create NATS client module
  └─ services/shared-lib/src/nats.rs
     └─ Connect to NATS
     └─ Publish message helper
     └─ Subscribe to topic helper
     └─ JetStream integration
     └─ Error handling

☐ Create shared error types
  └─ services/shared-lib/src/error.rs
     └─ ServiceError enum
        - DatabaseError
        - NatsError
        - AuthError
        - ValidationError
        - NotFoundError
        - InternalError
     └─ Implement Display and Error traits
     └─ HTTP status code mapping

☐ Set up logging and tracing
  └─ services/shared-lib/src/observability.rs
     └─ Initialize tracing subscriber
     └─ JSON formatter for production
     └─ Pretty formatter for development
     └─ Filter by log level from env
     └─ Add service name to all logs
```

### Step 1.4: Frontend Foundation
```
☐ Initialize Vite + React project
  └─ cd frontend
  └─ npm create vite@latest . -- --template react-ts
  └─ Install dependencies:
     - React 19
     - TailwindCSS 4.1
     - @shadcn/ui 3.5
     - @tanstack/router 1.134
     - @tanstack/react-query (for API calls)
     - axios (HTTP client)
     - zustand (state management)

☐ Configure TailwindCSS
  └─ npx tailwindcss init -p
  └─ Configure tailwind.config.js
  └─ Add to src/index.css

☐ Set up ShadCN UI
  └─ npx shadcn-ui@latest init
  └─ Configure components.json
  └─ Install base components:
     - button
     - input
     - form
     - card
     - dialog
     - toast

☐ Configure TanStack Router
  └─ Create router configuration
  └─ Set up route tree
  └─ Create layout components
  └─ Configure route loaders

☐ Create API client setup
  └─ src/lib/api-client.ts
     └─ Axios instance with base URL
     └─ Request interceptor (add auth token)
     └─ Response interceptor (error handling)
     └─ Refresh token logic

☐ Create environment configuration
  └─ .env.development
     VITE_API_URL=http://localhost:8000
     VITE_WS_URL=ws://localhost:8000
  
  └─ .env.production
     VITE_API_URL=https://api.unityplan.org
     VITE_WS_URL=wss://api.unityplan.org

☐ Set up project structure
  /frontend/src
    ├── components/        # Reusable components
    ├── pages/            # Route pages
    ├── layouts/          # Layout components
    ├── lib/              # Utilities
    ├── hooks/            # Custom hooks
    ├── stores/           # Zustand stores
    ├── types/            # TypeScript types
    └── api/              # API client functions
```

---

## STAGE 2: Database Schema & Migrations

### Step 2.1: Set up SQLx Migrations
```
☐ Install SQLx CLI
  └─ cargo install sqlx-cli --no-default-features --features postgres

☐ Create migration directory
  └─ services/migrations/

☐ Initialize SQLx for each service
  └─ Create .sqlx directory for offline mode
  └─ Configure sqlx-data.json
```

### Step 2.2: Global Schema Migration
```
☐ Create migration: 001_create_global_schema.sql
  
  -- Global territory registry
  CREATE TABLE global.territories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    database_schema VARCHAR(63) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_territories_code ON global.territories(code);
  CREATE INDEX idx_territories_status ON global.territories(status);
  
  -- Global admin users (minimal)
  CREATE TABLE global.admins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'admin',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  -- Global settings
  CREATE TABLE global.settings (
    key VARCHAR(255) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );

☐ Run migration
  └─ sqlx migrate run --database-url $DATABASE_URL

☐ Verify migration
  └─ Check tables in Adminer
  └─ Test queries
```

### Step 2.3: Territory Schema Template
```
☐ Create migration: 002_create_territory_template.sql
  
  -- Function to create territory schema
  CREATE OR REPLACE FUNCTION create_territory_schema(
    territory_code VARCHAR(10)
  ) RETURNS VOID AS $$
  DECLARE
    schema_name VARCHAR(63);
  BEGIN
    schema_name := 'territory_' || territory_code;
    
    -- Create schema
    EXECUTE format('CREATE SCHEMA IF NOT EXISTS %I', schema_name);
    
    -- Set search path
    EXECUTE format('SET search_path TO %I', schema_name);
    
    -- Users table
    EXECUTE format('
      CREATE TABLE %I.users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email VARCHAR(255) UNIQUE NOT NULL,
        username VARCHAR(100) UNIQUE NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        full_name VARCHAR(255),
        avatar_url TEXT,
        bio TEXT,
        status VARCHAR(20) DEFAULT ''active'',
        email_verified BOOLEAN DEFAULT FALSE,
        privacy_settings JSONB DEFAULT ''{}''::jsonb,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        last_login_at TIMESTAMPTZ
      )', schema_name);
    
    -- More tables will be added in next steps...
    
  END;
  $$ LANGUAGE plpgsql;

☐ Create function to create test territory
  └─ INSERT INTO global.territories (code, name, database_schema)
      VALUES ('TEST', 'Test Territory', 'territory_test');
  └─ SELECT create_territory_schema('TEST');

☐ Verify territory schema created
  └─ Check schema exists in database
  └─ Verify users table structure
```

---

**Dependencies Completed:**
- ✅ Docker infrastructure running
- ✅ Rust workspace initialized
- ✅ Shared library created
- ✅ Frontend foundation setup
- ✅ Database migrations framework ready
- ✅ Global schema created
- ✅ Territory schema template created

**Next Stage:** Authentication Service Implementation
## STAGE 3: Authentication Service

### Step 3.1: Auth Service Scaffolding
```
☐ Create auth-service crate
  └─ cargo new services/auth-service --bin
  
  └─ Update Cargo.toml dependencies:
     shared-lib = { path = "../shared-lib" }
     actix-web = "4"
     actix-cors = "0.7"
     tokio = { version = "1", features = ["full"] }
     sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "uuid"] }
     serde = { version = "1", features = ["derive"] }
     serde_json = "1"
     jsonwebtoken = "9"
     bcrypt = "0.15"
     uuid = { version = "1", features = ["v4", "serde"] }
     chrono = { version = "0.4", features = ["serde"] }
     validator = { version = "0.18", features = ["derive"] }
     openidconnect = "3"
     tracing = "0.1"
     tracing-actix-web = "0.7"

☐ Create service structure
  /services/auth-service/src
    ├── main.rs              # Entry point
    ├── config.rs            # Service config
    ├── handlers/            # HTTP handlers
    │   ├── mod.rs
    │   ├── auth.rs          # Login, logout, refresh
    │   └── oidc.rs          # OpenID Connect
    ├── models/              # Data models
    │   ├── mod.rs
    │   ├── user.rs
    │   └── token.rs
    ├── services/            # Business logic
    │   ├── mod.rs
    │   ├── auth_service.rs
    │   └── token_service.rs
    └── middleware/          # Auth middleware
        ├── mod.rs
        └── jwt.rs

☐ Implement main.rs
  - Initialize tracing/logging
  - Load configuration
  - Create database pool
  - Create NATS client
  - Configure Actix server
  - Register routes
  - Health check endpoint
  - Graceful shutdown
```

### Step 3.2: Auth Database Schema
```
☐ Add auth tables to territory schema template
  
  -- Refresh tokens table
  CREATE TABLE {schema}.refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    user_agent TEXT,
    ip_address INET
  );
  
  CREATE INDEX idx_refresh_tokens_user ON {schema}.refresh_tokens(user_id);
  CREATE INDEX idx_refresh_tokens_expires ON {schema}.refresh_tokens(expires_at);
  
  -- Login attempts (for rate limiting)
  CREATE TABLE {schema}.login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    ip_address INET NOT NULL,
    successful BOOLEAN NOT NULL,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_login_attempts_email ON {schema}.login_attempts(email, attempted_at);
  CREATE INDEX idx_login_attempts_ip ON {schema}.login_attempts(ip_address, attempted_at);
  
  -- Password reset tokens
  CREATE TABLE {schema}.password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_password_reset_user ON {schema}.password_reset_tokens(user_id);

☐ Run migration
  └─ sqlx migrate run

☐ Update territory creation function to include auth tables
```

### Step 3.3: JWT Token Service
```
☐ Implement token service (services/token_service.rs)
  
  pub struct TokenService {
      jwt_secret: String,
      access_token_ttl: i64,  // 15 minutes
      refresh_token_ttl: i64, // 7 days
  }
  
  Functions to implement:
  ☐ generate_access_token(user_id, territory_code, roles)
    └─ Create JWT with claims
    └─ Set expiration
    └─ Sign with secret
  
  ☐ generate_refresh_token()
    └─ Generate random secure token
    └─ Hash token for storage
    └─ Return (token, hash)
  
  ☐ verify_access_token(token)
    └─ Decode JWT
    └─ Verify signature
    └─ Check expiration
    └─ Extract claims
  
  ☐ verify_refresh_token(token, user_id, db)
    └─ Hash provided token
    └─ Query database
    └─ Check expiration
    └─ Check not revoked
  
  ☐ revoke_refresh_token(token_id, db)
    └─ Update revoked_at timestamp

☐ Create JWT claims structure
  #[derive(Serialize, Deserialize)]
  pub struct JwtClaims {
      pub sub: String,          // user_id
      pub territory: String,    // territory_code
      pub roles: Vec<String>,   // user roles
      pub exp: i64,             // expiration
      pub iat: i64,             // issued at
      pub jti: String,          // JWT ID
  }
```

### Step 3.4: Auth Handlers Implementation
```
☐ POST /auth/register - User registration
  Request body:
  {
    "email": "user@example.com",
    "username": "johndoe",
    "password": "SecurePass123!",
    "full_name": "John Doe",
    "territory_code": "TEST"
  }
  
  Implementation steps:
  └─ Validate request data (email format, password strength)
  └─ Check territory exists and is active
  └─ Switch to territory schema
  └─ Check email/username not already exists
  └─ Hash password with bcrypt (cost: 12)
  └─ Insert user into database
  └─ Generate access & refresh tokens
  └─ Store refresh token hash in database
  └─ Return tokens + user info
  
  Response:
  {
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "username": "johndoe",
      "full_name": "John Doe"
    },
    "access_token": "eyJ...",
    "refresh_token": "random_string",
    "expires_in": 900
  }

☐ POST /auth/login - User login
  Request body:
  {
    "email": "user@example.com",
    "password": "SecurePass123!",
    "territory_code": "TEST"
  }
  
  Implementation steps:
  └─ Validate request data
  └─ Check territory exists
  └─ Check rate limiting (max 5 attempts per 15 min)
  └─ Switch to territory schema
  └─ Query user by email
  └─ Verify password with bcrypt
  └─ Log login attempt (success/fail)
  └─ If successful:
     └─ Update last_login_at
     └─ Generate tokens
     └─ Store refresh token
     └─ Return tokens + user info
  └─ If failed:
     └─ Return generic error (security)

☐ POST /auth/refresh - Refresh access token
  Request body:
  {
    "refresh_token": "random_string",
    "territory_code": "TEST"
  }
  
  Implementation:
  └─ Validate refresh token
  └─ Get user from database
  └─ Generate new access token
  └─ Optionally rotate refresh token
  └─ Return new tokens

☐ POST /auth/logout - Logout user
  Request headers:
  Authorization: Bearer {access_token}
  
  Request body:
  {
    "refresh_token": "random_string"
  }
  
  Implementation:
  └─ Verify access token
  └─ Revoke refresh token
  └─ Return success

☐ GET /auth/me - Get current user info
  Request headers:
  Authorization: Bearer {access_token}
  
  Implementation:
  └─ Extract user_id from JWT
  └─ Query user from database
  └─ Return user info (exclude password_hash)
```

### Step 3.5: JWT Middleware
```
☐ Implement JWT authentication middleware
  └─ middleware/jwt.rs
  
  Functions:
  ☐ extract_bearer_token(request)
    └─ Get Authorization header
    └─ Extract token from "Bearer {token}"
  
  ☐ authenticate(request) -> Result<JwtClaims>
    └─ Extract token
    └─ Verify token signature
    └─ Check expiration
    └─ Extract claims
    └─ Add claims to request extensions
  
  ☐ require_auth() - Middleware wrapper
    └─ Call authenticate
    └─ If valid: proceed
    └─ If invalid: return 401 Unauthorized

☐ Create optional auth middleware
  └─ Attempts authentication but doesn't fail if missing
  └─ Useful for public endpoints with optional user context
```

### Step 3.6: Auth Service Testing
```
☐ Unit tests
  └─ Token generation and verification
  └─ Password hashing and verification
  └─ JWT claims extraction
  └─ Refresh token validation

☐ Integration tests
  └─ Register new user
     └─ Valid data
     └─ Duplicate email
     └─ Invalid email format
     └─ Weak password
  
  └─ Login
     └─ Valid credentials
     └─ Invalid password
     └─ Non-existent user
     └─ Rate limiting
  
  └─ Refresh token
     └─ Valid token
     └─ Expired token
     └─ Revoked token
  
  └─ Protected endpoints
     └─ Valid JWT
     └─ Expired JWT
     └─ Invalid JWT
     └─ Missing JWT

☐ Load testing
  └─ Login endpoint: 100 req/s
  └─ Refresh endpoint: 50 req/s
  └─ Verify performance targets

☐ Manual testing with curl/Postman
  └─ Create test scripts
  └─ Document API usage
```

---

**Dependencies Completed:**
- ✅ Auth service scaffold created
- ✅ Database schema for auth
- ✅ JWT token service implemented
- ✅ Auth handlers implemented
- ✅ JWT middleware created
- ✅ Tests passing

**Next Stage:** User Service Implementation
## STAGE 4: User Service

### Step 4.1: User Service Scaffolding
```
☐ Create user-service crate
  └─ cargo new services/user-service --bin
  
  └─ Dependencies (similar to auth-service plus):
     actix-multipart = "0.6"  # For file uploads
     image = "0.24"           # Image processing
     reqwest = "0.11"         # HTTP client for external calls

☐ Create service structure
  /services/user-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── profile.rs       # Profile CRUD
    │   ├── avatar.rs        # Avatar upload
    │   └── privacy.rs       # Privacy settings
    ├── models/
    │   ├── mod.rs
    │   ├── user.rs
    │   └── privacy.rs
    └── services/
        ├── mod.rs
        ├── user_service.rs
        └── storage_service.rs  # For avatar storage
```

### Step 4.2: User Database Schema Extensions
```
☐ Add user profile tables to territory schema
  
  -- User profiles (extended data)
  CREATE TABLE {schema}.user_profiles (
    user_id UUID PRIMARY KEY REFERENCES {schema}.users(id) ON DELETE CASCADE,
    date_of_birth DATE,
    country VARCHAR(2),
    city VARCHAR(100),
    timezone VARCHAR(50),
    language_preference VARCHAR(10) DEFAULT 'en',
    notification_preferences JSONB DEFAULT '{}'::jsonb,
    theme_preference VARCHAR(20) DEFAULT 'system',
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  -- User connections (following/followers)
  CREATE TABLE {schema}.user_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, following_id),
    CHECK(follower_id != following_id)
  );
  
  CREATE INDEX idx_user_connections_follower ON {schema}.user_connections(follower_id);
  CREATE INDEX idx_user_connections_following ON {schema}.user_connections(following_id);
  
  -- User blocking
  CREATE TABLE {schema}.user_blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    blocker_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    blocked_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(blocker_id, blocked_id)
  );
  
  CREATE INDEX idx_user_blocks_blocker ON {schema}.user_blocks(blocker_id);

☐ Run migration
  └─ sqlx migrate run
```

### Step 4.3: User Profile Handlers
```
☐ GET /users/me - Get current user's full profile
  Headers: Authorization: Bearer {token}
  
  Response:
  {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "full_name": "John Doe",
    "avatar_url": "https://...",
    "bio": "Software developer",
    "profile": {
      "country": "DK",
      "city": "Copenhagen",
      "timezone": "Europe/Copenhagen",
      "language_preference": "da"
    },
    "privacy_settings": {
      "profile_visibility": "public",
      "email_visibility": "private",
      "show_online_status": true
    },
    "stats": {
      "followers": 42,
      "following": 18,
      "courses_completed": 5,
      "badges_earned": 3
    }
  }
  
  Implementation:
  └─ Extract user_id from JWT
  └─ Query user + profile (LEFT JOIN)
  └─ Count followers/following
  └─ Get badge/course stats
  └─ Return combined data

☐ GET /users/{user_id} - Get another user's public profile
  Headers: Authorization: Bearer {token} (optional)
  
  Implementation:
  └─ Query user by ID
  └─ Check privacy settings
  └─ If requester is blocked, return 404
  └─ Return only public fields based on privacy settings
  
☐ PUT /users/me - Update current user's profile
  Headers: Authorization: Bearer {token}
  
  Request body:
  {
    "full_name": "John Smith",
    "bio": "Updated bio",
    "profile": {
      "city": "Aarhus",
      "timezone": "Europe/Copenhagen"
    }
  }
  
  Implementation:
  └─ Extract user_id from JWT
  └─ Validate input data
  └─ Update users table
  └─ Upsert user_profiles table
  └─ Update updated_at timestamp
  └─ Return updated profile

☐ DELETE /users/me - Delete account
  Headers: Authorization: Bearer {token}
  
  Request body:
  {
    "password": "current_password",
    "confirmation": "DELETE MY ACCOUNT"
  }
  
  Implementation:
  └─ Verify password
  └─ Check confirmation text
  └─ Soft delete (set status = 'deleted')
  └─ Anonymize personal data (GDPR)
  └─ Keep content but mark as [deleted user]
  └─ Revoke all tokens
  └─ Publish user_deleted event to NATS
```

### Step 4.4: Avatar Upload Handler
```
☐ POST /users/me/avatar - Upload avatar
  Headers: 
    Authorization: Bearer {token}
    Content-Type: multipart/form-data
  
  Request: Form with 'avatar' file field
  
  Implementation:
  └─ Validate file type (JPEG, PNG, WebP)
  └─ Validate file size (<5MB)
  └─ Generate unique filename
  └─ Resize image to 512x512 (square)
  └─ Save to local storage (for now)
     Path: ./uploads/avatars/{user_id}/{filename}
  └─ Update user.avatar_url in database
  └─ Return new avatar URL
  
  Response:
  {
    "avatar_url": "/avatars/{user_id}/{filename}"
  }

☐ DELETE /users/me/avatar - Remove avatar
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Delete file from storage
  └─ Set avatar_url to NULL
  └─ Return success

☐ GET /avatars/{user_id}/{filename} - Serve avatar file
  Implementation:
  └─ Validate path (prevent directory traversal)
  └─ Return file with proper content-type
  └─ Add caching headers (max-age: 1 day)
```

### Step 4.5: Privacy Settings Handler
```
☐ PUT /users/me/privacy - Update privacy settings
  Headers: Authorization: Bearer {token}
  
  Request body:
  {
    "profile_visibility": "public",      // public, connections, private
    "email_visibility": "private",       // public, connections, private
    "show_online_status": true,
    "show_last_seen": false,
    "allow_messages_from": "everyone",   // everyone, connections, nobody
    "show_course_progress": true
  }
  
  Implementation:
  └─ Validate privacy options
  └─ Update user.privacy_settings JSONB
  └─ Return updated settings
```

### Step 4.6: User Connections Handlers
```
☐ POST /users/{user_id}/follow - Follow user
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Extract follower_id from JWT
  └─ Validate user_id exists
  └─ Check not blocked
  └─ Insert into user_connections
  └─ Publish user_followed event to NATS
  └─ Return success

☐ DELETE /users/{user_id}/follow - Unfollow user
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Delete from user_connections
  └─ Return success

☐ GET /users/{user_id}/followers - Get followers list
  Query params: ?page=1&limit=20
  
  Implementation:
  └─ Check privacy settings
  └─ Query followers with pagination
  └─ Return user list

☐ GET /users/{user_id}/following - Get following list
  Implementation similar to followers

☐ POST /users/{user_id}/block - Block user
  Headers: Authorization: Bearer {token}
  
  Request body:
  {
    "reason": "Harassment"
  }
  
  Implementation:
  └─ Insert into user_blocks
  └─ Remove any connections
  └─ Return success

☐ DELETE /users/{user_id}/block - Unblock user

☐ GET /users/me/blocks - Get blocked users list
```

### Step 4.7: User Search Handler
```
☐ GET /users/search - Search users
  Query params:
    ?query=john
    &page=1
    &limit=20
    &territory=TEST  (optional)
  
  Implementation:
  └─ Validate query (min 2 chars)
  └─ Search by username or full_name (ILIKE)
  └─ Only return users with public profiles
  └─ Exclude blocked users
  └─ Paginate results
  └─ Return user list with minimal info
  
  Response:
  {
    "users": [
      {
        "id": "uuid",
        "username": "johndoe",
        "full_name": "John Doe",
        "avatar_url": "https://...",
        "bio": "Developer"
      }
    ],
    "total": 142,
    "page": 1,
    "pages": 8
  }
```

### Step 4.8: User Service Testing
```
☐ Unit tests
  └─ Profile data validation
  └─ Privacy settings validation
  └─ Image processing
  └─ Search query building

☐ Integration tests
  └─ Create and update profile
  └─ Upload avatar
     └─ Valid image
     └─ Invalid format
     └─ File too large
  └─ Privacy settings
  └─ Follow/unfollow users
  └─ Block/unblock users
  └─ Search users
  └─ Profile visibility based on privacy

☐ Manual testing
  └─ Test with various image formats
  └─ Test privacy settings combinations
  └─ Test search functionality
```

---

## STAGE 5: Frontend - Authentication & Profile

### Step 5.1: Auth Store (Zustand)
```
☐ Create auth store (src/stores/auth-store.ts)
  
  interface AuthState {
    user: User | null;
    accessToken: string | null;
    refreshToken: string | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    
    // Actions
    login: (credentials) => Promise<void>;
    register: (data) => Promise<void>;
    logout: () => Promise<void>;
    refreshAccessToken: () => Promise<void>;
    loadUser: () => Promise<void>;
  }
  
  Implementation:
  └─ Store tokens in localStorage
  └─ Load tokens on app init
  └─ Auto-refresh access token before expiry
  └─ Clear state on logout
```

### Step 5.2: API Client Functions
```
☐ Create auth API client (src/api/auth.ts)
  
  Functions:
  ☐ register(data: RegisterRequest): Promise<AuthResponse>
  ☐ login(credentials: LoginRequest): Promise<AuthResponse>
  ☐ logout(refreshToken: string): Promise<void>
  ☐ refreshToken(token: string): Promise<AuthResponse>
  ☐ getCurrentUser(): Promise<User>

☐ Create user API client (src/api/users.ts)
  
  Functions:
  ☐ getMyProfile(): Promise<UserProfile>
  ☐ getUser(userId: string): Promise<UserProfile>
  ☐ updateProfile(data: UpdateProfileRequest): Promise<UserProfile>
  ☐ uploadAvatar(file: File): Promise<{avatar_url: string}>
  ☐ updatePrivacy(settings: PrivacySettings): Promise<void>
  ☐ followUser(userId: string): Promise<void>
  ☐ unfollowUser(userId: string): Promise<void>
  ☐ blockUser(userId: string, reason: string): Promise<void>
  ☐ searchUsers(query: string, page: number): Promise<SearchResults>
```

### Step 5.3: Auth Pages
```
☐ Create login page (src/pages/auth/login.tsx)
  
  Layout:
  - Logo and branding
  - Email input
  - Password input
  - "Remember me" checkbox
  - Territory selector (dropdown)
  - Login button
  - "Forgot password?" link
  - "Don't have an account? Register" link
  
  Implementation:
  └─ Form validation (react-hook-form + zod)
  └─ Show loading state during login
  └─ Handle errors (show toast)
  └─ Redirect to dashboard on success
  └─ Store territory selection

☐ Create register page (src/pages/auth/register.tsx)
  
  Layout:
  - Email input (with validation)
  - Username input (check availability)
  - Password input (with strength meter)
  - Confirm password input
  - Full name input
  - Territory selector
  - Terms of service checkbox
  - Register button
  - "Already have account? Login" link
  
  Implementation:
  └─ Real-time validation
  └─ Password strength indicator
  └─ Check username availability (debounced)
  └─ Agree to terms validation
  └─ Handle registration errors
  └─ Auto-login after registration
  └─ Redirect to onboarding flow

☐ Create password reset page
  └─ Request reset token
  └─ Reset password with token
```

### Step 5.4: Profile Pages
```
☐ Create profile view page (src/pages/profile/[userId].tsx)
  
  Layout:
  - Avatar (large)
  - Username and full name
  - Bio
  - Stats (followers, following, badges, courses)
  - Follow/Unfollow button (if not own profile)
  - Edit profile button (if own profile)
  - Tabs:
    * About (profile info)
    * Courses (completed/in-progress)
    * Badges
    * Activity
  
  Implementation:
  └─ Load user profile
  └─ Handle follow/unfollow
  └─ Respect privacy settings
  └─ Show appropriate data based on relationship

☐ Create profile edit page (src/pages/profile/edit.tsx)
  
  Sections:
  - Avatar upload (with preview)
  - Basic info (name, bio, username)
  - Location (country, city, timezone)
  - Language preference
  - Privacy settings (toggle switches)
  - Account actions (delete account)
  
  Implementation:
  └─ Load current profile data
  └─ Form with sections
  └─ Avatar upload with preview
  └─ Auto-save or manual save
  └─ Confirm before account deletion
```

### Step 5.5: Protected Routes
```
☐ Create route guard (src/components/ProtectedRoute.tsx)
  
  Implementation:
  └─ Check if user is authenticated
  └─ If not, redirect to login
  └─ Show loading state while checking
  └─ Store intended destination for redirect after login

☐ Configure router with protected routes
  - /dashboard (protected)
  - /profile/* (protected)
  - /courses/* (protected)
  - /auth/* (public only - redirect if authenticated)
```

### Step 5.6: UI Components
```
☐ Create avatar component (src/components/Avatar.tsx)
  └─ Show user avatar or fallback initials
  └─ Different sizes (sm, md, lg, xl)
  └─ Loading state
  └─ Online status indicator (optional)

☐ Create user card component
  └─ Mini profile card for lists
  └─ Avatar + name + bio snippet
  └─ Follow button

☐ Create profile header component
  └─ Reusable profile header
  └─ Cover photo (future)
  └─ Avatar, name, stats

☐ Create privacy settings form
  └─ Privacy options with descriptions
  └─ Toggle switches
  └─ Save changes
```

### Step 5.7: Frontend Testing
```
☐ Unit tests for components
  └─ Avatar component
  └─ User card component
  └─ Form validation

☐ Integration tests
  └─ Login flow
  └─ Registration flow
  └─ Profile update flow
  └─ Avatar upload

☐ E2E tests (Playwright/Cypress)
  └─ Complete registration → login → profile edit flow
  └─ Follow/unfollow user
  └─ Privacy settings changes
```

---

**Dependencies Completed:**
- ✅ User service implemented
- ✅ Frontend auth store created
- ✅ Auth pages created
- ✅ Profile pages created
- ✅ Protected routes configured
- ✅ Basic UI components created
- ✅ Integration between frontend and backend working

**Next Stage:** Territory Service & Badge System
## STAGE 6: Territory Service & Badge System

### Step 6.1: Territory Service Scaffolding
```
☐ Create territory-service crate
  └─ cargo new services/territory-service --bin
  
  └─ Standard dependencies (actix-web, sqlx, etc.)

☐ Create service structure
  /services/territory-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── territory.rs
    │   └── admin.rs
    ├── models/
    │   ├── mod.rs
    │   └── territory.rs
    └── services/
        ├── mod.rs
        └── territory_service.rs
```

### Step 6.2: Territory Handlers
```
☐ GET /territories - List all active territories
  Response:
  {
    "territories": [
      {
        "code": "DK",
        "name": "Denmark",
        "status": "active",
        "stats": {
          "users": 142,
          "courses": 23,
          "communities": 8
        }
      }
    ]
  }

☐ GET /territories/{code} - Get territory details
  Implementation:
  └─ Query from global.territories
  └─ Get statistics from territory schema
  └─ Return territory info

☐ POST /territories - Create new territory (admin only)
  Request:
  {
    "code": "NO",
    "name": "Norway",
    "settings": {
      "default_language": "no",
      "timezone": "Europe/Oslo"
    }
  }
  
  Implementation:
  └─ Validate admin permissions
  └─ Check code uniqueness
  └─ Insert into global.territories
  └─ Call create_territory_schema(code)
  └─ Publish territory_created event
  └─ Return territory info
```

### Step 6.3: Badge Service Scaffolding
```
☐ Create badge-service crate
  └─ cargo new services/badge-service --bin

☐ Create service structure
  /services/badge-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── badges.rs
    │   └── awards.rs
    ├── models/
    │   ├── mod.rs
    │   ├── badge.rs
    │   └── badge_award.rs
    └── services/
        ├── mod.rs
        └── badge_service.rs
```

### Step 6.4: Badge Database Schema
```
☐ Add badge tables to territory schema
  
  -- Badge definitions (templates)
  CREATE TABLE {schema}.badge_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,  -- code_of_conduct, achievement, role
    icon_url TEXT,
    color VARCHAR(7),  -- Hex color
    criteria JSONB NOT NULL,  -- Requirements to earn
    required_for_permissions JSONB DEFAULT '[]'::jsonb,
    expires_after_days INT,  -- NULL = never expires
    is_active BOOLEAN DEFAULT TRUE,
    created_by UUID REFERENCES {schema}.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_badges_category ON {schema}.badge_definitions(category);
  CREATE INDEX idx_badges_code ON {schema}.badge_definitions(code);
  
  -- Badge awards (user-specific)
  CREATE TABLE {schema}.badge_awards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    badge_id UUID NOT NULL REFERENCES {schema}.badge_definitions(id),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    awarded_by UUID REFERENCES {schema}.users(id),
    awarded_reason TEXT,
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,  -- Calculated from badge.expires_after_days
    revoked_at TIMESTAMPTZ,
    revoked_by UUID REFERENCES {schema}.users(id),
    revoked_reason TEXT,
    metadata JSONB DEFAULT '{}'::jsonb,
    UNIQUE(badge_id, user_id, awarded_at)
  );
  
  CREATE INDEX idx_badge_awards_user ON {schema}.badge_awards(user_id);
  CREATE INDEX idx_badge_awards_badge ON {schema}.badge_awards(badge_id);
  CREATE INDEX idx_badge_awards_expires ON {schema}.badge_awards(expires_at);
  
  -- Badge progression (for badges requiring multiple steps)
  CREATE TABLE {schema}.badge_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    badge_id UUID NOT NULL REFERENCES {schema}.badge_definitions(id),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    progress JSONB NOT NULL,  -- Track completion of criteria
    current_step INT DEFAULT 0,
    total_steps INT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(badge_id, user_id)
  );
  
  CREATE INDEX idx_badge_progress_user ON {schema}.badge_progress(user_id);

☐ Run migration
```

### Step 6.5: Seed Code of Conduct Badge
```
☐ Create seed script for essential badges
  
  INSERT INTO territory_test.badge_definitions 
  (code, name, description, category, criteria, required_for_permissions)
  VALUES
  (
    'code_of_conduct',
    'Code of Conduct',
    'Completed the Code of Conduct training and agreed to community guidelines',
    'code_of_conduct',
    '{
      "type": "course_completion",
      "course_code": "code_of_conduct_training",
      "min_score": 80
    }'::jsonb,
    '["create_post", "create_topic", "comment", "vote"]'::jsonb
  );

☐ Create function to check badge expiration
  └─ Scheduled job (every hour)
  └─ Query expired badges
  └─ Revoke permissions
  └─ Send notifications to users
```

### Step 6.6: Badge Handlers Implementation
```
☐ GET /badges - List all available badges
  Query params: ?category=code_of_conduct
  
  Response:
  {
    "badges": [
      {
        "id": "uuid",
        "code": "code_of_conduct",
        "name": "Code of Conduct",
        "description": "...",
        "category": "code_of_conduct",
        "icon_url": "/badges/code_of_conduct.svg",
        "criteria": {...}
      }
    ]
  }

☐ GET /badges/{badge_id} - Get badge details
  └─ Return badge definition
  └─ Include earn statistics

☐ GET /users/{user_id}/badges - Get user's badges
  Implementation:
  └─ Query badge_awards for user
  └─ Join with badge_definitions
  └─ Filter out expired/revoked badges
  └─ Return active badges only (unless requesting own)
  
  Response:
  {
    "badges": [
      {
        "badge": {...},
        "awarded_at": "2025-01-15T10:00:00Z",
        "expires_at": "2025-07-15T10:00:00Z",
        "days_until_expiry": 42
      }
    ]
  }

☐ POST /badges/award - Award badge to user (system/admin)
  Request:
  {
    "badge_code": "code_of_conduct",
    "user_id": "uuid",
    "reason": "Completed training"
  }
  
  Implementation:
  └─ Verify awarding permissions
  └─ Get badge definition
  └─ Calculate expiration date
  └─ Insert badge_award
  └─ Publish badge_awarded event to NATS
  └─ Send notification to user
  └─ Return award details

☐ POST /badges/revoke - Revoke badge (admin/system)
  Request:
  {
    "award_id": "uuid",
    "reason": "Code of conduct violation"
  }
  
  Implementation:
  └─ Verify permissions
  └─ Update badge_award (set revoked_at)
  └─ Remove associated permissions
  └─ Publish badge_revoked event
  └─ Send notification to user

☐ GET /users/me/badge-progress - Get current badge progress
  Response:
  {
    "progress": [
      {
        "badge": {...},
        "current_step": 2,
        "total_steps": 5,
        "progress": {
          "lessons_completed": 2,
          "quiz_score": 75
        },
        "percent_complete": 40
      }
    ]
  }
```

### Step 6.7: Permission Checking System
```
☐ Create permission checker (shared-lib)
  
  pub async fn check_user_permission(
      user_id: Uuid,
      permission: &str,
      territory_code: &str,
      db: &PgPool
  ) -> Result<bool> {
      // Query user's active badges
      // Check if any badge grants this permission
      // Check expiration
      // Return true/false
  }

☐ Create middleware for permission enforcement
  └─ require_permission("create_post")
  └─ Check user has required badge
  └─ Return 403 if missing permission

☐ Integrate with forum service (later)
  └─ Check "create_topic" permission
  └─ Check "create_post" permission
  └─ Check "comment" permission
```

### Step 6.8: Badge Event Handlers (NATS)
```
☐ Subscribe to course completion events
  Topic: course.completed.{user_id}
  
  Handler:
  └─ Check if completion earns any badges
  └─ Award eligible badges automatically
  └─ Update badge progress

☐ Subscribe to violation events
  Topic: moderation.violation.{user_id}
  
  Handler:
  └─ Check strike count
  └─ If 3 strikes: revoke Code of Conduct badge
  └─ Remove forum permissions

☐ Publish badge events
  └─ badge.awarded.{user_id}
  └─ badge.revoked.{user_id}
  └─ badge.expiring.{user_id} (7 days before)
```

### Step 6.9: Testing Badge System
```
☐ Unit tests
  └─ Permission checking logic
  └─ Expiration calculation
  └─ Badge criteria validation

☐ Integration tests
  └─ Award badge
  └─ Revoke badge
  └─ Check permissions with/without badge
  └─ Auto-award on course completion
  └─ Badge expiration handling

☐ E2E scenarios
  └─ User completes Code of Conduct course
  └─ Badge automatically awarded
  └─ User can now create forum posts
  └─ Badge expires after 180 days
  └─ User loses forum permissions
  └─ User retakes course
  └─ Badge renewed
```

---

## STAGE 7: Course Service (LMS)

### Step 7.1: Course Service Scaffolding
```
☐ Create course-service crate
  └─ cargo new services/course-service --bin

☐ Create service structure
  /services/course-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── courses.rs
    │   ├── lessons.rs
    │   ├── enrollments.rs
    │   └── quizzes.rs
    ├── models/
    │   ├── mod.rs
    │   ├── course.rs
    │   ├── lesson.rs
    │   └── enrollment.rs
    └── services/
        ├── mod.rs
        └── course_service.rs
```

### Step 7.2: Course Database Schema
```
☐ Add course tables to territory schema
  
  -- Courses
  CREATE TABLE {schema}.courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    difficulty_level VARCHAR(20), -- beginner, intermediate, advanced
    estimated_duration_minutes INT,
    thumbnail_url TEXT,
    status VARCHAR(20) DEFAULT 'draft', -- draft, published, archived
    created_by UUID NOT NULL REFERENCES {schema}.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ
  );
  
  CREATE INDEX idx_courses_status ON {schema}.courses(status);
  CREATE INDEX idx_courses_category ON {schema}.courses(category);
  
  -- Lessons (within courses)
  CREATE TABLE {schema}.lessons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES {schema}.courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    content_type VARCHAR(20), -- video, text, quiz, interactive
    content_url TEXT,  -- IPFS hash or local path
    ipfs_cid VARCHAR(100),  -- Content identifier
    duration_minutes INT,
    order_index INT NOT NULL,
    is_required BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_lessons_course ON {schema}.lessons(course_id, order_index);
  
  -- Course enrollments
  CREATE TABLE {schema}.course_enrollments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES {schema}.courses(id),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    enrolled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    completion_percentage INT DEFAULT 0,
    last_accessed_at TIMESTAMPTZ,
    status VARCHAR(20) DEFAULT 'active', -- active, completed, dropped
    UNIQUE(course_id, user_id)
  );
  
  CREATE INDEX idx_enrollments_user ON {schema}.course_enrollments(user_id);
  CREATE INDEX idx_enrollments_course ON {schema}.course_enrollments(course_id);
  
  -- Lesson progress
  CREATE TABLE {schema}.lesson_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    enrollment_id UUID NOT NULL REFERENCES {schema}.course_enrollments(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL REFERENCES {schema}.lessons(id),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    time_spent_seconds INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'not_started',
    UNIQUE(enrollment_id, lesson_id)
  );
  
  CREATE INDEX idx_lesson_progress_enrollment ON {schema}.lesson_progress(enrollment_id);
  
  -- Quizzes
  CREATE TABLE {schema}.quizzes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lesson_id UUID REFERENCES {schema}.lessons(id) ON DELETE CASCADE,
    course_id UUID REFERENCES {schema}.courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    passing_score INT DEFAULT 80,
    max_attempts INT,
    time_limit_minutes INT,
    questions JSONB NOT NULL, -- Array of questions
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  -- Quiz attempts
  CREATE TABLE {schema}.quiz_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id UUID NOT NULL REFERENCES {schema}.quizzes(id),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    answers JSONB NOT NULL,
    score INT NOT NULL,
    passed BOOLEAN NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    time_spent_seconds INT
  );
  
  CREATE INDEX idx_quiz_attempts_user_quiz ON {schema}.quiz_attempts(user_id, quiz_id);

☐ Run migration
```

### Step 7.3: Seed Code of Conduct Course
```
☐ Create Code of Conduct training course
  
  INSERT INTO territory_test.courses 
  (code, title, description, category, difficulty_level, estimated_duration_minutes, status, created_by)
  VALUES (
    'code_of_conduct_training',
    'Community Code of Conduct',
    'Learn about our community guidelines and expectations',
    'onboarding',
    'beginner',
    30,
    'published',
    (SELECT id FROM territory_test.users LIMIT 1)
  );
  
☐ Create lessons for Code of Conduct
  - Lesson 1: Welcome & Introduction (5 min)
  - Lesson 2: Respectful Communication (10 min)
  - Lesson 3: Content Guidelines (10 min)
  - Lesson 4: Reporting & Moderation (5 min)
  - Lesson 5: Quiz (pass required)

☐ Create quiz questions
  └─ Multiple choice questions
  └─ True/false questions
  └─ Require 80% to pass
  └─ Max 3 attempts
```

### Step 7.4: Course Handlers
```
☐ GET /courses - List published courses
  Query params: ?category=onboarding&page=1&limit=20
  
  Response:
  {
    "courses": [
      {
        "id": "uuid",
        "code": "code_of_conduct_training",
        "title": "Community Code of Conduct",
        "description": "...",
        "category": "onboarding",
        "difficulty_level": "beginner",
        "estimated_duration_minutes": 30,
        "thumbnail_url": "...",
        "lessons_count": 5,
        "enrolled_count": 142
      }
    ],
    "total": 23,
    "page": 1
  }

☐ GET /courses/{course_id} - Get course details
  Implementation:
  └─ Get course info
  └─ Get lessons (if enrolled or creator)
  └─ Get user's enrollment status
  └─ Get progress if enrolled
  
  Response:
  {
    "course": {...},
    "lessons": [...],
    "enrollment": {
      "enrolled": true,
      "progress": 60,
      "completed_lessons": 3,
      "total_lessons": 5
    }
  }

☐ POST /courses/{course_id}/enroll - Enroll in course
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Check not already enrolled
  └─ Create enrollment record
  └─ Publish course.enrolled event
  └─ Return enrollment info

☐ GET /courses/{course_id}/lessons/{lesson_id} - Get lesson content
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Verify enrollment
  └─ Get lesson content
  └─ Track access (update last_accessed_at)
  └─ Return lesson data

☐ POST /courses/{course_id}/lessons/{lesson_id}/complete - Mark lesson complete
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Verify enrollment
  └─ Update lesson_progress (set completed_at)
  └─ Recalculate course completion percentage
  └─ Check if course completed
  └─ If completed: publish course.completed event
  └─ Return progress update

☐ POST /quizzes/{quiz_id}/submit - Submit quiz answers
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "answers": [
      {"question_id": 1, "answer": "A"},
      {"question_id": 2, "answer": true}
    ]
  }
  
  Implementation:
  └─ Verify enrollment
  └─ Check max attempts not exceeded
  └─ Grade answers
  └─ Calculate score
  └─ Determine pass/fail
  └─ Save quiz_attempt
  └─ If passed: mark lesson complete
  └─ Return results (with correct answers if failed)

☐ GET /users/me/enrollments - Get my enrolled courses
  └─ List all enrollments with progress
  └─ Filter by status (active, completed, dropped)
```

---

**Dependencies Completed:**
- ✅ Territory service created
- ✅ Badge service implemented
- ✅ Permission system working
- ✅ Course service created
- ✅ Code of Conduct course seeded
- ✅ Badge auto-award on course completion

**Next Stage:** Forum Service, IPFS Service, and remaining services
## STAGE 8: Forum Service & IPFS Integration

### Step 8.1: Forum Service Scaffolding
```
☐ Create forum-service crate
  └─ cargo new services/forum-service --bin

☐ Create service structure
  /services/forum-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   ├── topics.rs
    │   ├── posts.rs
    │   ├── moderation.rs
    │   └── reactions.rs
    ├── models/
    │   ├── mod.rs
    │   ├── topic.rs
    │   ├── post.rs
    │   └── strike.rs
    └── services/
        ├── mod.rs
        ├── forum_service.rs
        └── moderation_service.rs
```

### Step 8.2: Forum Database Schema
```
☐ Add forum tables to territory schema
  
  -- Forum categories
  CREATE TABLE {schema}.forum_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    slug VARCHAR(100) UNIQUE NOT NULL,
    order_index INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  -- Forum topics (threads)
  CREATE TABLE {schema}.forum_topics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id UUID NOT NULL REFERENCES {schema}.forum_categories(id),
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) UNIQUE NOT NULL,
    created_by UUID NOT NULL REFERENCES {schema}.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_post_at TIMESTAMPTZ,
    is_pinned BOOLEAN DEFAULT FALSE,
    is_locked BOOLEAN DEFAULT FALSE,
    view_count INT DEFAULT 0,
    status VARCHAR(20) DEFAULT 'active'
  );
  
  CREATE INDEX idx_topics_category ON {schema}.forum_topics(category_id, last_post_at DESC);
  CREATE INDEX idx_topics_created_by ON {schema}.forum_topics(created_by);
  
  -- Forum posts
  CREATE TABLE {schema}.forum_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id UUID NOT NULL REFERENCES {schema}.forum_topics(id) ON DELETE CASCADE,
    parent_post_id UUID REFERENCES {schema}.forum_posts(id),
    content TEXT NOT NULL,
    created_by UUID NOT NULL REFERENCES {schema}.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMPTZ,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES {schema}.users(id)
  );
  
  CREATE INDEX idx_posts_topic ON {schema}.forum_posts(topic_id, created_at);
  CREATE INDEX idx_posts_created_by ON {schema}.forum_posts(created_by);
  
  -- Post reactions
  CREATE TABLE {schema}.post_reactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES {schema}.forum_posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    reaction_type VARCHAR(20) NOT NULL, -- like, love, laugh, sad, angry
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, user_id, reaction_type)
  );
  
  CREATE INDEX idx_reactions_post ON {schema}.post_reactions(post_id);
  
  -- User strikes (3-strike system)
  CREATE TABLE {schema}.user_strikes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES {schema}.users(id) ON DELETE CASCADE,
    issued_by UUID NOT NULL REFERENCES {schema}.users(id),
    reason TEXT NOT NULL,
    related_post_id UUID REFERENCES {schema}.forum_posts(id),
    strike_number INT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE
  );
  
  CREATE INDEX idx_strikes_user ON {schema}.user_strikes(user_id, is_active);
  
  -- Moderation log
  CREATE TABLE {schema}.moderation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    action_type VARCHAR(50) NOT NULL, -- delete_post, lock_topic, issue_strike, ban_user
    target_type VARCHAR(50) NOT NULL, -- post, topic, user
    target_id UUID NOT NULL,
    moderator_id UUID NOT NULL REFERENCES {schema}.users(id),
    reason TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_moderation_target ON {schema}.moderation_actions(target_type, target_id);

☐ Run migration
```

### Step 8.3: Forum Handlers Implementation
```
☐ GET /forum/categories - List forum categories
  Response:
  {
    "categories": [
      {
        "id": "uuid",
        "name": "General Discussion",
        "description": "General topics",
        "slug": "general",
        "topics_count": 142,
        "posts_count": 1523,
        "latest_post": {...}
      }
    ]
  }

☐ GET /forum/categories/{slug}/topics - List topics in category
  Query params: ?page=1&sort=latest
  
  Response:
  {
    "topics": [
      {
        "id": "uuid",
        "title": "Welcome to UnityPlan",
        "slug": "welcome-to-unityplan",
        "created_by": {...user},
        "created_at": "...",
        "last_post_at": "...",
        "posts_count": 23,
        "view_count": 142,
        "is_pinned": true,
        "is_locked": false
      }
    ]
  }

☐ POST /forum/topics - Create new topic
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "category_id": "uuid",
    "title": "How do I get started?",
    "content": "I'm new here and..."
  }
  
  Implementation:
  └─ Check user has "create_topic" permission (Code of Conduct badge)
  └─ Validate title and content
  └─ Generate slug from title
  └─ Create topic
  └─ Create first post
  └─ Publish topic.created event
  └─ Return topic info
  
  Error if no permission:
  {
    "error": "permission_denied",
    "message": "You need the Code of Conduct badge to create topics",
    "required_badge": "code_of_conduct"
  }

☐ GET /forum/topics/{slug} - Get topic with posts
  Query params: ?page=1
  
  Implementation:
  └─ Get topic info
  └─ Increment view_count
  └─ Get posts (paginated, 20 per page)
  └─ Include user info for each post
  └─ Include reaction counts
  └─ Return topic + posts

☐ POST /forum/topics/{topic_id}/posts - Create post (reply)
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "content": "Great question! Here's how...",
    "parent_post_id": "uuid" // optional, for threading
  }
  
  Implementation:
  └─ Check "create_post" permission
  └─ Check topic not locked
  └─ Validate content
  └─ Create post
  └─ Update topic.last_post_at
  └─ Publish post.created event
  └─ Return post info

☐ PUT /forum/posts/{post_id} - Edit post
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "content": "Updated content..."
  }
  
  Implementation:
  └─ Verify post ownership or moderator role
  └─ Update content
  └─ Set edited_at timestamp
  └─ Return updated post

☐ DELETE /forum/posts/{post_id} - Delete post
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Verify ownership or moderator role
  └─ Soft delete (set is_deleted = true)
  └─ Log moderation action if moderator
  └─ Return success

☐ POST /forum/posts/{post_id}/reactions - Add reaction
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "reaction_type": "like"
  }
  
  Implementation:
  └─ Validate reaction type
  └─ Insert or update reaction
  └─ Return reaction counts
```

### Step 8.4: Moderation System
```
☐ POST /forum/moderation/strike - Issue strike to user
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "user_id": "uuid",
    "reason": "Violated community guidelines",
    "related_post_id": "uuid"
  }
  
  Implementation:
  └─ Verify moderator permission
  └─ Count user's active strikes
  └─ Insert new strike (strike_number = count + 1)
  └─ Set expiration (90 days from now)
  └─ Log moderation action
  └─ Send notification to user
  └─ If strike_number == 3:
     └─ Revoke Code of Conduct badge
     └─ Publish moderation.violation event
     └─ Remove forum permissions
  └─ Return strike info

☐ GET /forum/moderation/queue - Get moderation queue
  Headers: Authorization: Bearer {token} (moderator)
  
  Response:
  {
    "flagged_posts": [
      {
        "post": {...},
        "flags_count": 5,
        "flag_reasons": ["spam", "harassment"],
        "flagged_at": "..."
      }
    ]
  }

☐ POST /forum/posts/{post_id}/flag - Flag post for moderation
  Headers: Authorization: Bearer {token}
  
  Request:
  {
    "reason": "spam"  // spam, harassment, offtopic, misinformation
  }
  
  Implementation:
  └─ Record flag
  └─ If flags > threshold: add to moderation queue
  └─ Notify moderators

☐ POST /forum/topics/{topic_id}/lock - Lock topic
  Headers: Authorization: Bearer {token} (moderator)
  
  Implementation:
  └─ Set is_locked = true
  └─ Log moderation action
  └─ Prevent new posts
```

### Step 8.5: Forum Frontend Pages
```
☐ Create forum category list page
  └─ Show all categories
  └─ Category stats
  └─ Latest posts preview

☐ Create topic list page
  └─ Filter by category
  └─ Sort options (latest, popular, oldest)
  └─ Pinned topics at top
  └─ Pagination

☐ Create topic view page
  └─ Topic title and first post
  └─ All replies (threaded or flat)
  └─ Pagination
  └─ Reply editor (if have permission)
  └─ Reaction buttons
  └─ Report button
  └─ Moderation tools (if moderator)

☐ Create topic creation form
  └─ Category selector
  └─ Title input
  └─ Rich text editor
  └─ Preview mode
  └─ Check permission before showing

☐ Create moderation dashboard (moderators only)
  └─ Flagged posts queue
  └─ Recent moderation actions
  └─ User strike management
  └─ Quick action buttons
```

---

## STAGE 9: IPFS Service

### Step 9.1: IPFS Setup
```
☐ Add IPFS to docker-compose.yml
  
  ipfs:
    image: ipfs/kubo:latest
    ports:
      - "5001:5001"  # API
      - "8080:8080"  # Gateway
    volumes:
      - ./docker/ipfs-data:/data/ipfs
    environment:
      - IPFS_PROFILE=server

☐ Initialize IPFS
  └─ docker-compose up -d ipfs
  └─ Configure IPFS (CORS, API access)
  └─ Test upload/download
```

### Step 9.2: IPFS Service Scaffolding
```
☐ Create ipfs-service crate
  └─ cargo new services/ipfs-service --bin
  
  └─ Dependencies:
     ipfs-api-backend-hyper = "0.6"
     futures = "0.3"

☐ Create service structure
  /services/ipfs-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   ├── mod.rs
    │   └── upload.rs
    ├── models/
    │   └── mod.rs
    └── services/
        ├── mod.rs
        └── ipfs_client.rs
```

### Step 9.3: IPFS Handlers
```
☐ POST /ipfs/upload - Upload file to IPFS
  Headers: 
    Authorization: Bearer {token}
    Content-Type: multipart/form-data
  
  Request: Form with 'file' field
  
  Implementation:
  └─ Validate file size (<100MB)
  └─ Validate file type (based on use case)
  └─ Upload to IPFS
  └─ Get CID (Content Identifier)
  └─ Optionally pin file
  └─ Store metadata in database
  └─ Return CID and gateway URL
  
  Response:
  {
    "cid": "Qm...",
    "url": "http://localhost:8080/ipfs/Qm...",
    "size": 1024567,
    "filename": "document.pdf"
  }

☐ GET /ipfs/{cid} - Retrieve file metadata
  Response:
  {
    "cid": "Qm...",
    "size": 1024567,
    "pinned": true,
    "uploaded_by": "uuid",
    "uploaded_at": "..."
  }

☐ POST /ipfs/{cid}/pin - Pin content (prevent garbage collection)
  Headers: Authorization: Bearer {token}
  
  Implementation:
  └─ Verify permissions (admin or uploader)
  └─ Pin to local IPFS node
  └─ Update database
  └─ Return success

☐ DELETE /ipfs/{cid}/pin - Unpin content
  Implementation:
  └─ Verify permissions
  └─ Unpin from IPFS node
  └─ Update database
  └─ Return success
```

### Step 9.4: Course Content Integration
```
☐ Update course lesson creation to use IPFS
  └─ Upload lesson content to IPFS
  └─ Store CID in lessons.ipfs_cid
  └─ Generate gateway URL for content_url
  
☐ Update lesson retrieval to serve from IPFS
  └─ Get CID from database
  └─ Return IPFS gateway URL
  └─ Or proxy content through service

☐ Create content upload UI
  └─ Drag-and-drop file upload
  └─ Progress indicator
  └─ Preview uploaded content
  └─ Display CID and URLs
```

---

## STAGE 10: Translation & Matrix Services

### Step 10.1: Translation Service (Basic)
```
☐ Create translation-service crate
  └─ cargo new services/translation-service --bin
  
  └─ Dependencies:
     reqwest = "0.11"  # For external API calls
     redis = "0.24"    # For caching

☐ Create service structure
  /services/translation-service/src
    ├── main.rs
    ├── config.rs
    ├── handlers/
    │   └── translate.rs
    └── services/
        └── translation_service.rs

☐ Implement translation handler
  POST /translate
  
  Request:
  {
    "text": "Hello world",
    "source_lang": "en",
    "target_lang": "da"
  }
  
  Implementation:
  └─ Check Redis cache first
  └─ If not cached: call external API (LibreTranslate or Google)
  └─ Cache result (90-day TTL)
  └─ Return translation
  
  Response:
  {
    "translated_text": "Hej verden",
    "source_lang": "en",
    "target_lang": "da",
    "cached": false
  }
```

### Step 10.2: Matrix Gateway (Basic)
```
☐ Add Matrix Synapse to docker-compose.yml
  
  matrix-synapse:
    image: matrixdotorg/synapse:latest
    ports:
      - "8008:8008"
    volumes:
      - ./docker/matrix-data:/data
    environment:
      - SYNAPSE_SERVER_NAME=localhost
      - SYNAPSE_REPORT_STATS=no

☐ Create matrix-gateway crate
  └─ cargo new services/matrix-gateway --bin
  
  └─ Dependencies:
     ruma = "0.10"
     tokio-tungstenite = "0.21"  # WebSocket

☐ Basic Matrix integration
  └─ Register users on Matrix when they register
  └─ Create Matrix room for each forum topic
  └─ Sync messages between forum and Matrix
  └─ (Full implementation in later stages)
```

---

## STAGE 11: Frontend - Course & Forum UI

### Step 11.1: Course Pages
```
☐ Create course catalog page
  └─ List all published courses
  └─ Filter by category
  └─ Search courses
  └─ Show enrollment status

☐ Create course detail page
  └─ Course info and description
  └─ Lesson list with progress
  └─ Enroll button
  └─ Start/continue learning button
  └─ Prerequisites display

☐ Create lesson viewer page
  └─ Lesson content display
  └─ Video player (if video lesson)
  └─ Text content rendering
  └─ Next/previous lesson navigation
  └─ Mark complete button
  └─ Progress bar

☐ Create quiz page
  └─ Display questions
  └─ Answer selection
  └─ Submit button
  └─ Results display
  └─ Retry option (if failed)
  └─ Show correct answers

☐ Create my learning page
  └─ Enrolled courses
  └─ Progress overview
  └─ Completed courses
  └─ Recommended courses
```

### Step 11.2: Forum Pages (described earlier)
```
☐ Forum category list
☐ Topic list
☐ Topic view with posts
☐ Create topic form
☐ Moderation dashboard
```

---

## STAGE 12: Testing, Documentation & Deployment

### Step 12.1: Comprehensive Testing
```
☐ Unit tests for all services (80%+ coverage)
☐ Integration tests for API endpoints
☐ E2E tests for critical user flows:
  └─ Registration → Login → Complete Code of Conduct → Create Forum Post
  └─ Enroll in course → Complete lessons → Earn badge
  └─ Upload content to IPFS → Use in course

☐ Load testing
  └─ Auth service: 100 req/s
  └─ Course service: 50 req/s
  └─ Forum service: 50 req/s
  └─ Database performance under load

☐ Security testing
  └─ SQL injection prevention
  └─ XSS prevention
  └─ CSRF protection
  └─ Rate limiting
  └─ Permission enforcement
```

### Step 12.2: Documentation
```
☐ API documentation (OpenAPI/Swagger)
  └─ Generate from code
  └─ Interactive API explorer
  └─ Example requests/responses

☐ Developer documentation
  └─ Setup instructions
  └─ Architecture overview
  └─ Service interactions
  └─ Database schema diagrams
  └─ Contribution guidelines

☐ User documentation
  └─ Getting started guide
  └─ Feature tutorials
  └─ FAQ
  └─ Video walkthroughs
```

### Step 12.3: Deployment Setup
```
☐ Production docker-compose.yml
  └─ Environment-specific configs
  └─ Secrets management
  └─ Resource limits
  └─ Health checks
  └─ Restart policies

☐ CI/CD pipeline (GitHub Actions)
  └─ Run tests on PR
  └─ Build Docker images
  └─ Deploy to staging
  └─ Deploy to production (manual approval)

☐ Monitoring setup
  └─ Prometheus metrics collection
  └─ Grafana dashboards
  └─ Alert rules
  └─ Log aggregation

☐ Backup strategy
  └─ Database backups (daily)
  └─ IPFS content backups
  └─ Configuration backups
  └─ Restore procedures
```

---

## Phase 1 Complete Checklist

```
Infrastructure:
☐ Docker infrastructure running
☐ PostgreSQL with TimescaleDB
☐ NATS message bus
☐ Redis caching
☐ IPFS node
☐ Matrix Synapse

Backend Services:
☐ Auth Service (login, register, JWT)
☐ User Service (profiles, privacy, connections)
☐ Territory Service (territory management)
☐ Badge Service (badges, permissions, auto-award)
☐ Course Service (courses, lessons, quizzes)
☐ Forum Service (topics, posts, moderation)
☐ IPFS Service (file upload/storage)
☐ Translation Service (basic caching)
☐ Matrix Gateway (basic integration)

Frontend:
☐ Authentication pages (login, register)
☐ Profile pages (view, edit)
☐ Course catalog and viewer
☐ Forum (categories, topics, posts)
☐ Protected routes
☐ Responsive design
☐ Error handling

Features Complete:
☐ User registration and authentication
☐ JWT-based auth with refresh tokens
☐ Multi-territory support
☐ User profiles with privacy settings
☐ Badge system with permission enforcement
☐ Code of Conduct course and badge
☐ Course enrollment and completion
☐ Forum with 3-strike moderation
☐ IPFS content storage
☐ Basic translation caching
☐ Matrix room creation

Testing:
☐ Unit tests (>80% coverage)
☐ Integration tests
☐ E2E tests for critical flows
☐ Load testing passed
☐ Security testing passed

Documentation:
☐ API documentation
☐ Developer documentation
☐ User documentation
☐ Deployment guides

Deployment:
☐ Production environment configured
☐ CI/CD pipeline working
☐ Monitoring and alerting
☐ Backup/restore tested
☐ 3-5 territories deployed
☐ 50-100 beta users
```

---

**Estimated Timeline:** 6-9 months with 3-5 developers
**Success Criteria:** All checklist items completed, 99.5% uptime, <200ms API response time
