#!/bin/bash
set -e

# Load environment variables
source .env

# Label IDs
PRIORITY_HIGH_ID=1
PRIORITY_MEDIUM_ID=2
TYPE_INFRASTRUCTURE_ID=8
AREA_TESTING_ID=24

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
    else
        echo "❌ Failed to create issue: ${title}"
        echo "$response" | jq .
    fi
    
    sleep 0.5
}

echo "=========================================="
echo "Creating Stage 13 open issues (Testing, Documentation & Deployment)..."
echo "=========================================="
echo ""

# Stage 13: Testing, Documentation & Deployment - 11 tasks total

create_open_issue \
    "Stage 13.1: Backend unit tests (80%+ coverage)" \
    "## Description
Write comprehensive unit tests for all backend services to achieve 80%+ code coverage.

## Services to Test
- auth-service
- user-service
- badge-service
- territory-service
- utility-service
- course-service
- matrix-bridge
- ipfs-service
- forum-service
- translation-service

## Tasks
- [ ] Set up test infrastructure with cargo test
- [ ] Write handler tests for all endpoints
- [ ] Write service layer tests (business logic)
- [ ] Write model/validation tests
- [ ] Mock database and external dependencies
- [ ] Generate coverage reports (tarpaulin or cargo-llvm-cov)
- [ ] Achieve 80%+ coverage for each service

## Acceptance Criteria
- [ ] All services have >80% test coverage
- [ ] Tests run in CI pipeline
- [ ] Coverage reports generated
- [ ] All edge cases tested
- [ ] Error handling tested

## Technical Notes
- Use cargo test for running tests
- Use cargo-tarpaulin or cargo-llvm-cov for coverage
- Mock NATS, Redis, PostgreSQL connections
- Use test fixtures for database setup
- Test both success and error paths

## Dependencies
- Stage 12: Frontend Complete" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.2: Integration tests for API endpoints" \
    "## Description
Write integration tests for all API endpoints testing full request/response cycles.

## Tasks
- [ ] Set up integration test environment
- [ ] Use testcontainers for PostgreSQL, Redis, NATS
- [ ] Test all API endpoints with real databases
- [ ] Test authentication flows
- [ ] Test authorization (permissions)
- [ ] Test CORS and rate limiting
- [ ] Test error responses (4xx, 5xx)
- [ ] Test pagination and filtering

## Acceptance Criteria
- [ ] All API endpoints tested with integration tests
- [ ] Database transactions tested
- [ ] Authentication/authorization tested
- [ ] Rate limiting verified
- [ ] Error cases handled
- [ ] Tests run in isolated containers

## Technical Notes
- Use testcontainers-rs for Docker containers
- Separate test database per test suite
- Clean up data between tests
- Test with real NATS, Redis connections
- Verify HTTP status codes and response bodies

## Dependencies
- Stage 13.1: unit tests" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.3: Frontend unit and component tests" \
    "## Description
Write unit and component tests for frontend React components.

## Tasks
- [ ] Set up Vitest testing environment
- [ ] Write tests for utility functions
- [ ] Write tests for hooks (useAuth, etc)
- [ ] Write component tests with Testing Library
- [ ] Test form validation logic
- [ ] Test API integration hooks
- [ ] Mock API responses with MSW
- [ ] Achieve 70%+ coverage

## Acceptance Criteria
- [ ] All utility functions tested
- [ ] All custom hooks tested
- [ ] All components have basic tests
- [ ] Form validation tested
- [ ] API mocking working
- [ ] Coverage >70%

## Technical Notes
- Use Vitest for test runner
- Use Testing Library for component tests
- Use MSW for API mocking
- Test user interactions (clicks, inputs)
- Test error states and loading states

## Dependencies
- Stage 13.2: integration tests" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.4: End-to-end tests for critical flows" \
    "## Description
Write E2E tests for critical user flows using Playwright or similar.

## Critical Flows to Test
1. User registration and login
2. Profile creation and editing
3. Course enrollment and completion
4. Quiz taking and badge earning
5. Forum topic creation and posting
6. File upload to IPFS
7. Translation usage

## Tasks
- [ ] Set up Playwright for E2E testing
- [ ] Write registration/login flow test
- [ ] Write profile management test
- [ ] Write course completion flow test
- [ ] Write forum interaction test
- [ ] Write file upload test
- [ ] Configure test database seeding
- [ ] Run tests in CI pipeline

## Acceptance Criteria
- [ ] All critical flows tested E2E
- [ ] Tests run against local environment
- [ ] Tests verify UI and backend integration
- [ ] Screenshots on failure
- [ ] Tests run in CI

## Technical Notes
- Use Playwright for E2E testing
- Run against docker-compose environment
- Seed database with test data
- Clean up after tests
- Take screenshots on failures
- Video recording for debugging

## Dependencies
- Stage 13.3: frontend tests" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.5: Load and performance testing" \
    "## Description
Perform load testing to validate system performance under realistic load.

## Load Test Scenarios
1. Concurrent user logins (100 users)
2. Course page loads (500 req/sec)
3. Forum browsing (200 concurrent users)
4. API endpoint stress test
5. Database query performance
6. Redis cache hit rates

## Tasks
- [ ] Set up load testing tools (k6 or Artillery)
- [ ] Write load test scripts for each scenario
- [ ] Test authentication endpoints
- [ ] Test course/forum read operations
- [ ] Test write operations (posts, enrollments)
- [ ] Monitor database performance
- [ ] Monitor Redis performance
- [ ] Generate performance reports

## Acceptance Criteria
- [ ] System handles 100 concurrent users
- [ ] API response times <200ms (p95)
- [ ] Database queries optimized
- [ ] Redis cache hit rate >80%
- [ ] No memory leaks detected
- [ ] Performance reports generated

## Technical Notes
- Use k6 or Artillery for load testing
- Monitor with Prometheus/Grafana
- Test realistic user behavior
- Gradually increase load
- Identify bottlenecks
- Document performance baselines

## Dependencies
- Stage 13.4: E2E tests" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.6: Security testing and vulnerability scanning" \
    "## Description
Perform security testing and vulnerability scanning on the platform.

## Security Tests
- [ ] OWASP Top 10 vulnerability checks
- [ ] SQL injection testing
- [ ] XSS testing
- [ ] CSRF protection verification
- [ ] Authentication bypass attempts
- [ ] Authorization bypass attempts
- [ ] Dependency vulnerability scanning
- [ ] SSRF protection testing
- [ ] Rate limiting effectiveness

## Tasks
- [ ] Run cargo audit for Rust dependencies
- [ ] Run npm audit for frontend dependencies
- [ ] Use OWASP ZAP or similar for scanning
- [ ] Test authentication security
- [ ] Test authorization rules
- [ ] Verify HTTPS/TLS configuration
- [ ] Test input validation
- [ ] Generate security report

## Acceptance Criteria
- [ ] No critical vulnerabilities found
- [ ] All dependencies up to date
- [ ] OWASP Top 10 protected
- [ ] Security headers configured
- [ ] Input validation comprehensive
- [ ] Security report documented

## Technical Notes
- Use cargo audit for Rust
- Use npm audit for Node.js
- Use OWASP ZAP for web scanning
- Test with real attack vectors
- Document findings and fixes
- Regular security updates

## Dependencies
- Stage 13.5: load testing" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.7: API documentation (OpenAPI/Swagger)" \
    "## Description
Generate comprehensive API documentation for all services using OpenAPI/Swagger.

## Tasks
- [ ] Add OpenAPI annotations to all endpoints
- [ ] Generate OpenAPI spec for each service
- [ ] Set up Swagger UI for interactive docs
- [ ] Document request/response schemas
- [ ] Document authentication requirements
- [ ] Document error responses
- [ ] Add example requests/responses
- [ ] Deploy API docs to /docs endpoint

## Acceptance Criteria
- [ ] All endpoints documented
- [ ] OpenAPI 3.0 spec generated
- [ ] Swagger UI accessible
- [ ] Request/response examples included
- [ ] Authentication documented
- [ ] Error codes documented

## Technical Notes
- Use utoipa crate for OpenAPI in Rust
- Generate spec automatically from code
- Host Swagger UI in frontend
- Include curl examples
- Document rate limits
- Version API docs

## Dependencies
- Stage 13.6: security testing" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.8: Developer documentation" \
    "## Description
Create comprehensive developer documentation for onboarding and contribution.

## Documentation to Create
- [ ] Getting started guide
- [ ] Architecture overview
- [ ] Service development guide
- [ ] Frontend development guide
- [ ] Database migration guide
- [ ] Testing guide
- [ ] Deployment guide
- [ ] Contribution guidelines
- [ ] Code style guide

## Acceptance Criteria
- [ ] All guides complete and accurate
- [ ] Code examples included
- [ ] Troubleshooting sections
- [ ] Screenshots/diagrams where helpful
- [ ] Table of contents
- [ ] Published to docs/ directory

## Technical Notes
- Use Markdown for all docs
- Include architecture diagrams
- Document common issues
- Link to API docs
- Keep docs in sync with code
- Version docs with releases

## Dependencies
- Stage 13.7: API docs" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.9: User documentation and help guides" \
    "## Description
Create user-facing documentation and help guides for platform users.

## Documentation to Create
- [ ] Platform overview
- [ ] Registration and login guide
- [ ] Profile management guide
- [ ] Course enrollment guide
- [ ] Forum usage guide
- [ ] Badge system explanation
- [ ] Community guidelines
- [ ] Privacy policy
- [ ] Terms of service
- [ ] FAQ

## Acceptance Criteria
- [ ] All user guides complete
- [ ] Written for non-technical users
- [ ] Screenshots included
- [ ] Step-by-step instructions
- [ ] Searchable help center
- [ ] Multiple languages (future)

## Technical Notes
- Use simple language
- Include screenshots
- Video tutorials optional
- Make searchable
- Link from UI help buttons
- Track popular help topics

## Dependencies
- Stage 13.8: developer docs" \
    "${PRIORITY_MEDIUM_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.10: Production deployment configuration" \
    "## Description
Create production-ready deployment configuration and infrastructure.

## Tasks
- [ ] Create production docker-compose.yml
- [ ] Configure environment variables for production
- [ ] Set up HTTPS/TLS certificates
- [ ] Configure production database (backups, replication)
- [ ] Set up production Redis (persistence)
- [ ] Configure NATS clustering
- [ ] Set up log aggregation
- [ ] Configure monitoring (Prometheus, Grafana)
- [ ] Set up backup strategy
- [ ] Create deployment scripts

## Acceptance Criteria
- [ ] Production docker-compose ready
- [ ] HTTPS configured
- [ ] Database backups automated
- [ ] Monitoring dashboards working
- [ ] Log aggregation functional
- [ ] Deployment scripts tested
- [ ] Rollback procedure documented

## Technical Notes
- Use Let's Encrypt for TLS
- PostgreSQL streaming replication
- Redis RDB + AOF persistence
- NATS clustering for HA
- Prometheus + Grafana for monitoring
- Loki for log aggregation
- Automated daily backups

## Dependencies
- Stage 13.9: user documentation" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

create_open_issue \
    "Stage 13.11: CI/CD pipeline setup" \
    "## Description
Set up automated CI/CD pipeline for testing, building, and deployment.

## Pipeline Stages
1. Build: Compile Rust services and frontend
2. Test: Run unit, integration, E2E tests
3. Lint: Run cargo clippy and eslint
4. Security: Run cargo audit and npm audit
5. Build Docker images
6. Push to registry (optional)
7. Deploy to staging (optional)
8. Deploy to production (manual approval)

## Tasks
- [ ] Create GitHub Actions workflow
- [ ] Set up build jobs for all services
- [ ] Run all tests in CI
- [ ] Run linting and formatting checks
- [ ] Generate test coverage reports
- [ ] Build Docker images
- [ ] Set up deployment jobs
- [ ] Configure secrets management

## Acceptance Criteria
- [ ] CI runs on every push
- [ ] All tests run automatically
- [ ] Coverage reports generated
- [ ] Docker images built
- [ ] Deployment automated (staging)
- [ ] Production deployment requires approval
- [ ] Pipeline completes in <15 minutes

## Technical Notes
- Use GitHub Actions
- Cache dependencies for speed
- Parallel test execution
- Matrix builds for services
- Separate staging/production jobs
- Use GitHub Container Registry
- Notify on failures

## Dependencies
- Stage 13.10: production config" \
    "${PRIORITY_HIGH_ID},${TYPE_INFRASTRUCTURE_ID},${AREA_TESTING_ID}"

echo ""
echo "=========================================="
echo "✅ All Stage 13 issues created!"
echo "=========================================="
echo ""
echo "Summary: Created 11 open issues for Stage 13 (Testing, Documentation & Deployment)"
echo ""
