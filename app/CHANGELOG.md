# Frontend Changelog

All notable changes to the Unity Platform frontend application will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned - Stage 5: Frontend Auth & Profile (Phase 1 MVP)

- **Authentication Pages:**
  - Login page with react-hook-form + zod validation
  - Registration page with two-step invitation validation
  - Password reset flow
- **Profile Management:**
  - Profile view page (display name, bio, location, website, links)
  - Profile edit page with image upload
  - Profile links management (max 10)
  - Language proficiency management (4-dimensional skills)
- **Settings Pages:**
  - General settings (theme, language, timezone)
  - Privacy settings (profile visibility, show email/location, allow messages)
  - Notification settings (email, badge, course, forum, marketing)
- **Protected Routes:**
  - Route guard component (AuthGuard)
  - TanStack Router with file-based routing
  - Dashboard and authenticated pages
- **State Management:**
  - Auth store (Zustand) with token persistence
  - UI store for theme and preferences
- **API Integration:**
  - API client with token refresh interceptor
  - TanStack Query for data fetching/caching
  - Integration with auth-service (7 endpoints)
  - Integration with user-service (24 endpoints)

### Planned - Stage 12: Course & Forum UI (Phase 1 MVP)

- Course catalog and detail pages
- Lesson viewer with progress tracking
- Quiz interface with validation
- Forum category and topic listing
- Topic view with posts and replies
- Moderation dashboard

---

## [0.0.0] - 2025-11-14

**Release Stage:** Pre-Alpha (Project Scaffolding)

### Added - Initial Setup

- Vite + React + TypeScript project scaffolding
- TanStack Router v1.134.10 (type-safe routing)
- TanStack Query v5 (data fetching and caching)
- Zustand (state management for auth/UI only)
- React Hook Form + Zod (form handling and validation)
- shadcn/ui v3.5.0 (accessible component library)
- TailwindCSS v4.1.16 (utility-first styling)

### Infrastructure

- Vite 5.x development server
- TypeScript configuration (strict mode)
- ESLint configuration
- Vitest for unit testing
- Testing Library for component testing
- Environment variable setup (.env.development, .env.production)

### Project Structure

- `/src/api/` - API client functions
- `/src/components/` - Reusable UI components
- `/src/pages/` - Page components
- `/src/routes/` - TanStack Router routes
- `/src/stores/` - Zustand state stores
- `/src/lib/` - Utility functions
- `/src/hooks/` - Custom React hooks
- `/src/types/` - TypeScript type definitions
- `/src/styles/` - Global styles and theme
- `/public/` - Static assets

### Design System

- shadcn/ui components with OKLCH theming
- Accessible, keyboard-navigable components
- Dark/light/system theme support
- Responsive design (mobile-first)

### Dependencies - Core

- React 19.2.0
- Vite 5.x
- TypeScript (latest)
- @tanstack/react-router 1.134.10
- @tanstack/react-query 5.90.7
- zustand (latest)
- react-hook-form (latest)
- zod (latest)

### Dependencies - UI

- @radix-ui/* components
- tailwindcss 4.1.16
- @tailwindcss/vite 4.1.16
- lucide-react (icons)
- class-variance-authority
- clsx + tailwind-merge

### Dependencies - Testing

- vitest (latest)
- @testing-library/react (latest)
- @testing-library/jest-dom (latest)
- @vitejs/plugin-react (latest)

### Configuration

- PostCSS with TailwindCSS v4
- TanStack Router config (tsr.config.json)
- Vite config with React plugin
- TypeScript strict mode enabled
- ESLint with TypeScript and React plugins

### Notes

- Frontend not yet connected to backend APIs
- No components implemented yet
- Project structure ready for Stage 5 implementation
- Stack chosen for stability (React 18) during MVP phase
- TanStack Query handles all server data (not Zustand)
- Zustand only for auth tokens and UI state

---

**Application:** Unity Platform App  
**Location:** app/  
**Framework:** React + Vite + TypeScript  
**Version:** 0.0.0 (Pre-Alpha)
