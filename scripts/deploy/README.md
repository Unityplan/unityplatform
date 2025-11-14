# Deployment Scripts

Scripts for deploying Unity Platform pods (production environments).

## Scripts

### deploy-multi-pod.sh

Deploy all production pods (Denmark, Norway, Sweden, Europe).

**Usage:**

```bash
./deploy/deploy-multi-pod.sh [--clean]
```

**Options:**

- `--clean` - Clean deployment (removes existing volumes)

**Deploys:**

- Denmark pod (DK) - `unityplatform_dk`
- Norway pod (NO) - `unityplatform_no`
- Sweden pod (SE) - `unityplatform_se`
- Europe pod (EU) - `unityplatform_de`, `unityplatform_fr`, `unityplatform_es`

---

### verify-multi-pod.sh

Verify multi-pod deployment health and connectivity.

**Usage:**

```bash
./deploy/verify-multi-pod.sh
```

**Checks:**

- All pod services running
- Health endpoints responding
- Database connectivity
- NATS messaging
- Service mesh connectivity

## CHANGELOG

### [Unreleased]

### [1.0.0] - 2025-11-14

#### Added

- Organized deployment scripts into `deploy/` subdirectory

#### Changed

- **BREAKING**: Scripts moved from `scripts/` root to `scripts/deploy/`

---

**Category**: Deployment & Production  
**Maintainer**: Unity Platform Team  
**Last Updated**: 2025-11-14
