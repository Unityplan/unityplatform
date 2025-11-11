/**
 * Geocoding utilities for converting coordinates to human-readable locations
 * Uses OpenStreetMap's Nominatim API for reverse geocoding
 */

export interface GeocodeResult {
    city: string;
    country: string;
    displayName: string; // "City, Country" format
    fullAddress?: string; // Complete address from API
}

/**
 * Reverse geocode coordinates to get city and country
 * Uses Nominatim API (free, no API key required)
 * 
 * @param lat - Latitude
 * @param lng - Longitude
 * @returns GeocodeResult with city, country, and display name
 */
export async function reverseGeocode(lat: number, lng: number): Promise<GeocodeResult> {
    try {
        const response = await fetch(
            `https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lng}&zoom=18&addressdetails=1`,
            {
                headers: {
                    'Accept-Language': 'en', // Get results in English
                },
            }
        );

        if (!response.ok) {
            throw new Error('Geocoding request failed');
        }

        const data = await response.json();

        // Try to get the most specific location available
        // Priority: village/hamlet/suburb -> town -> city -> municipality -> county
        const locality =
            data.address?.village ||
            data.address?.hamlet ||
            data.address?.suburb ||
            data.address?.town ||
            data.address?.city ||
            data.address?.municipality ||
            data.address?.county ||
            'Unknown Location';

        // Get broader region (municipality or county) for additional context
        const region =
            data.address?.municipality ||
            data.address?.county ||
            data.address?.state_district ||
            null;

        // Get country
        const country = data.address?.country || 'Unknown Country';

        // Create display name with region if available and different from locality
        let displayName = locality;
        if (region && region !== locality) {
            displayName = `${locality}, ${region}, ${country}`;
        } else {
            displayName = `${locality}, ${country}`;
        }

        return {
            city: locality,
            country,
            displayName,
            fullAddress: data.display_name,
        };
    } catch (error) {
        console.error('Reverse geocoding error:', error);
        // Return fallback values if geocoding fails
        return {
            city: 'Unknown Location',
            country: 'Unknown Country',
            displayName: 'Unknown Location, Unknown Country',
        };
    }
}

/**
 * Format coordinates as a readable string
 * @param lat - Latitude
 * @param lng - Longitude
 * @returns Formatted coordinate string
 */
export function formatCoordinates(lat: number, lng: number): string {
    const latDir = lat >= 0 ? 'N' : 'S';
    const lngDir = lng >= 0 ? 'E' : 'W';
    return `${Math.abs(lat).toFixed(4)}°${latDir}, ${Math.abs(lng).toFixed(4)}°${lngDir}`;
}

/**
 * Parse location string to extract coordinates if stored
 * Expected format: "City, Country" or "[lat,lng]City, Country"
 * 
 * @param location - Location string from database
 * @returns Coordinates if found, null otherwise
 */
export function parseLocationCoordinates(location: string | null | undefined): [number, number] | null {
    if (!location) return null;

    // Try to parse coordinates from format: "[lat,lng]City, Country"
    const coordMatch = location.match(/^\[(-?\d+\.?\d*),(-?\d+\.?\d*)\]/);
    if (coordMatch) {
        const lat = parseFloat(coordMatch[1]);
        const lng = parseFloat(coordMatch[2]);
        if (!isNaN(lat) && !isNaN(lng)) {
            return [lat, lng];
        }
    }

    return null;
}

/**
 * Extract display name from location string
 * If format is "[lat,lng]City, Country", returns "City, Country"
 * Otherwise returns the original string
 * 
 * @param location - Location string from database
 * @returns Display name without coordinates
 */
export function getLocationDisplayName(location: string | null | undefined): string {
    if (!location) return '';

    // Remove coordinates prefix if present
    const displayMatch = location.match(/^\[-?\d+\.?\d*,-?\d+\.?\d*\](.+)$/);
    if (displayMatch) {
        return displayMatch[1];
    }

    return location;
}

/**
 * Encode location with coordinates for storage
 * Format: "[lat,lng]City, Country"
 * 
 * @param lat - Latitude
 * @param lng - Longitude
 * @param displayName - Human-readable location name
 * @returns Encoded location string
 */
export function encodeLocation(lat: number, lng: number, displayName: string): string {
    return `[${lat},${lng}]${displayName}`;
}
