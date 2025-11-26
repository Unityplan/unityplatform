# Community Types

This document describes the different types of communities in the Unity Platform and when to use each type.

## Overview

The Unity Platform supports 5 community types, organized into three categories:

### Physical Communities (Geographic)

| Type | Icon | Color | Description |
|------|------|-------|-------------|
| **Zone** | 🗺️ Map | Blue | Large geographic regions (countries, districts, cities) |
| **Neighborhood** | 👥 Users | Green | Local physical communities where people gather |

### Organizational Communities (Interest-based)

| Type | Icon | Color | Description |
|------|------|-------|-------------|
| **Guild** | 🔨 Hammer | Amber | Skill-based communities for practitioners |
| **Study Group** | 📖 Book | Purple | Learning-focused communities for education |

### Container Communities

| Type | Icon | Color | Description |
|------|------|-------|-------------|
| **Group** | 📦 Package | Orange | Container for organizing related communities |

---

## Detailed Descriptions

### Zone (Physical)

**Purpose:** Structural geographic organization

**Examples:**

- Denmark (country)
- Copenhagen (city)
- Northern Jutland (region)

**Characteristics:**

- Must have a territory_id
- Can contain other Zones, Neighborhoods, and any organizational communities
- Typically has geographic boundaries (coverage_area)
- No direct members (users join child communities)
- Always requires Code of Conduct badge

**Parent Rules:**

- Root zones have no parent
- Child zones can only have Zone or Neighborhood parents

---

### Neighborhood (Physical)

**Purpose:** Local physical communities where people actually meet

**Examples:**

- Nørrebro
- Vesterbro
- Christianshavn

**Characteristics:**

- Represents a physical location where people gather
- Must have a Zone or Neighborhood parent
- Can have members who are physically present in the area
- Typically has circular coverage area (center + radius)
- Restricted to Code of Conduct badge only

**Parent Rules:**

- Must have a Zone or Neighborhood parent
- Can contain Guilds, Study Groups, and Groups

---

### Guild (Organizational)

**Purpose:** Skill-based communities for practitioners of a craft

**Examples:**

- Permaculture Guild
- Rust Developers
- Urban Farming Guild
- Holochain Architects

**Characteristics:**

- Focused on a specific skill, craft, or profession
- Members are practitioners who share knowledge and collaborate
- Can have badge requirements beyond Code of Conduct
- Can be attached to any parent community type

**Parent Rules:**

- Can have any community type as parent
- Typically attached to Zones for global reach or Neighborhoods for local chapters
- Cannot contain other communities (leaf node)

---

### Study Group (Organizational)

**Purpose:** Learning-focused communities for education

**Examples:**

- Rust 101 (beginner course)
- Advanced Systems (deep dive)
- Fungi Cultivation

**Characteristics:**

- Focused on learning a specific subject
- Members are students and teachers
- Often time-bounded (course duration)
- Can have badge requirements beyond Code of Conduct

**Parent Rules:**

- Can have any community type as parent
- Often children of Guilds (advanced learning) or Groups (topic collections)
- Cannot contain other communities (leaf node)

---

### Group (Container)

**Purpose:** Organize related communities into a collapsible section

**Examples:**

- Mycology (contains mushroom-related guilds and study groups)
- Sustainability (contains permaculture, urban farming, etc.)
- Blockchain (contains Holochain, Rust, etc.)

**Characteristics:**

- **Container only** - used to organize other communities
- Displayed as a collapsible card in the community flow visualization
- Can have badge requirements that gate access to all children
- Click to "drill down" and see contents

**Parent Rules:**

- Can have any community type as parent
- **Can contain** Guilds, Study Groups, and other Groups
- Children inherit badge requirements if `inherit_requirements` is true

---

## Visual Representation

In the Community Flow visualization:

```
                    ┌─────────────────┐
                    │   Code of       │
                    │   Conduct       │
                    │    (Badge)      │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
        ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
        │  Denmark  │  │  Aarhus   │  │Copenhagen │
        │   ZONE    │  │   ZONE    │  │   ZONE    │
        └─────┬─────┘  └─────┬─────┘  └─────┬─────┘
              │              │              │
     ┌────────┴────────┐     │      ┌───────┴───────┐
     │                 │     │      │               │
┌────▼────┐      ┌─────▼─────┐  ┌───▼────┐   ┌──────▼──────┐
│Permacul-│      │  Mycology │  │Nørrebro│   │  Vesterbro  │
│ture     │      │ ╔═══════╗ │  │NEIGHBOR│   │ NEIGHBORHOOD│
│ GUILD   │      │ ║ GROUP ║ │  └────────┘   └─────────────┘
└─────────┘      │ ╚═══════╝ │
                 │ 1 Guild   │
                 │ 2 Study   │
                 │ Groups    │
                 │           │
                 │ Click to  │
                 │ explore → │
                 └───────────┘
```

- **Regular communities** (Zone, Neighborhood, Guild, Study Group) are shown as solid bordered boxes
- **Group communities** are shown as dashed bordered cards with content summary
- Click on a Group to drill down and see its contents

---

## Badge Requirements & Inheritance

### Inheritance Rules

1. **Direct Requirements:** Any community can have badge requirements set directly
2. **Inherited Requirements:** If `inherit_requirements` is true, children inherit parent requirements
3. **Effective Requirements:** The `/effective-requirements` endpoint returns all requirements (direct + inherited)

### Example Inheritance Chain

```
Denmark Zone (requires: Code of Conduct)
├── Mycology Group (requires: Community Manager badge)
│   ├── Mushroom Foraging Guild (inherits: CoC + Community Manager)
│   └── Fungi Cultivation Study Group (inherits: CoC + Community Manager)
└── Permaculture Guild (requires: PDC Certificate, inherits: CoC)
```

### Visual Indicators

- Badge nodes appear between parent and child in the flow visualization
- Group cards show a 🔒 lock icon with badge name when they have requirements
- Inherited requirements are not shown as separate nodes (reduces visual clutter)

---

## When to Use Each Type

| Scenario | Recommended Type |
|----------|------------------|
| Creating a regional structure | Zone |
| Local meetup location | Neighborhood |
| Skill-based practitioner community | Guild |
| Course or learning group | Study Group |
| Organizing related communities | Group |
| Badge-gated section | Group (with requirement) |
| Topic collection | Group |

---

## API Endpoints

```
POST   /api/v1/communities              # Create (specify community_type)
GET    /api/v1/communities              # List (filter by community_type)
GET    /api/v1/communities/{id}         # Get single
GET    /api/v1/communities/{id}/group-summary    # Get Group summary
GET    /api/v1/communities/{id}/effective-requirements  # Get all requirements
```
