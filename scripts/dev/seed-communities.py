#!/usr/bin/env python3
import requests
import json
import time
import sys

# Configuration
AUTH_SERVICE_URL = "http://localhost:8001/api/v1/auth"
COMMUNITY_SERVICE_URL = "http://localhost:8006/api/v1/communities"
TERRITORY_CODE = "dk"

# Seed User (Alice Admin - Platform Manager)
SEED_USER = {
    "username": "alice_admin",
    "password": "SecurePass123!",
    "email": "alice@unityplatform.test",
    "territory": TERRITORY_CODE
}

def log(msg):
    print(f"[Seed] {msg}")

def get_token():
    # Try login first
    try:
        log(f"Attempting login for {SEED_USER['username']}...")
        response = requests.post(f"{AUTH_SERVICE_URL}/login", json={
            "username": SEED_USER["username"],
            "password": SEED_USER["password"],
            "territory": SEED_USER["territory"]
        })
        
        if response.status_code == 200:
            log("Login successful.")
            return response.json()["accessToken"]
    except Exception as e:
        log(f"Login failed: {e}")

    # If login fails, try register
    try:
        log(f"Registering user {SEED_USER['username']}...")
        response = requests.post(f"{AUTH_SERVICE_URL}/register", json={
            "username": SEED_USER["username"],
            "password": SEED_USER["password"],
            "email": SEED_USER["email"],
            "territory": SEED_USER["territory"]
        })
        
        if response.status_code == 201 or response.status_code == 200:
            log("Registration successful.")
            return response.json()["accessToken"]
        else:
            log(f"Registration failed: {response.text}")
            sys.exit(1)
    except Exception as e:
        log(f"Registration error: {e}")
        sys.exit(1)

def create_community(token, data):
    headers = {"Authorization": f"Bearer {token}"}
    
    # Check if exists first (by slug)
    try:
        # We can't easily check by slug via API without listing, but create will fail with 409
        response = requests.post(COMMUNITY_SERVICE_URL, json=data, headers=headers)
        
        if response.status_code == 201:
            log(f"Created community: {data['name']}")
            return response.json()["id"]
        elif response.status_code == 409:
            log(f"Community already exists: {data['name']}")
            # Try to find it to return ID
            # This is a bit hacky, assuming list returns it
            list_resp = requests.get(COMMUNITY_SERVICE_URL, params={"search": data["slug"]}, headers=headers)
            if list_resp.status_code == 200:
                for c in list_resp.json():
                    if c["slug"] == data["slug"]:
                        return c["id"]
            return None
        else:
            log(f"Failed to create {data['name']}: {response.text}")
            return None
    except Exception as e:
        log(f"Error creating community: {e}")
        return None

def seed_communities(token):
    # 1. Root Physical Community (Denmark)
    dk_id = create_community(token, {
        "name": "Denmark",
        "slug": "denmark",
        "description": "The sovereign territory of Denmark.",
        "community_type": "zone",
        "territory_id": "dk",
        "inherit_requirements": False,
        "location_lat": 56.2639,
        "location_lng": 9.5018,
        "coverage_area": {
            "type": "polygon",
            "coordinates": [
                {"lat": 57.75, "lng": 10.6},
                {"lat": 57.75, "lng": 8.0},
                {"lat": 54.5, "lng": 8.0},
                {"lat": 54.5, "lng": 12.7},
                {"lat": 56.0, "lng": 12.7}
            ]
        }
    })

    if not dk_id:
        log("Could not get root community ID. Aborting.")
        return

    # 2. Regions/Cities
    cities = [
        {
            "name": "Copenhagen", "slug": "copenhagen", "desc": "Capital city", 
            "territory_id": "dk", "inherit_requirements": False,
            "lat": 55.6761, "lng": 12.5683,
            "coverage": {
                "type": "polygon",
                "coordinates": [
                    {"lat": 55.73, "lng": 12.45},
                    {"lat": 55.73, "lng": 12.65},
                    {"lat": 55.61, "lng": 12.65},
                    {"lat": 55.61, "lng": 12.45}
                ]
            }
        },
        {
            "name": "Aarhus", "slug": "aarhus", "desc": "City of smiles", 
            "territory_id": "dk", "inherit_requirements": False,
            "lat": 56.1629, "lng": 10.2039,
            "coverage": {
                "type": "polygon",
                "coordinates": [
                    {"lat": 56.25, "lng": 10.10},
                    {"lat": 56.25, "lng": 10.30},
                    {"lat": 56.10, "lng": 10.30},
                    {"lat": 56.10, "lng": 10.10}
                ]
            }
        },
        {
            "name": "Odense", "slug": "odense", "desc": "H.C. Andersen's hometown", 
            "territory_id": "dk", "inherit_requirements": False,
            "lat": 55.4038, "lng": 10.4024,
            "coverage": {
                "type": "polygon",
                "coordinates": [
                    {"lat": 55.45, "lng": 10.30},
                    {"lat": 55.45, "lng": 10.50},
                    {"lat": 55.35, "lng": 10.50},
                    {"lat": 55.35, "lng": 10.30}
                ]
            }
        },
        {
            "name": "Aalborg", "slug": "aalborg", "desc": "North Jutland hub", 
            "territory_id": "dk", "inherit_requirements": False,
            "lat": 57.0488, "lng": 9.9217,
            "coverage": {
                "type": "polygon",
                "coordinates": [
                    {"lat": 57.10, "lng": 9.80},
                    {"lat": 57.10, "lng": 10.05},
                    {"lat": 57.00, "lng": 10.05},
                    {"lat": 57.00, "lng": 9.80}
                ]
            }
        }
    ]

    city_ids = {}
    for city in cities:
        cid = create_community(token, {
            "name": city["name"],
            "slug": city["slug"],
            "description": city["desc"],
            "community_type": "zone",
            "territory_id": city["territory_id"],
            "parent_community_id": dk_id,
            "inherit_requirements": city["inherit_requirements"],
            "location_lat": city["lat"],
            "location_lng": city["lng"],
            "coverage_area": city["coverage"]
        })
        if cid:
            city_ids[city["slug"]] = cid

    # 3. Neighborhoods in Copenhagen
    cph_hoods = [
        {
            "name": "Nørrebro", "slug": "noerrebro", "desc": "Multicultural and vibrant",
            "lat": 55.6906, "lng": 12.5553, "radius": 1500.0
        },
        {
            "name": "Vesterbro", "slug": "vesterbro", "desc": "Hipster paradise",
            "lat": 55.6694, "lng": 12.5486, "radius": 1500.0
        },
        {
            "name": "Østerbro", "slug": "oesterbro", "desc": "Family friendly",
            "lat": 55.7083, "lng": 12.5783, "radius": 1500.0
        },
        {
            "name": "Christianshavn", "slug": "christianshavn", "desc": "Canals and history",
            "lat": 55.6722, "lng": 12.5917, "radius": 1000.0
        }
    ]

    for hood in cph_hoods:
        if "copenhagen" in city_ids:
            create_community(token, {
                "name": hood["name"],
                "slug": hood["slug"],
                "description": hood["desc"],
                "community_type": "neighborhood",
                "territory_id": "dk",
                "parent_community_id": city_ids["copenhagen"],
                "inherit_requirements": False,
                "location_lat": hood["lat"],
                "location_lng": hood["lng"],
                "coverage_area": {
                    "type": "circle",
                    "center": {"lat": hood["lat"], "lng": hood["lng"]},
                    "radius": hood["radius"]
                }
            })

    # 4. Guilds (Interest based, can be global or attached to root)
    guilds = [
        {"name": "Permaculture Guild", "slug": "permaculture-guild", "desc": "Sustainable living practices"},
        {"name": "Rust Developers", "slug": "rust-devs", "desc": "Ferris fans unite"},
        {"name": "Holochain Architects", "slug": "holochain-arch", "desc": "Building the distributed web"},
        {"name": "Urban Farming", "slug": "urban-farming", "desc": "Growing food in the city"}
    ]

    for guild in guilds:
        create_community(token, {
            "name": guild["name"],
            "slug": guild["slug"],
            "description": guild["desc"],
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": dk_id, # Attached to Denmark for now
            "inherit_requirements": False
        })

    # 5. Study Groups
    study_groups = [
        {"name": "Rust 101", "slug": "rust-101", "desc": "Beginner Rust course"},
        {"name": "Advanced Systems", "slug": "adv-systems", "desc": "Deep dive into systems programming"},
    ]

    for group in study_groups:
        create_community(token, {
            "name": group["name"],
            "slug": group["slug"],
            "description": group["desc"],
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": dk_id,
            "inherit_requirements": False
        })

    # 6. Groups (Container communities)
    # Mycology is now a Group that contains related communities
    mycology_id = create_community(token, {
        "name": "Mycology",
        "slug": "mycology-group",
        "description": "Container for all fungi-related communities",
        "community_type": "group",
        "territory_id": "dk",
        "parent_community_id": dk_id,
        "inherit_requirements": False
    })

    # Create child communities inside the Mycology group
    if mycology_id:
        create_community(token, {
            "name": "Mushroom Foraging",
            "slug": "mushroom-foraging",
            "description": "Finding and identifying wild mushrooms",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": mycology_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Fungi Cultivation",
            "slug": "fungi-cultivation",
            "description": "Growing mushrooms at home and in farms",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": mycology_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Medicinal Mushrooms",
            "slug": "medicinal-mushrooms",
            "description": "Research and discussion on therapeutic fungi",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": mycology_id,
            "inherit_requirements": True
        })

    # 7. Sub-communities for Guilds (Chapters)
    # Assuming we can find the guild IDs, but for simplicity let's just create some attached to cities
    if "aarhus" in city_ids:
        create_community(token, {
            "name": "Aarhus Rust Meetup",
            "slug": "aarhus-rust",
            "description": "Local Rust chapter",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["aarhus"],
            "inherit_requirements": True
        })

    log("Seeding complete!")

if __name__ == "__main__":
    token = get_token()
    if token:
        seed_communities(token)
