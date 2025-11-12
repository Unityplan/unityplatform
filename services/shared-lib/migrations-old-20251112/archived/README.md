# Archived Migrations

This directory contains database migrations that were created during early development and have been superseded by the consolidated MVP core schema migration.

## Archived Migrations

- **20251108000001_global_schema** - Initial global schema design
- **20251108000002_territory_schema** - Initial territory schema design
- **20251108000003_seed_data_dk** - Denmark territory seed data
- **20251108000004_user_profiles_and_connections** - User profiles and social connections
- **20251108000005_auto_create_user_profile** - Automated profile creation trigger
- **20251110000001_global_invitation_registry** - Global invitation token registry

## Reason for Archival

These migrations were replaced by:

- `20251111000001_mvp_core_schema.sql` - Comprehensive MVP Phase 1 schema

The new migration incorporates all functionality from the archived migrations with:

- Updated database schema design based on finalized requirements
- Groups/bubbles system for badge-based access control
- Complete social features (notifications, events, connections, moderation)
- Territory and guild community types
- Comprehensive indexing strategy

## Date Archived

November 11, 2025

## Status

These migrations should **NOT** be run on any database. They are kept for historical reference only.
