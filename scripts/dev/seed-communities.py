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

    # 4. Guilds (Interest-based practice communities)
    guilds = [
        {"name": "Sovereignty & Self-Governance", "slug": "sovereignty-guild", 
         "desc": "Practices for personal and collective sovereignty, consent-based decision making, and self-determination"},
        {"name": "Earth Regeneration", "slug": "earth-regeneration", 
         "desc": "Restoring ecosystems, rewilding, and healing damaged landscapes"},
        {"name": "Syntropic Agroforestry", "slug": "syntropic-agroforestry", 
         "desc": "Practicing Ernst Götsch's methods of regenerative agriculture through forest succession"},
        {"name": "Clean Water Stewardship", "slug": "clean-water-stewards", 
         "desc": "Protecting, purifying, and restoring water sources and watersheds"},
        {"name": "Seed Keepers", "slug": "seed-keepers", 
         "desc": "Preserving heirloom seeds, seed saving, and maintaining genetic diversity"},
        {"name": "Natural Building", "slug": "natural-building", 
         "desc": "Building with earth, straw, timber, and natural materials"},
        {"name": "Holistic Health Practitioners", "slug": "holistic-health", 
         "desc": "Natural medicine, herbalism, bodywork, and preventive health practices"},
        {"name": "Food Sovereignty", "slug": "food-sovereignty", 
         "desc": "Local food systems, community gardens, and food independence"}
    ]

    guild_ids = {}
    for guild in guilds:
        gid = create_community(token, {
            "name": guild["name"],
            "slug": guild["slug"],
            "description": guild["desc"],
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": dk_id,
            "inherit_requirements": False
        })
        if gid:
            guild_ids[guild["slug"]] = gid

    # 5. Study Groups (Learning-focused communities)
    study_groups = [
        {"name": "Introduction to Permaculture Design", "slug": "permaculture-intro", 
         "desc": "72-hour PDC curriculum covering ethics, principles, and design methodology"},
        {"name": "Nonviolent Communication Circle", "slug": "nvc-circle", 
         "desc": "Learning Marshall Rosenberg's NVC for compassionate communication"},
        {"name": "Water Harvesting & Retention", "slug": "water-harvesting", 
         "desc": "Designing swales, ponds, and water retention landscapes"},
        {"name": "Fermentation Fundamentals", "slug": "fermentation-fundamentals", 
         "desc": "Learning the art of fermented foods for gut health and preservation"},
        {"name": "Trauma-Informed Facilitation", "slug": "trauma-informed-facilitation", 
         "desc": "Creating safe spaces and understanding nervous system regulation"},
        {"name": "Mushroom Cultivation Basics", "slug": "mushroom-cultivation", 
         "desc": "Growing gourmet and medicinal mushrooms at home"},
        {"name": "Herbal Medicine Making", "slug": "herbal-medicine", 
         "desc": "Creating tinctures, salves, and herbal preparations"},
        {"name": "Sociocracy & Consent Decision Making", "slug": "sociocracy-study", 
         "desc": "Learning circular governance and consent-based organizational structures"}
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

    # 6. Groups (Container communities for related topics)
    
    # Healing & Wellness Group
    healing_id = create_community(token, {
        "name": "Healing & Wellness",
        "slug": "healing-wellness",
        "description": "Container for all healing modalities and wellness practices",
        "community_type": "group",
        "territory_id": "dk",
        "parent_community_id": dk_id,
        "inherit_requirements": False
    })

    if healing_id:
        create_community(token, {
            "name": "Breathwork & Meditation",
            "slug": "breathwork-meditation",
            "description": "Exploring conscious breathing techniques and meditation practices",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": healing_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Plant Medicine Circle",
            "slug": "plant-medicine",
            "description": "Respectful exploration of sacred plant allies and traditional ceremonies",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": healing_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Somatic Healing",
            "slug": "somatic-healing",
            "description": "Body-based approaches to healing trauma and restoring vitality",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": healing_id,
            "inherit_requirements": True
        })

    # Regenerative Land Group
    land_id = create_community(token, {
        "name": "Regenerative Land Practices",
        "slug": "regenerative-land",
        "description": "All practices related to healing and working with land",
        "community_type": "group",
        "territory_id": "dk",
        "parent_community_id": dk_id,
        "inherit_requirements": False
    })

    if land_id:
        create_community(token, {
            "name": "Food Forest Design",
            "slug": "food-forest-design",
            "description": "Designing multi-layered edible ecosystems",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": land_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Soil Regeneration",
            "slug": "soil-regeneration",
            "description": "Building living soil through composting, cover crops, and no-till methods",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": land_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Wildcraft & Foraging",
            "slug": "wildcraft-foraging",
            "description": "Ethical wildcrafting, foraging, and connecting with local ecosystems",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": land_id,
            "inherit_requirements": True
        })

    # Community Building Group
    community_building_id = create_community(token, {
        "name": "Intentional Community Building",
        "slug": "intentional-community",
        "description": "Resources for creating and sustaining intentional communities",
        "community_type": "group",
        "territory_id": "dk",
        "parent_community_id": dk_id,
        "inherit_requirements": False
    })

    if community_building_id:
        create_community(token, {
            "name": "Ecovillage Design",
            "slug": "ecovillage-design",
            "description": "Planning and developing sustainable human settlements",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": community_building_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Conflict Transformation",
            "slug": "conflict-transformation",
            "description": "Tools and practices for healthy conflict resolution in communities",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": community_building_id,
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Gift Economy Experiments",
            "slug": "gift-economy",
            "description": "Exploring alternatives to transactional economics",
            "community_type": "study_group",
            "territory_id": "dk",
            "parent_community_id": community_building_id,
            "inherit_requirements": True
        })

    # 7. Local Chapters of Guilds (attached to cities)
    if "copenhagen" in city_ids:
        create_community(token, {
            "name": "Copenhagen Food Forest Network",
            "slug": "cph-food-forest",
            "description": "Urban food forests and edible landscapes in Copenhagen",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["copenhagen"],
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "København Healing Circle",
            "slug": "cph-healing-circle",
            "description": "Weekly gathering for holistic health practitioners in Copenhagen",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["copenhagen"],
            "inherit_requirements": True
        })

    if "aarhus" in city_ids:
        create_community(token, {
            "name": "Aarhus Seed Library",
            "slug": "aarhus-seed-library",
            "description": "Community seed saving and sharing initiative",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["aarhus"],
            "inherit_requirements": True
        })
        create_community(token, {
            "name": "Aarhus Water Protectors",
            "slug": "aarhus-water-protectors",
            "description": "Protecting and restoring local waterways",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["aarhus"],
            "inherit_requirements": True
        })

    if "odense" in city_ids:
        create_community(token, {
            "name": "Fyn Regenerative Farmers",
            "slug": "fyn-regen-farmers",
            "description": "Network of regenerative farmers on Funen island",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["odense"],
            "inherit_requirements": True
        })

    if "aalborg" in city_ids:
        create_community(token, {
            "name": "Nordjylland Herbal Guild",
            "slug": "nordjylland-herbal",
            "description": "Traditional herbalism and wildcrafting in North Jutland",
            "community_type": "guild",
            "territory_id": "dk",
            "parent_community_id": city_ids["aalborg"],
            "inherit_requirements": True
        })

    log("Seeding complete!")

if __name__ == "__main__":
    token = get_token()
    if token:
        seed_communities(token)
