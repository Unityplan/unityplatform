#!/usr/bin/env python3
"""
Community Seed Script - Guilds, Study Groups, and Groups

This script seeds non-geographic communities:
- Guilds (practice-based communities)
- Study Groups (learning-focused communities)  
- Groups (container communities)
- Local chapters attached to geographic zones

Prerequisites:
- Run seed-denmark-geo.py first to create geographic hierarchy
- Auth service and Community service must be running

Usage:
    python3 scripts/dev/seed-communities.py
"""

import requests
import json
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
    """Get auth token - try login first, then register if needed."""
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

def find_community_by_slug(token, slug):
    """Find a community by its slug."""
    headers = {"Authorization": f"Bearer {token}"}
    try:
        response = requests.get(COMMUNITY_SERVICE_URL, params={"search": slug}, headers=headers)
        if response.status_code == 200:
            for c in response.json():
                if c.get("slug") == slug:
                    return c.get("id")
    except Exception as e:
        log(f"Error finding community by slug '{slug}': {e}")
    return None

def create_community(token, data):
    """Create a community, returning its ID."""
    headers = {"Authorization": f"Bearer {token}"}
    
    try:
        response = requests.post(COMMUNITY_SERVICE_URL, json=data, headers=headers)
        
        if response.status_code == 201:
            log(f"Created: {data['name']}")
            return response.json()["id"]
        elif response.status_code == 409:
            log(f"Already exists: {data['name']}")
            # Try to find it to return ID
            return find_community_by_slug(token, data["slug"])
        else:
            log(f"Failed to create {data['name']}: {response.text}")
            return None
    except Exception as e:
        log(f"Error creating community: {e}")
        return None

def seed_communities(token):
    """Seed guilds, study groups, groups, and local chapters."""
    
    # =========================================================================
    # Step 1: Find root Denmark community (created by geo script)
    # =========================================================================
    log("Looking for Danmark root community...")
    dk_id = find_community_by_slug(token, "danmark")
    
    if not dk_id:
        # Fallback - try "denmark" slug
        dk_id = find_community_by_slug(token, "denmark")
    
    if not dk_id:
        log("ERROR: Root community (Danmark/Denmark) not found!")
        log("Please run seed-denmark-geo.py first to create geographic hierarchy.")
        return
    
    log(f"Found Danmark: {dk_id}")

    # =========================================================================
    # Step 2: Guilds (Interest-based practice communities)
    # =========================================================================
    log("")
    log("=== Creating Guilds ===")
    
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

    # =========================================================================
    # Step 3: Study Groups (Learning-focused communities)
    # =========================================================================
    log("")
    log("=== Creating Study Groups ===")
    
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

    # =========================================================================
    # Step 4: Groups (Container communities for related topics)
    # =========================================================================
    log("")
    log("=== Creating Groups ===")
    
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

    # =========================================================================
    # Step 5: Local Chapters (attached to geographic communities)
    # =========================================================================
    log("")
    log("=== Creating Local Chapters ===")
    
    # Look up municipality communities created by geo script
    # Using "Kommune" suffix as created by seed-denmark-geo.py
    city_slugs = {
        "copenhagen": "koebenhavn-kommune",  # København Kommune
        "aarhus": "aarhus-kommune",           # Århus Kommune  
        "odense": "odense-kommune",           # Odense Kommune
        "aalborg": "aalborg-kommune"          # Aalborg Kommune
    }
    
    city_ids = {}
    for city_key, slug in city_slugs.items():
        cid = find_community_by_slug(token, slug)
        if cid:
            city_ids[city_key] = cid
            log(f"Found {city_key}: {cid}")
        else:
            log(f"Note: {city_key} ({slug}) not found - skipping local chapters")

    # Copenhagen chapters
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

    # Aarhus chapters
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

    # Odense chapters
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

    # Aalborg chapters
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

    log("")
    log("=== Seeding complete! ===")

if __name__ == "__main__":
    token = get_token()
    if token:
        seed_communities(token)
