#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs from list-labels.sh
PRIORITY_HIGH_ID=2
PRIORITY_MEDIUM_ID=3
TYPE_FEATURE_ID=5
TYPE_INFRASTRUCTURE_ID=8
AREA_INVITATION_SERVICE_ID=26
AREA_AUTH_SERVICE_ID=9
AREA_FRONTEND_ID=14

# Milestone ID
MILESTONE_ID=1  # v0.1.0-alpha.2

# Function to create open issue
create_open_issue() {
    local title="$1"
    local body="$2"
    local label_ids="$3"
    
    # Convert label IDs to JSON array of numbers
    labels_json=$(printf '%s\n' ${label_ids//,/ } | jq -R 'tonumber' | jq -s .)
    
    # Create the issue (open by default)
    response=$(curl -s -X POST \
        -H "Authorization: token ${FORGEJO_TOKEN}" \
        -H "Content-Type: application/json" \
        "${FORGEJO_URL}/api/v1/repos/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues" \
        -d "{
            \"title\": \"${title}\",
            \"body\": \"${body}\",
            \"labels\": ${labels_json},
            \"milestone\": ${MILESTONE_ID}
        }")
    
    issue_number=$(echo "$response" | jq -r '.number')
    
    if [ "$issue_number" != "null" ]; then
        echo "✅ Created issue #${issue_number}: ${title}"
        echo "${FORGEJO_URL}/${FORGEJO_OWNER}/${FORGEJO_REPO}/issues/${issue_number}"
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 5.9 issues (Invitation System)..."
echo "=========================================="
echo ""

# ============================================================================
# Backend - Database Migration (Issues 5.9.1 - 5.9.3)
# ============================================================================

create_open_issue \
    "5.9.1: Create invitation service database migration" \
    "## Description
Create database migration for invitation-service tables following the \`{service}_{entity}_{data}\` naming convention.

## Acceptance Criteria
- [ ] Migration file created: \`services/shared-lib/migrations/20251118000009_invitation_service_tables.sql\`
- [ ] Global table created: \`global.registry_invitation\`
- [ ] Territory table created: \`territory_{code}.invitation_invitations_tokens\`
- [ ] Territory table created: \`territory_{code}.invitation_invitations_uses\`
- [ ] All indexes created as per DATABASE.md
- [ ] Table comments added for service independence documentation
- [ ] Column comments added for cross-service references
- [ ] Migration tested on clean database
- [ ] Migration tested with existing data (DK pod)

## Technical Details
Tables to create:

**1. Global Registry:**
\`\`\`sql
CREATE TABLE global.registry_invitation (
    token VARCHAR(255) PRIMARY KEY,
    territory_code VARCHAR(10) NOT NULL REFERENCES global.territories(code),
    territory_token_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
\`\`\`

**2. Territory Tokens:**
\`\`\`sql
CREATE TABLE territory_{code}.invitation_invitations_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token VARCHAR(255) NOT NULL UNIQUE,
    created_by UUID NOT NULL,  -- No FK for service independence
    max_uses INT NOT NULL DEFAULT 1,
    uses_count INT NOT NULL DEFAULT 0,
    expires_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
\`\`\`

**3. Territory Uses:**
\`\`\`sql
CREATE TABLE territory_{code}.invitation_invitations_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invitation_id UUID NOT NULL REFERENCES invitation_invitations_tokens(id),
    used_by UUID NOT NULL,  -- No FK for service independence
    ip_address INET,
    user_agent TEXT,
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (invitation_id, used_by)
);
\`\`\`

## Reference
\`docs/architecture/services/invitation-service/DATABASE.md\`

## Dependencies
None - this is the foundation for all invitation service work" \
    "${PRIORITY_HIGH_ID},${TYPE_DATABASE_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.2: Apply invitation service migration to DK pod" \
    "## Description
Apply the invitation service migration to the Denmark pod database.

## Acceptance Criteria
- [ ] Migration applied successfully to \`unityplatform_dk\` database
- [ ] All tables created in global schema
- [ ] All tables created in territory_dk schema
- [ ] Migration recorded in \`shared_lib_migrations\` table
- [ ] Database schema validated with \`\\dt territory_dk.invitation*\`
- [ ] Test data created for development (optional)

## Commands
\`\`\`bash
# Apply migration
psql -h localhost -p 5432 -U unityplatform -d unityplatform_dk \\
  -f services/shared-lib/migrations/20251118000009_invitation_service_tables.sql

# Verify tables
psql -h localhost -p 5432 -U unityplatform -d unityplatform_dk \\
  -c \"\\dt global.registry_invitation\" \\
  -c \"\\dt territory_dk.invitation_invitations_*\"
\`\`\`

## Dependencies
- #5.9.1 (database migration must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_DEPLOYMENT_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.3: Update MIGRATIONS-MASTER.md with invitation tables" \
    "## Description
Document the invitation service tables in the master migrations documentation.

## Acceptance Criteria
- [ ] Add invitation tables to migration tracking
- [ ] Document naming convention rationale
- [ ] Add examples in \"Service Table Naming\" section
- [ ] Update \"Migration History\" section
- [ ] Git commit with message: \`docs: add invitation service tables to MIGRATIONS-MASTER\`

## Reference
\`docs/architecture/MIGRATIONS-MASTER.md\`

## Dependencies
- #5.9.1 (migration must exist to document)" \
    "${PRIORITY_MEDIUM_ID},${TYPE_DOCUMENTATION_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

# ============================================================================
# Backend - Invitation Service Implementation (Issues 5.9.4 - 5.9.9)
# ============================================================================

create_open_issue \
    "5.9.4: Create invitation-service project structure" \
    "## Description
Create the Rust project structure for invitation-service following the standard service pattern.

## Acceptance Criteria
- [ ] Create \`services/invitation-service/\` directory
- [ ] Create \`Cargo.toml\` with dependencies
- [ ] Create standard directory structure:
  - \`src/main.rs\` - Server setup with middleware
  - \`src/lib.rs\` - Public exports
  - \`src/handlers/\` - HTTP request handlers
  - \`src/models/\` - Request/Response types
  - \`src/services/\` - Business logic
- [ ] Configure shared-lib dependencies
- [ ] Add to workspace \`services/Cargo.toml\`
- [ ] Service builds successfully with \`cargo build -p invitation-service\`

## Standard Dependencies
\`\`\`toml
[dependencies]
actix-web = \"4.5\"
serde = { version = \"1.0\", features = [\"derive\"] }
shared_lib = { path = \"../shared-lib\" }
sqlx = { version = \"0.7\", features = [\"runtime-tokio-rustls\", \"postgres\", \"uuid\", \"chrono\"] }
uuid = { version = \"1.0\", features = [\"v4\", \"serde\"] }
chrono = { version = \"0.4\", features = [\"serde\"] }
validator = { version = \"0.16\", features = [\"derive\"] }
rand = \"0.8\"
\`\`\`

## Reference
See \`services/auth-service/\` for structure pattern

## Dependencies
- #5.9.1 (database tables must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.5: Implement invitation token generation service" \
    "## Description
Implement the core service logic for generating unique invitation tokens.

## Acceptance Criteria
- [ ] Create \`src/services/invitation_service.rs\`
- [ ] Implement \`generate_token()\` function:
  - 16 random alphanumeric characters
  - Format as XXXX-XXXX-XXXX-XXXX
  - Exclude confusing characters (0, O, I, 1, l)
- [ ] Implement global uniqueness check against \`registry_invitation\`
- [ ] Implement database transaction:
  - Insert into \`invitation_invitations_tokens\`
  - Insert into \`global.registry_invitation\`
  - Rollback on conflict
- [ ] Unit tests for token generation
- [ ] Unit tests for uniqueness enforcement

## Function Signature
\`\`\`rust
pub async fn create_invitation(
    pool: &PgPool,
    created_by: Uuid,
    territory_code: &str,
    max_uses: i32,
    expires_in_days: Option<i32>,
    metadata: Option<serde_json::Value>,
) -> Result<Invitation, AppError>
\`\`\`

## Dependencies
- #5.9.4 (service structure must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.6: Implement invitation validation endpoint" \
    "## Description
Implement POST /api/v1/invitations/validate endpoint called by auth-service during registration.

## Acceptance Criteria
- [ ] Create \`src/handlers/invitation_handler.rs\`
- [ ] Implement \`validate_invitation()\` handler
- [ ] Validate request body (token required)
- [ ] Check token exists in database
- [ ] Check \`is_active = true\`
- [ ] Check not expired (\`expires_at > NOW()\` or NULL)
- [ ] Check usage limit (\`uses_count < max_uses\` or \`max_uses = 0\`)
- [ ] Return validation result with metadata
- [ ] Endpoint accessible without authentication (public)
- [ ] Integration test with test database

## API Contract
\`\`\`rust
// Request
struct ValidateRequest {
    token: String,
}

// Response
struct ValidateResponse {
    valid: bool,
    invitation_id: Option<Uuid>,
    created_by: Option<Uuid>,
    uses_remaining: Option<i32>,
    expires_at: Option<DateTime<Utc>>,
    metadata: Option<serde_json::Value>,
}
\`\`\`

## Reference
\`docs/architecture/services/invitation-service/API.md\`

## Dependencies
- #5.9.5 (token generation logic must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.7: Implement invitation usage tracking endpoint" \
    "## Description
Implement POST /api/v1/invitations/use endpoint called by auth-service after successful registration.

## Acceptance Criteria
- [ ] Implement \`use_invitation()\` handler
- [ ] Validate request body (token, used_by required)
- [ ] Database transaction:
  - Insert into \`invitation_invitations_uses\`
  - Increment \`uses_count\` in \`invitation_invitations_tokens\`
  - Set \`is_active = false\` if fully used
- [ ] Handle duplicate usage (same user, same invitation)
- [ ] Return remaining uses count
- [ ] Publish NATS event: \`invitation.used\`
- [ ] Integration test with test database

## API Contract
\`\`\`rust
// Request
struct UseRequest {
    token: String,
    used_by: Uuid,
    ip_address: Option<String>,
    user_agent: Option<String>,
}

// Response
struct UseResponse {
    invitation_id: Uuid,
    uses_remaining: i32,
    fully_used: bool,
}
\`\`\`

## Reference
\`docs/architecture/services/invitation-service/API.md\`

## Dependencies
- #5.9.6 (validation endpoint must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.8: Implement manager-only invitation creation endpoint" \
    "## Description
Implement POST /api/v1/invitations endpoint with manager role authorization.

## Acceptance Criteria
- [ ] Implement \`create_invitation()\` handler
- [ ] Require JWT authentication (shared-lib middleware)
- [ ] Verify manager role from JWT claims (territory or community manager)
- [ ] Validate request body (max_uses, expires_in_days)
- [ ] Call \`invitation_service::create_invitation()\`
- [ ] Return invitation token and invite URL
- [ ] Return 403 if user is not a manager
- [ ] Publish NATS event: \`invitation.created\`
- [ ] Integration test with manager JWT
- [ ] Integration test with non-manager JWT (403 expected)

## API Contract
\`\`\`rust
// Request
struct CreateRequest {
    max_uses: i32,           // 1 = single-use, 0 = unlimited
    expires_in_days: Option<i32>,
    metadata: Option<serde_json::Value>,
}

// Response
struct CreateResponse {
    id: Uuid,
    token: String,
    created_by: Uuid,
    max_uses: i32,
    expires_at: Option<DateTime<Utc>>,
    invite_url: String,
}
\`\`\`

## Manager Role Check
\`\`\`rust
// Extract from JWT claims or verify with territory-service
if !user.is_manager() {
    return Err(AppError::Forbidden(\"Only managers can create invitations\".into()));
}
\`\`\`

## Reference
\`docs/architecture/services/invitation-service/API.md\`

## Dependencies
- #5.9.5 (token generation service must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.9: Implement invitation listing and revocation endpoints" \
    "## Description
Implement GET /invitations/me and DELETE /invitations/{id} endpoints.

## Acceptance Criteria
- [ ] Implement \`list_my_invitations()\` handler
  - Filter by status (active, used, expired, revoked)
  - Pagination support (page, limit)
  - Include usage count and uses array
  - Return only user's own invitations
- [ ] Implement \`get_invitation_uses()\` handler
  - Return detailed usage list
  - Include username lookup (join with auth_users_core)
  - Only allow creator or manager to view
- [ ] Implement \`revoke_invitation()\` handler
  - Set \`is_active = false\`
  - Set \`revoked_at = NOW()\`
  - Set \`revoked_by = current_user_id\`
  - Only allow creator or manager to revoke
  - Publish NATS event: \`invitation.revoked\`
- [ ] Integration tests for all endpoints

## Reference
\`docs/architecture/services/invitation-service/API.md\`

## Dependencies
- #5.9.8 (creation endpoint provides base patterns)" \
    "${PRIORITY_MEDIUM_ID},${TYPE_BACKEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

# ============================================================================
# Backend - Auth Service Integration (Issues 5.9.10 - 5.9.11)
# ============================================================================

create_open_issue \
    "5.9.10: Update auth-service registration with production mode" \
    "## Description
Update auth-service registration to enforce invitation-only registration in production.

## Acceptance Criteria
- [ ] Add environment variable: \`ALLOW_OPEN_REGISTRATION\` (default: false)
- [ ] Update \`AppConfig\` in shared-lib or auth-service config
- [ ] Update registration handler:
  - If production mode: require \`invitation_token\` field (non-optional)
  - If dev mode: allow optional \`invitation_token\` for testing
- [ ] Call invitation-service \`/validate\` endpoint (HTTP client)
- [ ] Call invitation-service \`/use\` endpoint after successful registration
- [ ] Handle invitation-service errors gracefully
- [ ] Return clear error if invitation required but missing
- [ ] Update API documentation
- [ ] Integration tests for both modes

## Implementation
\`\`\`rust
// In registration handler
if !config.allow_open_registration {
    let invitation_token = body.invitation_token
        .ok_or(AppError::BadRequest(\"Invitation token required in production\"))?;
    
    // Validate with invitation-service
    let validation = invitation_client
        .validate(&invitation_token)
        .await?;
    
    if !validation.valid {
        return Err(AppError::BadRequest(\"Invalid or expired invitation\"));
    }
}

// After successful registration
if let Some(token) = &body.invitation_token {
    invitation_client
        .mark_used(token, user.id, ip_address, user_agent)
        .await?;
}
\`\`\`

## Reference
\`docs/architecture/services/auth-service/README.md\`

## Dependencies
- #5.9.7 (invitation usage endpoint must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_AUTH_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.11: Create invitation-service HTTP client for auth-service" \
    "## Description
Create HTTP client for auth-service to communicate with invitation-service.

## Acceptance Criteria
- [ ] Create \`src/clients/invitation_client.rs\` in auth-service
- [ ] Implement \`validate()\` method (POST /api/v1/invitations/validate)
- [ ] Implement \`mark_used()\` method (POST /api/v1/invitations/use)
- [ ] Use \`reqwest\` for HTTP requests
- [ ] Handle network errors gracefully
- [ ] Add request timeout (5 seconds)
- [ ] Add retry logic for transient failures (optional)
- [ ] Unit tests with mocked HTTP responses
- [ ] Configuration: \`INVITATION_SERVICE_URL\` environment variable

## Client Implementation
\`\`\`rust
pub struct InvitationClient {
    base_url: String,
    client: reqwest::Client,
}

impl InvitationClient {
    pub async fn validate(&self, token: &str) -> Result<ValidationResponse> {
        let response = self.client
            .post(format!(\"{}/api/v1/invitations/validate\", self.base_url))
            .json(&ValidateRequest { token: token.to_string() })
            .timeout(Duration::from_secs(5))
            .send()
            .await?;
        
        let body: ApiResponse<ValidationResponse> = response.json().await?;
        Ok(body.data)
    }
    
    pub async fn mark_used(
        &self,
        token: &str,
        used_by: Uuid,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<UseResponse> {
        // Similar implementation
    }
}
\`\`\`

## Dependencies
- #5.9.6 (validation endpoint must exist)
- #5.9.7 (usage endpoint must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_BACKEND_ID},${AREA_AUTH_SERVICE_ID},${STAGE_5_ID}"

# ============================================================================
# Frontend - Invitation UI (Issues 5.9.12 - 5.9.15)
# ============================================================================

create_open_issue \
    "5.9.12: Create invitation API client" \
    "## Description
Create TypeScript API client for invitation-service endpoints.

## Acceptance Criteria
- [ ] Create \`app/src/api/invitations.ts\`
- [ ] Implement type definitions for all request/response types
- [ ] Implement API methods:
  - \`createInvitation()\`
  - \`validateInvitation()\` (public, no auth)
  - \`listMyInvitations()\`
  - \`getInvitationUses()\`
  - \`revokeInvitation()\`
- [ ] Use \`api-client.ts\` for axios instance (auth interceptors)
- [ ] Error handling for all methods
- [ ] TypeScript types exported

## Implementation
\`\`\`typescript
// app/src/api/invitations.ts
import { apiClient } from '@/lib/api-client';

export interface CreateInvitationRequest {
  max_uses: number;
  expires_in_days?: number;
  metadata?: Record<string, any>;
}

export interface Invitation {
  id: string;
  token: string;
  created_by: string;
  max_uses: number;
  uses_count: number;
  is_active: boolean;
  expires_at?: string;
  created_at: string;
  status: 'active' | 'used' | 'expired' | 'revoked';
  uses: InvitationUse[];
}

export const invitationApi = {
  create: (data: CreateInvitationRequest) =>
    apiClient.post<{ data: Invitation }>('/invitations', data),
  
  listMy: (params?: { status?: string; page?: number; limit?: number }) =>
    apiClient.get<{ data: { invitations: Invitation[] } }>('/invitations/me', { params }),
  
  getUses: (id: string) =>
    apiClient.get<{ data: { uses: InvitationUse[] } }>(\`/invitations/\${id}/uses\`),
  
  revoke: (id: string) =>
    apiClient.delete<{ data: { invitation_id: string } }>(\`/invitations/\${id}\`),
};
\`\`\`

## Dependencies
- #5.9.8, #5.9.9 (backend endpoints must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_FRONTEND_ID},${AREA_INVITATION_SERVICE_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.13: Create invitation management page" \
    "## Description
Create the invitation management page where managers can create and manage invitations.

## Acceptance Criteria
- [ ] Create \`app/src/pages/invitations/InvitationManagementPage.tsx\`
- [ ] Display list of user's created invitations
- [ ] Show invitation status (active, used, expired, revoked)
- [ ] Show usage count (2/5 uses)
- [ ] Button to create new invitation (opens dialog)
- [ ] Button to copy invitation link
- [ ] Button to revoke invitation
- [ ] Filter by status (tabs: All, Active, Used, Expired)
- [ ] Pagination support
- [ ] Loading states with skeletons
- [ ] Error handling with toast notifications
- [ ] Protected route (manager-only)

## UI Components
- \`InvitationCard\` - Display single invitation with actions
- \`CreateInvitationDialog\` - Form to create new invitation
- \`InvitationUsesDialog\` - Show who used the invitation

## Route
\`/invitations\` or \`/dashboard/invitations\`

## Dependencies
- #5.9.12 (API client must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_FRONTEND_ID},${AREA_FRONTEND_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.14: Create invitation creation dialog" \
    "## Description
Create the dialog for creating new invitation tokens.

## Acceptance Criteria
- [ ] Create \`app/src/components/invitations/CreateInvitationDialog.tsx\`
- [ ] Form fields:
  - Max uses (1, 5, 10, 25, unlimited)
  - Expiration (7 days, 30 days, never)
  - Purpose/Note (optional metadata)
- [ ] Form validation with react-hook-form + zod
- [ ] Submit to \`invitationApi.create()\`
- [ ] Show created invitation token
- [ ] Copy to clipboard button
- [ ] Copy full invite URL button
- [ ] Success message with confetti animation (optional)
- [ ] Error handling

## Form Schema
\`\`\`typescript
const createInvitationSchema = z.object({
  max_uses: z.number().min(0).max(1000),
  expires_in_days: z.number().min(1).max(365).optional(),
  purpose: z.string().max(200).optional(),
});
\`\`\`

## Dependencies
- #5.9.12 (API client must exist)" \
    "${PRIORITY_HIGH_ID},${TYPE_FRONTEND_ID},${AREA_FRONTEND_ID},${STAGE_5_ID}"

create_open_issue \
    "5.9.15: Update registration page with invitation token input" \
    "## Description
Update the registration page to handle invitation tokens.

## Acceptance Criteria
- [ ] Update \`RegisterPage.tsx\` to accept \`?invite=TOKEN\` query parameter
- [ ] Pre-fill invitation token field from query param
- [ ] Show invitation token input field
- [ ] Validate invitation format (XXXX-XXXX-XXXX-XXXX)
- [ ] Call \`invitationApi.validateInvitation()\` on blur (optional UX)
- [ ] Include invitation token in registration request
- [ ] Handle \"invitation required\" error from backend
- [ ] Show helpful error if invitation invalid/expired
- [ ] Dev mode: Make field optional with info message
- [ ] Production mode: Make field required

## Invitation Field
\`\`\`tsx
<FormField
  control={form.control}
  name=\"invitation_token\"
  render={({ field }) => (
    <FormItem>
      <FormLabel>
        Invitation Code {mode === 'production' && '*'}
      </FormLabel>
      <FormControl>
        <Input
          placeholder=\"XXXX-XXXX-XXXX-XXXX\"
          {...field}
          maxLength={19}
        />
      </FormControl>
      <FormDescription>
        {mode === 'dev' 
          ? 'Optional in development mode'
          : 'Required - Get an invitation from a community manager'
        }
      </FormDescription>
      <FormMessage />
    </FormItem>
  )}
/>
\`\`\`

## Dependencies
- #5.9.10 (auth-service must support invitation validation)
- #5.9.12 (API client must exist)" \
    "${PRIORITY_MEDIUM_ID},${TYPE_FRONTEND_ID},${AREA_FRONTEND_ID},${STAGE_5_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 5.9 issues created successfully!"
echo "=========================================="
echo ""
echo "Summary:"
echo "- Database Migration: 3 issues (#5.9.1-5.9.3)"
echo "- Invitation Service Backend: 6 issues (#5.9.4-5.9.9)"
echo "- Auth Service Integration: 2 issues (#5.9.10-5.9.11)"
echo "- Frontend: 4 issues (#5.9.12-5.9.15)"
echo "Total: 15 issues"
echo ""
echo "⚠️  NOTE: Make sure label 'area: invitation service' (ID 30) exists in Forgejo"
echo "   Create it if needed with: ./create-label.sh"
