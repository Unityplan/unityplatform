# event-service API Endpoints

**Base URL:** `http://localhost:8009`  
**Version:** v1  
**Status:** 📋 Planned (Future Phase)

---

## Event Management

### 1. Create Event

**Endpoint:** `POST /api/v1/events`  
**Authentication:** Bearer token required  
**Status:** 📋 Planned

**Request:**

```json
{
  "title": "Rust Copenhagen Meetup",
  "description": "Monthly meetup for Rust developers",
  "location": "Copenhagen",
  "event_type": "in_person",
  "start_time": "2025-12-01T18:00:00Z",
  "end_time": "2025-12-01T21:00:00Z",
  "max_attendees": 50,
  "community_id": "uuid"
}
```

**Event Types:**

- `online` - Virtual event (includes video link)
- `in_person` - Physical location
- `hybrid` - Both online and in-person

---

### 2. List Events

**Endpoint:** `GET /api/v1/events`  
**Query:** `upcoming=true`, `community_id=uuid`, `type=in_person`  
**Status:** 📋 Planned

---

### 3. RSVP to Event

**Endpoint:** `POST /api/v1/events/{id}/rsvp`  
**Status:** 📋 Planned

**Request:**

```json
{
  "status": "going"
}
```

**RSVP Status:**

- `going` - Will attend
- `interested` - Maybe attend
- `not_going` - Not attending

---

### 4. Get Attendees

**Endpoint:** `GET /api/v1/events/{id}/attendees`  
**Status:** 📋 Planned

---

### 5. Export to Calendar

**Endpoint:** `GET /api/v1/events/{id}/ical`  
**Status:** 📋 Planned  
**Format:** iCalendar (.ics) file

---

**Last Updated:** November 12, 2025  
**Implementation Status:** Future phase
