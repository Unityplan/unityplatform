#!/usr/bin/env python3
"""
Denmark Geographic Seed Script

This script seeds Denmark's geographic hierarchy:
- 1 Country (Denmark)
- 5 Regions (Hovedstaden, Midtjylland, Nordjylland, Sjælland, Syddanmark)
- 98 Municipalities (kommuner)
- 5 Neighborhoods per municipality (largest cities) with 5km radius

Data sources:
- Country: temp/dk.json
- Regions: https://simplemaps.com/static/svg/country/dk/admin1/dk.json
- Municipalities: https://github.com/magnuslarsen/geoJSON-Danish-municipalities

Usage:
    python scripts/dev/seed-denmark-geo.py [--dry-run] [--skip-municipalities] [--skip-neighborhoods]
"""

import os
import sys
import json
import uuid
import argparse
import psycopg2
from psycopg2.extras import RealDictCursor
from datetime import datetime
from typing import Optional

# Database connection settings
DB_HOST = os.getenv("DB_HOST", "localhost")
DB_PORT = os.getenv("DB_PORT", "5432")
DB_NAME = os.getenv("DB_NAME", "unityplatform_dk")
DB_USER = os.getenv("DB_USER", "unityplatform")
DB_PASSWORD = os.getenv("DB_PASSWORD", "unityplatform_dev_password_dk")

# Territory ID (Denmark)
TERRITORY_ID = os.getenv("TERRITORY_ID", "550e8400-e29b-41d4-a716-446655440001")

# Region ISO to name mapping
REGION_MAP = {
    "DK-81": {"name": "Nordjylland", "name_en": "North Denmark"},
    "DK-82": {"name": "Midtjylland", "name_en": "Central Denmark"},
    "DK-83": {"name": "Syddanmark", "name_en": "Southern Denmark"},
    "DK-84": {"name": "Hovedstaden", "name_en": "Capital Region"},
    "DK-85": {"name": "Sjælland", "name_en": "Zealand"},
}

# Municipality to region mapping (from data.csv)
MUNICIPALITY_REGION_MAP = {
    "Aabenraa": "DK-83",
    "Aalborg": "DK-81",
    "Århus": "DK-82",
    "Ærø": "DK-83",
    "Albertslund": "DK-84",
    "Allerød": "DK-84",
    "Assens": "DK-83",
    "Ballerup": "DK-84",
    "Billund": "DK-83",
    "Bornholm": "DK-84",
    "Brøndby": "DK-84",
    "Brønderslev-Dronninglund": "DK-81",
    "Christiansø": "DK-84",
    "Dragør": "DK-84",
    "Egedal": "DK-84",
    "Esbjerg": "DK-83",
    "Faaborg-Midtfyn": "DK-83",
    "Fanø": "DK-83",
    "Favrskov": "DK-82",
    "Faxe": "DK-85",
    "Fredensborg": "DK-84",
    "Fredericia": "DK-83",
    "Frederiksberg": "DK-84",
    "Frederikshavn": "DK-81",
    "Frederikssund": "DK-84",
    "Furesø": "DK-84",
    "Gentofte": "DK-84",
    "Gladsaxe": "DK-84",
    "Glostrup": "DK-84",
    "Greve": "DK-85",
    "Gribskov": "DK-84",
    "Guldborgsund": "DK-85",
    "Haderslev": "DK-83",
    "Halsnæs": "DK-84",
    "Hedensted": "DK-82",
    "Helsingør": "DK-84",
    "Herlev": "DK-84",
    "Herning": "DK-82",
    "Hillerød": "DK-84",
    "Hjørring": "DK-81",
    "Høje-Taastrup": "DK-84",
    "Holbæk": "DK-85",
    "Holstebro": "DK-82",
    "Horsens": "DK-82",
    "Hørsholm": "DK-84",
    "Hvidovre": "DK-84",
    "Ikast-Brande": "DK-82",
    "Ishøj": "DK-84",
    "Jammerbugt": "DK-81",
    "Kalundborg": "DK-85",
    "Kerteminde": "DK-83",
    "København": "DK-84",
    "Køge": "DK-85",
    "Kolding": "DK-83",
    "Læsø": "DK-81",
    "Langeland": "DK-83",
    "Lejre": "DK-85",
    "Lemvig": "DK-82",
    "Lolland": "DK-85",
    "Lyngby-Taarbæk": "DK-84",
    "Mariagerfjord": "DK-81",
    "Middelfart": "DK-83",
    "Morsø": "DK-81",
    "Næstved": "DK-85",
    "Norddjurs": "DK-82",
    "Nordfyns": "DK-83",
    "Nyborg": "DK-83",
    "Odder": "DK-82",
    "Odense": "DK-83",
    "Odsherred": "DK-85",
    "Randers": "DK-82",
    "Rebild": "DK-81",
    "Ringkøbing-Skjern": "DK-82",
    "Ringsted": "DK-85",
    "Rødovre": "DK-84",
    "Roskilde": "DK-85",
    "Rudersdal": "DK-84",
    "Samsø": "DK-82",
    "Silkeborg": "DK-82",
    "Skanderborg": "DK-82",
    "Skive": "DK-82",
    "Slagelse": "DK-85",
    "Solrød": "DK-85",
    "Sønderborg": "DK-83",
    "Sorø": "DK-85",
    "Stevns": "DK-85",
    "Struer": "DK-82",
    "Svendborg": "DK-83",
    "Syddjurs": "DK-82",
    "Tårnby": "DK-84",
    "Thisted": "DK-81",
    "Tønder": "DK-83",
    "Vallensbæk": "DK-84",
    "Varde": "DK-83",
    "Vejen": "DK-83",
    "Vejle": "DK-83",
    "Vesthimmerland": "DK-81",
    "Viborg": "DK-82",
    "Vordingborg": "DK-85",
}

# Major cities per municipality (top 5 neighborhoods per municipality)
# Format: municipality -> [(city_name, lat, lng), ...]
NEIGHBORHOOD_DATA = {
    # Hovedstaden (Capital Region)
    "København": [
        ("Indre By", 55.6794, 12.5700),
        ("Nørrebro", 55.6970, 12.5470),
        ("Vesterbro", 55.6686, 12.5478),
        ("Østerbro", 55.7083, 12.5767),
        ("Amager", 55.6500, 12.6000),
    ],
    "Frederiksberg": [
        ("Frederiksberg C", 55.6800, 12.5300),
        ("Frederiksberg Allé", 55.6720, 12.5350),
        ("Flintholm", 55.6830, 12.5100),
        ("Solbjerg", 55.6750, 12.5150),
        ("Falkoner Plads", 55.6830, 12.5280),
    ],
    "Gentofte": [
        ("Gentofte", 55.7500, 12.5500),
        ("Hellerup", 55.7300, 12.5700),
        ("Charlottenlund", 55.7600, 12.5800),
        ("Ordrup", 55.7550, 12.5650),
        ("Dyssegård", 55.7400, 12.5400),
    ],
    "Gladsaxe": [
        ("Gladsaxe", 55.7330, 12.4900),
        ("Bagsværd", 55.7600, 12.4500),
        ("Søborg", 55.7250, 12.5050),
        ("Mørkhøj", 55.7200, 12.4700),
        ("Buddinge", 55.7400, 12.5000),
    ],
    "Lyngby-Taarbæk": [
        ("Lyngby", 55.7700, 12.5000),
        ("Taarbæk", 55.7800, 12.6200),
        ("Virum", 55.7900, 12.4700),
        ("Sorgenfri", 55.7750, 12.5200),
        ("Lundtofte", 55.7850, 12.5100),
    ],
    
    # Midtjylland (Central Denmark)
    "Århus": [
        ("Aarhus C", 56.1567, 10.2108),
        ("Viby", 56.1300, 10.1900),
        ("Brabrand", 56.1600, 10.1200),
        ("Højbjerg", 56.1200, 10.2200),
        ("Risskov", 56.1900, 10.2300),
    ],
    "Herning": [
        ("Herning C", 56.1394, 8.9733),
        ("Gjellerup", 56.1500, 9.0100),
        ("Lind", 56.1200, 8.9500),
        ("Tjørring", 56.1600, 8.9300),
        ("Hammerum", 56.1300, 9.0400),
    ],
    "Silkeborg": [
        ("Silkeborg C", 56.1700, 9.5450),
        ("Alderslyst", 56.1850, 9.5600),
        ("Hvinningdal", 56.1600, 9.5200),
        ("Virklund", 56.1500, 9.5800),
        ("Them", 56.1000, 9.5000),
    ],
    "Viborg": [
        ("Viborg C", 56.4531, 9.4022),
        ("Overlund", 56.4600, 9.4200),
        ("Hald Ege", 56.4300, 9.3800),
        ("Bruunshåb", 56.4700, 9.4100),
        ("Løgstrup", 56.5200, 9.4400),
    ],
    "Randers": [
        ("Randers C", 56.4614, 10.0369),
        ("Vorup", 56.4500, 10.0200),
        ("Kristrup", 56.4400, 10.0500),
        ("Dronningborg", 56.4700, 10.0600),
        ("Paderup", 56.4800, 10.0300),
    ],
    
    # Syddanmark (Southern Denmark)
    "Odense": [
        ("Odense C", 55.4038, 10.4024),
        ("Bolbro", 55.3900, 10.3500),
        ("Dalum", 55.3700, 10.3800),
        ("Tarup", 55.4200, 10.4200),
        ("Seden", 55.4300, 10.4500),
    ],
    "Esbjerg": [
        ("Esbjerg C", 55.4700, 8.4522),
        ("Jerne", 55.4900, 8.4800),
        ("Sædding", 55.4600, 8.4100),
        ("Gjesing", 55.4500, 8.4800),
        ("Strandby", 55.4800, 8.4200),
    ],
    "Vejle": [
        ("Vejle C", 55.7092, 9.5358),
        ("Nørremarken", 55.7200, 9.5100),
        ("Hover", 55.7300, 9.5500),
        ("Mølholm", 55.7100, 9.5000),
        ("Bredballe", 55.7000, 9.5700),
    ],
    "Kolding": [
        ("Kolding C", 55.4904, 9.4722),
        ("Bramdrup", 55.4700, 9.5000),
        ("Seest", 55.5000, 9.5200),
        ("Vonsild", 55.5100, 9.4500),
        ("Dalby", 55.4600, 9.4300),
    ],
    "Sønderborg": [
        ("Sønderborg C", 54.9094, 9.7919),
        ("Ulkebøl", 54.9200, 9.8100),
        ("Augustenborg", 54.9500, 9.8700),
        ("Nordborg", 55.0500, 9.7500),
        ("Høruphav", 54.8900, 9.8500),
    ],
    
    # Nordjylland (North Denmark)
    "Aalborg": [
        ("Aalborg C", 57.0480, 9.9187),
        ("Nørresundby", 57.0700, 9.9200),
        ("Vejgaard", 57.0300, 9.9500),
        ("Hasseris", 57.0400, 9.8800),
        ("Svenstrup", 57.0100, 9.8500),
    ],
    "Hjørring": [
        ("Hjørring C", 57.4633, 9.9822),
        ("Hirtshals", 57.5900, 9.9600),
        ("Sindal", 57.4700, 10.2000),
        ("Vrå", 57.3600, 9.9700),
        ("Løkken", 57.3700, 9.7100),
    ],
    "Frederikshavn": [
        ("Frederikshavn C", 57.4406, 10.5364),
        ("Skagen", 57.7250, 10.5850),
        ("Sæby", 57.3300, 10.5300),
        ("Strandby", 57.4900, 10.5000),
        ("Bangsbostrand", 57.4200, 10.5200),
    ],
    "Thisted": [
        ("Thisted C", 56.9556, 8.6936),
        ("Hanstholm", 57.1200, 8.6200),
        ("Hurup", 56.7500, 8.4200),
        ("Nykøbing Mors", 56.7950, 8.8600),
        ("Snedsted", 56.8900, 8.5300),
    ],
    "Mariagerfjord": [
        ("Hobro", 56.6433, 9.7906),
        ("Mariager", 56.6500, 9.9700),
        ("Arden", 56.7700, 9.8600),
        ("Hadsund", 56.7100, 10.1100),
        ("Als", 56.7300, 10.0000),
    ],
    
    # Sjælland (Zealand)
    "Roskilde": [
        ("Roskilde C", 55.6419, 12.0803),
        ("Trekroner", 55.6600, 12.1300),
        ("Svogerslev", 55.6200, 12.0200),
        ("Jyllinge", 55.7400, 12.1000),
        ("Viby Sjælland", 55.5500, 12.0300),
    ],
    "Næstved": [
        ("Næstved C", 55.2297, 11.7608),
        ("Fensmark", 55.2700, 11.8000),
        ("Fuglebjerg", 55.3300, 11.5900),
        ("Karrebæksminde", 55.1700, 11.6500),
        ("Glumsø", 55.3500, 11.7000),
    ],
    "Slagelse": [
        ("Slagelse C", 55.4028, 11.3544),
        ("Korsør", 55.3300, 11.1400),
        ("Skælskør", 55.2500, 11.2900),
        ("Slots Bjergby", 55.3800, 11.3200),
        ("Vemmelev", 55.3700, 11.2400),
    ],
    "Holbæk": [
        ("Holbæk C", 55.7169, 11.7133),
        ("Tølløse", 55.6200, 11.7600),
        ("Svinninge", 55.7200, 11.5400),
        ("Jyderup", 55.6600, 11.4200),
        ("Regstrup", 55.6800, 11.5900),
    ],
    "Guldborgsund": [
        ("Nykøbing Falster", 54.7658, 11.8744),
        ("Sakskøbing", 54.7900, 11.6300),
        ("Nysted", 54.6700, 11.7300),
        ("Stubbekøbing", 54.8900, 12.0400),
        ("Marielyst", 54.6300, 11.9500),
    ],
}

# Default neighborhood radius in meters (5km)
NEIGHBORHOOD_RADIUS_M = 5000


def get_db_connection():
    """Create a database connection."""
    return psycopg2.connect(
        host=DB_HOST,
        port=DB_PORT,
        dbname=DB_NAME,
        user=DB_USER,
        password=DB_PASSWORD
    )


def generate_uuid() -> str:
    """Generate a new UUID."""
    return str(uuid.uuid4())


def calculate_centroid(coordinates):
    """Calculate the centroid of a polygon."""
    if not coordinates or not coordinates[0]:
        return None, None
    
    # Handle MultiPolygon vs Polygon
    if isinstance(coordinates[0][0][0], list):
        # MultiPolygon - use first polygon
        coords = coordinates[0][0]
    else:
        # Polygon
        coords = coordinates[0]
    
    total_lat = sum(c[1] for c in coords)
    total_lng = sum(c[0] for c in coords)
    count = len(coords)
    
    return total_lat / count, total_lng / count


def convert_geojson_to_coverage(geometry):
    """Convert GeoJSON geometry to our coverage_area format."""
    if geometry["type"] == "Polygon":
        coords = geometry["coordinates"][0]
        return {
            "type": "polygon",
            "coordinates": [{"lat": c[1], "lng": c[0]} for c in coords]
        }
    elif geometry["type"] == "MultiPolygon":
        # Use the largest polygon
        largest = max(geometry["coordinates"], key=lambda p: len(p[0]))
        coords = largest[0]
        return {
            "type": "polygon",
            "coordinates": [{"lat": c[1], "lng": c[0]} for c in coords]
        }
    return None


def create_circular_coverage(lat: float, lng: float, radius_m: float):
    """Create a circular coverage area (approximated as polygon)."""
    import math
    
    # Approximate degree per meter at this latitude
    lat_per_m = 1 / 111320
    lng_per_m = 1 / (111320 * math.cos(math.radians(lat)))
    
    # Create circle with 32 points
    points = []
    for i in range(32):
        angle = 2 * math.pi * i / 32
        d_lat = radius_m * lat_per_m * math.sin(angle)
        d_lng = radius_m * lng_per_m * math.cos(angle)
        points.append({"lat": lat + d_lat, "lng": lng + d_lng})
    
    return {
        "type": "polygon",
        "coordinates": points
    }


def get_coc_badge_id(conn, cursor) -> Optional[str]:
    """Get the Code of Conduct badge ID."""
    try:
        # Check if the table exists first
        cursor.execute("""
            SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_schema = 'territory_dk' 
                AND table_name = 'badge_definitions'
            )
        """)
        table_exists = cursor.fetchone()[0]
        
        if not table_exists:
            return None
            
        cursor.execute("""
            SELECT id FROM territory_dk.badge_definitions 
            WHERE slug = 'code-of-conduct'
            LIMIT 1
        """)
        row = cursor.fetchone()
        return row["id"] if row else None
    except Exception:
        # If anything fails, rollback to clean state and return None
        conn.rollback()
        return None


def generate_slug(name: str) -> str:
    """Generate a URL-friendly slug from a name."""
    import re
    # Convert to lowercase
    slug = name.lower()
    # Replace Danish characters
    slug = slug.replace('æ', 'ae').replace('ø', 'oe').replace('å', 'aa')
    slug = slug.replace('ä', 'ae').replace('ö', 'oe').replace('ü', 'ue')
    # Replace spaces and special chars with hyphens
    slug = re.sub(r'[^a-z0-9]+', '-', slug)
    # Remove leading/trailing hyphens
    slug = slug.strip('-')
    return slug


def create_community(cursor, data: dict, dry_run: bool = False) -> str:
    """Create a community in the database."""
    community_id = generate_uuid()
    now = datetime.utcnow()
    slug = data.get("slug") or generate_slug(data["name"])
    
    if dry_run:
        print(f"  [DRY-RUN] Would create: {data['name']} ({data['type']}) -> {community_id}")
        return community_id
    
    cursor.execute("""
        INSERT INTO territory_dk.community_communities (
            id, slug, territory_id, name, description, type, 
            parent_community_id, location_lat, location_lng, coverage_area,
            created_at, updated_at
        ) VALUES (
            %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s
        )
        RETURNING id
    """, (
        community_id,
        slug,
        TERRITORY_ID,
        data["name"],
        data.get("description", ""),
        data["type"],
        data.get("parent_community_id"),
        data.get("location_lat"),
        data.get("location_lng"),
        json.dumps(data.get("coverage_area")) if data.get("coverage_area") else None,
        now,
        now
    ))
    
    return community_id


def add_coc_requirement(cursor, community_id: str, badge_id: str, dry_run: bool = False):
    """Add Code of Conduct badge requirement to a community."""
    if dry_run:
        print(f"  [DRY-RUN] Would add CoC requirement to {community_id}")
        return
    
    now = datetime.utcnow()
    cursor.execute("""
        INSERT INTO territory_dk.community_badge_requirements (
            id, community_id, badge_id, context, created_at
        ) VALUES (%s, %s, %s, %s, %s)
        ON CONFLICT (community_id, badge_id, context) DO NOTHING
    """, (
        generate_uuid(),
        community_id,
        badge_id,
        "participate",
        now
    ))


def seed_denmark(cursor, dry_run: bool = False) -> str:
    """Create Denmark as a top-level Zone."""
    print("\n=== Creating Denmark (Country) ===")
    
    # Load GeoJSON from temp/dk.json
    script_dir = os.path.dirname(os.path.abspath(__file__))
    dk_file = os.path.join(script_dir, "../../temp/dk.json")
    
    lat, lng = 55.6761, 12.5683  # Copenhagen as default center
    coverage = None
    
    if os.path.exists(dk_file):
        with open(dk_file, "r") as f:
            dk_data = json.load(f)
            if dk_data.get("features"):
                feature = dk_data["features"][0]
                geometry = feature.get("geometry", {})
                lat, lng = calculate_centroid(geometry.get("coordinates", []))
                coverage = convert_geojson_to_coverage(geometry)
    
    community_id = create_community(cursor, {
        "name": "Danmark",
        "description": "Kongeriget Danmark - The Kingdom of Denmark",
        "type": "zone",
        "location_lat": lat,
        "location_lng": lng,
        "coverage_area": coverage
    }, dry_run)
    
    print(f"  Created: Danmark -> {community_id}")
    return community_id


def seed_regions(cursor, parent_id: str, coc_badge_id: Optional[str], dry_run: bool = False) -> dict:
    """Create the 5 Danish regions."""
    print("\n=== Creating Regions ===")
    
    # Load regions from SimpleMaps data (in-memory, parsed from earlier fetch)
    # For now, create with approximate centers
    
    region_centers = {
        "DK-81": (57.0480, 9.9187),   # Nordjylland (Aalborg)
        "DK-82": (56.1567, 10.2108),  # Midtjylland (Aarhus)
        "DK-83": (55.4038, 10.4024),  # Syddanmark (Odense)
        "DK-84": (55.6761, 12.5683),  # Hovedstaden (Copenhagen)
        "DK-85": (55.6419, 12.0803),  # Sjælland (Roskilde)
    }
    
    region_ids = {}
    
    for iso_code, info in REGION_MAP.items():
        lat, lng = region_centers.get(iso_code, (55.6761, 12.5683))
        
        community_id = create_community(cursor, {
            "name": f"Region {info['name']}",
            "description": f"{info['name_en']} Region of Denmark",
            "type": "zone",
            "parent_community_id": parent_id,
            "location_lat": lat,
            "location_lng": lng,
        }, dry_run)
        
        # Add CoC requirement
        if coc_badge_id and not dry_run:
            add_coc_requirement(cursor, community_id, coc_badge_id, dry_run)
        
        region_ids[iso_code] = community_id
        print(f"  Created: Region {info['name']} -> {community_id}")
    
    return region_ids


def seed_municipalities(cursor, region_ids: dict, coc_badge_id: Optional[str], dry_run: bool = False) -> dict:
    """Create municipalities under their respective regions."""
    print("\n=== Creating Municipalities ===")
    
    municipality_ids = {}
    
    for muni_name, iso_code in MUNICIPALITY_REGION_MAP.items():
        region_id = region_ids.get(iso_code)
        if not region_id:
            print(f"  Warning: No region found for {muni_name} ({iso_code})")
            continue
        
        # Get center from neighborhood data if available, else use region center
        neighborhoods = NEIGHBORHOOD_DATA.get(muni_name, [])
        if neighborhoods:
            lat, lng = neighborhoods[0][1], neighborhoods[0][2]
        else:
            # Approximate center based on region
            region_centers = {
                "DK-81": (57.0480, 9.9187),
                "DK-82": (56.1567, 10.2108),
                "DK-83": (55.4038, 10.4024),
                "DK-84": (55.6761, 12.5683),
                "DK-85": (55.6419, 12.0803),
            }
            lat, lng = region_centers.get(iso_code, (55.6761, 12.5683))
        
        community_id = create_community(cursor, {
            "name": f"{muni_name} Kommune",
            "description": f"{muni_name} Municipality",
            "type": "zone",
            "parent_community_id": region_id,
            "location_lat": lat,
            "location_lng": lng,
        }, dry_run)
        
        # Add CoC requirement
        if coc_badge_id and not dry_run:
            add_coc_requirement(cursor, community_id, coc_badge_id, dry_run)
        
        municipality_ids[muni_name] = community_id
        print(f"  Created: {muni_name} Kommune -> {community_id}")
    
    return municipality_ids


def seed_neighborhoods(cursor, municipality_ids: dict, coc_badge_id: Optional[str], dry_run: bool = False):
    """Create neighborhoods (5 per municipality where data available)."""
    print("\n=== Creating Neighborhoods ===")
    
    count = 0
    for muni_name, muni_id in municipality_ids.items():
        neighborhoods = NEIGHBORHOOD_DATA.get(muni_name, [])
        
        if not neighborhoods:
            # Create 5 placeholder neighborhoods with municipality center
            # This is a fallback for municipalities without predefined neighborhoods
            continue
        
        for nb_name, lat, lng in neighborhoods:
            coverage = create_circular_coverage(lat, lng, NEIGHBORHOOD_RADIUS_M)
            
            # Generate unique slug including municipality to avoid duplicates
            # e.g., "strandby-esbjerg" vs "strandby-frederikshavn"
            unique_slug = generate_slug(f"{nb_name}-{muni_name}")
            
            community_id = create_community(cursor, {
                "name": nb_name,
                "slug": unique_slug,
                "description": f"Neighborhood in {muni_name}",
                "type": "neighborhood",
                "parent_community_id": muni_id,
                "location_lat": lat,
                "location_lng": lng,
                "coverage_area": coverage,
            }, dry_run)
            
            # Add CoC requirement
            if coc_badge_id and not dry_run:
                add_coc_requirement(cursor, community_id, coc_badge_id, dry_run)
            
            count += 1
            print(f"  Created: {nb_name} ({muni_name}) -> {community_id}")
    
    print(f"\n  Total neighborhoods created: {count}")


def main():
    parser = argparse.ArgumentParser(description="Seed Denmark geographic communities")
    parser.add_argument("--dry-run", action="store_true", help="Don't actually insert, just show what would be done")
    parser.add_argument("--skip-municipalities", action="store_true", help="Skip creating municipalities")
    parser.add_argument("--skip-neighborhoods", action="store_true", help="Skip creating neighborhoods")
    args = parser.parse_args()
    
    print("=" * 60)
    print("Denmark Geographic Seed Script")
    print("=" * 60)
    
    if args.dry_run:
        print("*** DRY RUN MODE - No changes will be made ***")
    
    conn = get_db_connection()
    conn.autocommit = False
    
    try:
        with conn.cursor(cursor_factory=RealDictCursor) as cursor:
            # Get Code of Conduct badge ID
            coc_badge_id = get_coc_badge_id(conn, cursor)
            if coc_badge_id:
                print(f"\nFound Code of Conduct badge: {coc_badge_id}")
            else:
                print("\nWarning: Code of Conduct badge not found, requirements won't be added")
            
            # Create Denmark
            denmark_id = seed_denmark(cursor, args.dry_run)
            
            # Add CoC requirement to Denmark
            if coc_badge_id and not args.dry_run:
                add_coc_requirement(cursor, denmark_id, coc_badge_id, args.dry_run)
            
            # Create regions
            region_ids = seed_regions(cursor, denmark_id, coc_badge_id, args.dry_run)
            
            # Create municipalities
            if not args.skip_municipalities:
                municipality_ids = seed_municipalities(cursor, region_ids, coc_badge_id, args.dry_run)
            else:
                print("\n=== Skipping Municipalities ===")
                municipality_ids = {}
            
            # Create neighborhoods
            if not args.skip_neighborhoods and municipality_ids:
                seed_neighborhoods(cursor, municipality_ids, coc_badge_id, args.dry_run)
            else:
                print("\n=== Skipping Neighborhoods ===")
            
            if not args.dry_run:
                conn.commit()
                print("\n✓ All changes committed successfully!")
            else:
                conn.rollback()
                print("\n✓ Dry run completed (no changes made)")
    
    except Exception as e:
        conn.rollback()
        print(f"\n✗ Error: {e}")
        raise
    finally:
        conn.close()


if __name__ == "__main__":
    main()
