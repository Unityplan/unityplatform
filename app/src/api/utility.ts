/**
 * Utility Service API Client
 * 
 * Handles communication with the utility-service for favicon fetching
 * and other utility functions.
 */

import { useAuthStore } from '@/stores/authStore';

const UTILITY_BASE_URL = import.meta.env.VITE_UTILITY_SERVICE_URL || 'http://localhost:8014/api/v1/utility';

/**
 * Fetch a website's favicon
 * Returns a blob URL that can be used in img src
 */
export async function fetchFavicon(url: string, size: number = 32): Promise<string> {
    const { accessToken } = useAuthStore.getState();
    if (!accessToken) {
        throw new Error('Authentication required');
    }

    const params = new URLSearchParams({
        url,
        size: size.toString(),
    });

    const response = await fetch(`${UTILITY_BASE_URL}/favicon?${params}`, {
        headers: {
            'Authorization': `Bearer ${accessToken}`,
        },
    });

    if (!response.ok) {
        throw new Error(`Failed to fetch favicon: ${response.status}`);
    }

    const blob = await response.blob();
    return URL.createObjectURL(blob);
}

/**
 * Extract domain from URL for display
 */
export function extractDomain(url: string): string {
    try {
        const urlObj = new URL(url);
        return urlObj.hostname.replace('www.', '');
    } catch {
        return url;
    }
}
