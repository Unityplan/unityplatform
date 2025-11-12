# Platform Naming Conventions

**Last Updated:** November 11, 2025

---

## Platform vs. Project Names

### Unity Platform (The Software)

**What it is:** The open-source platform software being developed in this repository.

**Use cases:**

- Documentation about the platform architecture
- Technical specifications
- API documentation
- Developer guides
- Marketing/promotional materials about the software itself

**Examples:**

- "Unity Platform implements user sovereignty..."
- "The platform's multi-tenant architecture..."
- "Install Unity Platform on your server"

### unityplan.org (Example Deployment)

**What it is:** A specific deployment/project using Unity Platform for testing and development purposes.

**Use cases:**

- Example URLs in code comments
- Test deployment references
- Development environment examples
- **Should NOT** appear in generic platform documentation

**Examples (acceptable):**

```javascript
// Example deployment (unityplan.org test project)
const API_URL = 'https://api.denmark.example.org';
```

**Examples (avoid in docs):**

- ❌ "UnityPlan is a platform for..." → ✅ "Unity Platform is a platform for..."
- ❌ "Connect to unityplan.org" → ✅ "Connect to your-domain.org"

---

## Documentation Guidelines

### Generic Platform Documentation

Use neutral terminology that applies to any deployment:

**Good:**

- "The platform supports..."
- "Your deployment will have..."
- "Each territory pod connects to..."
- "Example: <https://api.example.org>"

**Avoid:**

- "UnityPlan provides..."
- "Connect to unityplan.org"
- "The unityplan.dk server"

### Example Code and URLs

When providing examples, use generic example domains:

**Recommended:**

- `example.org`, `example.com` (IANA reserved)
- `your-domain.org`
- `territory-name.example.org`

**For Development:**

- `localhost:8001` (clear it's local)
- `dev.example.org` (marked as dev)

**Avoid:**

- `unityplan.org` (unless explicitly noting it's the test project)
- Real production URLs in documentation

---

## Database Examples

### Territory Seed Data

```sql
-- ✅ Good: Generic example
INSERT INTO territories (code, name, pod_url, api_url)
VALUES (
    'dk',
    'Denmark',
    'https://denmark.example.org',
    'https://api.denmark.example.org'
);

-- ❌ Avoid: Specific to test project
VALUES (
    'dk', 
    'Denmark',
    'https://denmark.unityplan.org',
    'https://api.denmark.unityplan.org'
);
```

### API Examples

```json
// ✅ Good: Generic example
{
  "api_url": "https://api.example.org/v1",
  "matrix_server": "https://matrix.dk.example.org"
}

// ❌ Avoid: Test project specific
{
  "api_url": "https://api.unityplan.org/v1",
  "matrix_server": "https://matrix.dk.unityplan.org"
}
```

---

## When to Mention unityplan.org

**Acceptable contexts:**

1. **README.md** - Can mention it as "Example Deployment: unityplan.org"
2. **Development setup** - When referring to the actual test environment
3. **Release notes** - Deployment-specific information
4. **With clear labeling** - Always mark as "example" or "test deployment"

**Examples:**

```markdown
> **Note:** This guide uses example.org URLs. 
> The test deployment uses unityplan.org.
```

---

## Matrix IDs and Federation

**Generic documentation:**

```
@username:example.{territory}
@alice:example.dk
```

**Test deployment specific:**

```
// Only in deployment-specific configs
@alice:unityplan.dk
```

---

## File and Variable Names

### Codebase

**Configuration files:**

```bash
# ✅ Generic platform config
platform-config.yml
territory-dk.env

# ❌ Avoid project-specific names
unityplan-config.yml
```

**Variables:**

```rust
// ✅ Generic
let territory_url = config.get_territory_url("dk");
let platform_name = "Unity Platform";

// ❌ Avoid hardcoding project name
let unityplan_url = "https://unityplan.org";
```

---

## Summary Checklist

Before committing documentation:

- [ ] Platform name is "Unity Platform" (not "UnityPlan Platform")
- [ ] Example URLs use `example.org` (not `unityplan.org`)
- [ ] Code examples are generic and reusable
- [ ] Any `unityplan.org` references are clearly labeled as test deployment
- [ ] Variable names use generic terminology
- [ ] Database seed data uses example domains

---

## Migration from Old Naming

During the cleanup (November 11, 2025), we updated:

1. ✅ Root documentation files (README.md, VERSIONS.md)
2. ✅ Copilot instructions (.github/copilot-instructions.md)
3. ✅ Architecture documentation (all files in docs/architecture/)
4. ✅ API implementation guide (docs/guides/development/)
5. ✅ Database migration seed data
6. ✅ Live database territory records

**Remaining tasks:**

- [ ] Frontend code (when developed)
- [ ] Service configuration templates
- [ ] Docker compose examples
- [ ] Deployment scripts documentation
