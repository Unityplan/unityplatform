import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * JWT Claims structure (must match backend Claims)
 */
export interface JwtClaims {
  sub: string; // User ID
  territory: string;
  badges: string[]; // Badge slugs
  exp: number;
  iat: number;
}

/**
 * Parse JWT token to extract claims
 * Note: This does NOT validate the signature, just decodes the payload
 */
export function parseJwt(token: string): JwtClaims | null {
  try {
    const parts = token.split('.');
    if (parts.length !== 3) return null;
    
    const payload = parts[1];
    const decoded = atob(payload.replace(/-/g, '+').replace(/_/g, '/'));
    const claims = JSON.parse(decoded) as JwtClaims;
    
    // Ensure badges is always an array
    if (!Array.isArray(claims.badges)) {
      claims.badges = [];
    }
    
    return claims;
  } catch (e) {
    console.error('Failed to parse JWT:', e);
    return null;
  }
}
