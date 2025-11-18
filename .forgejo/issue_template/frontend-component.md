---
name: Frontend Component
about: Track implementation of a frontend component or page
title: 'Frontend: [COMPONENT_NAME]'
labels: 'type/feature,area/frontend-app'
---

## 🎯 Component Overview

**Component Name:** `[ComponentName]`  
**Type:** <!-- Page / Component / Hook / Utility -->  
**Location:** `src/[pages|components|hooks|lib]/[ComponentName].tsx`  
**Stage:** <!-- Phase 1 Stage number (5 or 12) -->  
**Priority:** <!-- High / Medium / Low -->

## 📋 Component Specification

**Purpose:**
<!-- What does this component do? -->

**User Story:**
As a [type of user], I want [goal] so that [benefit].

## 🎨 Design

**Figma/Mockup:**
<!-- Link to design mockup if available -->

**Visual Description:**
<!-- Describe the component's appearance and behavior -->

**Shadcn Components Used:**

- [ ] Button
- [ ] Card
- [ ] Dialog
- [ ] Form
- [ ] Input
- [ ] Select
- [ ] Other: ___________

## 🔌 Props/API

```typescript
interface ComponentNameProps {
  // Define props here
  prop1: string;
  prop2?: number;
  onAction?: () => void;
}
```

## 🏗️ Implementation Checklist

### 1. Component Setup

- [ ] Create component file
- [ ] Define TypeScript interfaces
- [ ] Add prop validation
- [ ] Export component

### 2. UI Implementation

- [ ] Build component structure
- [ ] Add Tailwind styling
- [ ] Integrate shadcn components
- [ ] Add responsive design
- [ ] Test on mobile/tablet/desktop

### 3. State Management

- [ ] Add local state (useState)
- [ ] Add Zustand store (if needed)
- [ ] Add TanStack Query (for data fetching)
- [ ] Handle loading states
- [ ] Handle error states

### 4. API Integration

- [ ] Create API service functions
- [ ] Add TanStack Query hooks
- [ ] Handle data mutations
- [ ] Add optimistic updates
- [ ] Handle API errors

### 5. Form Handling (if applicable)

- [ ] Add react-hook-form
- [ ] Add zod validation schema
- [ ] Add form fields
- [ ] Add error messages
- [ ] Add submit handler

### 6. Routing (if page)

- [ ] Add route to router.tsx
- [ ] Add route file in `src/routes/`
- [ ] Add navigation links
- [ ] Add breadcrumbs
- [ ] Test navigation

### 7. Accessibility

- [ ] Add ARIA labels
- [ ] Add keyboard navigation
- [ ] Test with screen reader
- [ ] Add focus management
- [ ] Test color contrast

### 8. Testing

- [ ] Unit tests with Vitest
- [ ] Component tests with Testing Library
- [ ] Test user interactions
- [ ] Test error scenarios
- [ ] Test responsive behavior

### 9. Documentation

- [ ] Add JSDoc comments
- [ ] Add Storybook story (optional)
- [ ] Update component docs
- [ ] Add usage examples

## 📊 State & Data Flow

**Data Sources:**

- Local state:
- Zustand store:
- API endpoint:
- URL params:

**Data Flow:**

```
User Action → Component → [State/API] → Update UI
```

## 🔗 Dependencies

**API Endpoints:**

- `GET /api/v1/[endpoint]`
- `POST /api/v1/[endpoint]`

**Other Components:**

- `ComponentA` -
- `ComponentB` -

**Services:**

- Backend service:

**Related Issues:**

- Depends on #
- Related to #

## 🎯 Acceptance Criteria

- [ ] Component renders correctly
- [ ] All props work as expected
- [ ] Responsive on mobile/tablet/desktop
- [ ] Accessible (ARIA, keyboard navigation)
- [ ] Tests passing with >80% coverage
- [ ] Error handling works
- [ ] Loading states implemented
- [ ] Integrated with backend API
- [ ] Code reviewed and approved

## 💡 Implementation Notes

<!-- Add any specific notes, decisions, or considerations -->

**Design Decisions:**
-

**Technical Choices:**
-

**Edge Cases:**
-

## 🎨 Styling Guidelines

**Theme:**

```typescript
// Use theme variables
className="bg-primary text-primary-foreground"
className="border border-border rounded-md"
```

**Responsive:**

```typescript
// Mobile-first approach
className="flex flex-col md:flex-row lg:gap-4"
```

## ✔️ Checklist

- [ ] I have checked existing components for reusability
- [ ] I have reviewed design mockups/guidelines
- [ ] I have defined all props and interfaces
- [ ] I have planned state management
- [ ] I have identified API dependencies
- [ ] I have added accessibility requirements
- [ ] I have added appropriate labels
