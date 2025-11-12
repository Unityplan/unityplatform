# event-service

**Port:** 8009  
**Version:** 0.1.0-alpha.1  
**Status:** ⏳ Planned - Phase 2  
**Bounded Context:** Events & Calendar

---

## 📋 Overview

The event-service manages events, RSVPs, event discovery, and calendar integration.

### **Responsibilities**

- ⏳ Create and manage events
- ⏳ RSVP management (going/interested/not going)
- ⏳ Event discovery (public events, community events)
- ⏳ Calendar integration (iCal export)
- ⏳ Event reminders
- ⏳ Recurring events

### **Not Responsible For**

- ❌ Event live streaming (external service)
- ❌ Ticket sales (not planned for MVP)
- ❌ Event check-in (not planned for MVP)

---

## 🗄️ Database Schema (Planned)

```sql
CREATE TABLE territory_{code}.events (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(500),
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    is_online BOOLEAN DEFAULT false,
    community_id UUID REFERENCES communities(id),
    created_by UUID REFERENCES users(id),
    visibility VARCHAR(20) DEFAULT 'public',
    max_attendees INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE territory_{code}.event_rsvps (
    event_id UUID REFERENCES events(id),
    user_id UUID REFERENCES users(id),
    status VARCHAR(20) DEFAULT 'going',  -- going/interested/not_going
    rsvp_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (event_id, user_id)
);
```

---

## 🔌 API Endpoints (Planned)

- POST /v1/events - Create event
- GET /v1/events - List events
- GET /v1/events/{id} - Get event details
- PATCH /v1/events/{id} - Update event
- DELETE /v1/events/{id} - Delete event
- POST /v1/events/{id}/rsvp - RSVP to event
- GET /v1/events/{id}/attendees - List attendees
- GET /v1/events/calendar.ics - Export calendar (iCal)

---

## 📡 NATS Events (Planned)

**Published:**

- event.created
- event.updated
- event.cancelled
- event.rsvp_updated

**Subscribed:**

- community.deleted (cancel community events)

---

## 🔮 Holochain Migration (Future)

```rust
#[hdk_entry_helper]
struct Event {
    title: String,
    description: String,
    location: Option<String>,
    start_time: Timestamp,
    end_time: Option<Timestamp>,
    created_by: AgentPubKey,
    visibility: EventVisibility,
}
```

---

**Last Updated:** November 12, 2025  
**Service Owner:** Core Team  
**Status:** Phase 2 - Not Yet Started  
**Dependencies:** community-service, notification-service
