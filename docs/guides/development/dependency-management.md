# Dependency Management Guide - Unity Platform

**Version:** 0.1.0-alpha.1  
**Status:** ✅ Active Standard  
**Last Updated:** November 14, 2025

## Overview

Unity Platform uses **Cargo workspace dependencies** to ensure all services use the same library versions. This prevents version conflicts, ensures consistent behavior, and simplifies dependency updates.

## The Problem We're Solving

**Without workspace dependencies:**

```toml
# service-a/Cargo.toml
actix-web = "4.9"
tokio = { version = "1.41", features = ["full"] }

# service-b/Cargo.toml  
actix-web = "4.8"  # ❌ Different version!
tokio = { version = "1.40", features = ["rt-multi-thread"] }  # ❌ Different features!
```

**Problems:**

- Version drift between services
- Difficult to upgrade dependencies (must update every Cargo.toml)
- Risk of incompatible feature flags
- Larger build times (multiple versions compiled)
- Potential runtime issues from version mismatches

## The Solution: Workspace Dependencies

### **Centralized Version Management**

All dependency versions are defined once in `services/Cargo.toml`:

```toml
# services/Cargo.toml
[workspace.dependencies]
actix-web = "4.9"
tokio = { version = "1.41", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "json"] }
# ... all other dependencies
```

### **Services Reference Workspace Versions**

Each service uses `{ workspace = true }`:

```toml
# services/auth-service/Cargo.toml
[package]
name = "auth-service"
version.workspace = true      # ← Package metadata from workspace
edition.workspace = true
authors.workspace = true

[dependencies]
actix-web = { workspace = true }     # ← Version from workspace
tokio = { workspace = true }         # ← Version + features from workspace
sqlx = { workspace = true }          # ← All features defined in workspace
```

## Standard Service Cargo.toml Template

**Use this template for every new service:**

```toml
[package]
name = "your-service"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
homepage.workspace = true
repository.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true

[dependencies]
# Shared library (always first)
shared-lib = { path = "../shared-lib" }

# Web framework
actix-web = { workspace = true }
actix-cors = { workspace = true }

# Async runtime
tokio = { workspace = true }

# Database (if needed)
sqlx = { workspace = true }
redis = { workspace = true }

# NATS messaging (if needed)
async-nats = { workspace = true }
bytes = { workspace = true }
futures-util = { workspace = true }

# Serialization
serde = { workspace = true }
serde_json = { workspace = true }

# Validation
validator = { workspace = true }

# UUID & Time
uuid = { workspace = true }
chrono = { workspace = true }

# Logging
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

# Configuration
dotenvy = { workspace = true }

# Error handling
thiserror = { workspace = true }
anyhow = { workspace = true }

# API Documentation
utoipa = { workspace = true }
utoipa-swagger-ui = { workspace = true }

# Authentication (if needed)
jsonwebtoken = { workspace = true }
argon2 = { workspace = true }
rand = { workspace = true }
base64 = { workspace = true }

[dev-dependencies]
# Add test dependencies here (also use workspace = true when available)
```

## Benefits

### ✅ **Consistency**

- All services use identical library versions
- No version drift between microservices
- Identical feature flags across workspace

### ✅ **Maintainability**

- Update dependency once → applies to all services
- Easy to see all project dependencies in one file
- Clear dependency inventory

### ✅ **Build Performance**

- Cargo only compiles each dependency version once
- Shared build cache across services
- Faster CI/CD pipelines

### ✅ **Safety**

- No accidental version mismatches
- Consistent behavior across services
- Easier to reproduce bugs

## Adding New Dependencies

### Step 1: Add to Workspace

```toml
# services/Cargo.toml
[workspace.dependencies]
# ... existing dependencies ...

# New dependency
your-new-crate = { version = "1.0", features = ["feature-a", "feature-b"] }
```

### Step 2: Use in Service

```toml
# services/your-service/Cargo.toml
[dependencies]
your-new-crate = { workspace = true }
```

### Step 3: Verify

```bash
cd services
cargo check --workspace
```

## Upgrading Dependencies

### Single Dependency Update

```bash
# Update in workspace Cargo.toml
cd services
cargo update -p actix-web

# Verify all services still work
cargo test --workspace
cargo build --workspace
```

### Major Version Upgrade

1. Update version in `services/Cargo.toml`
2. Check breaking changes in dependency changelog
3. Update code in all affected services
4. Test entire workspace:

```bash
cargo test --workspace
cargo build --workspace --release
```

## Special Cases

### Service-Specific Dependencies

If a dependency is only used by one service and unlikely to be shared:

```toml
# Still add to workspace for consistency
[workspace.dependencies]
service-specific-crate = "1.0"

# Then use in that one service
[dependencies]
service-specific-crate = { workspace = true }
```

**Rationale:** Even single-service dependencies benefit from centralized version management.

### Different Feature Flags Needed

If two services need different features from the same crate:

**Option 1: Union of Features (Recommended)**

```toml
# services/Cargo.toml - Include all needed features
[workspace.dependencies]
tokio = { version = "1.41", features = ["full"] }  # Superset of all needs
```

**Option 2: Separate Dependency Names (Rare)**

```toml
# Only if feature conflict exists (very rare)
[workspace.dependencies]
tokio-full = { package = "tokio", version = "1.41", features = ["full"] }
tokio-minimal = { package = "tokio", version = "1.41", features = ["rt-multi-thread"] }
```

## Validation Checklist

Before merging new services, verify:

- [ ] ✅ All dependencies use `{ workspace = true }`
- [ ] ✅ No hardcoded versions in service Cargo.toml
- [ ] ✅ Package metadata uses `.workspace = true`
- [ ] ✅ `cargo check --workspace` passes
- [ ] ✅ No duplicate dependencies with different versions
- [ ] ✅ All features needed are in workspace definition

## Common Mistakes

### ❌ **Hardcoded Versions**

```toml
# DON'T DO THIS
actix-web = "4.9"  # ❌ Should be { workspace = true }
```

### ❌ **Missing Package Metadata**

```toml
# DON'T DO THIS
[package]
name = "my-service"
version = "0.1.0"  # ❌ Should be version.workspace = true
edition = "2021"   # ❌ Should be edition.workspace = true
```

### ❌ **Partial Workspace Usage**

```toml
# DON'T DO THIS - Be consistent
actix-web = { workspace = true }  # ✅
tokio = "1.41"                    # ❌ Should also use workspace
```

## Reference Examples

- ✅ **auth-service/Cargo.toml** - Perfect example of workspace pattern
- ✅ **badge-service/Cargo.toml** - Updated to use workspace dependencies
- ✅ **user-service/Cargo.toml** - Updated to use workspace dependencies

## Tools

### Check for Non-Workspace Dependencies

```bash
# Find any hardcoded versions
cd services
grep -r "version = " */Cargo.toml | grep -v "workspace = true" | grep -v "path = "
```

### Verify Workspace Consistency

```bash
# Ensure all services build together
cargo check --workspace
cargo test --workspace
cargo clippy --workspace
```

## See Also

- [Cargo Workspace Documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Workspace Dependencies RFC](https://rust-lang.github.io/rfcs/2906-cargo-workspace-deduplicate.html)
- `services/Cargo.toml` - Central dependency definitions
- `docs/guides/development/versioning-strategy.md` - Version numbering strategy
