import { useState, useCallback } from 'react';
import { Map, MapMarker, MapPopup, MapTileLayer } from '@/components/ui/map';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { MapPin, Loader2, Navigation, AlertCircle } from 'lucide-react';
import { reverseGeocode, type GeocodeResult, encodeLocation, parseLocationCoordinates, getLocationDisplayName } from '@/lib/geocoding';
import type { LatLngExpression, LeafletMouseEvent } from 'leaflet';
import { useMapEvents } from 'react-leaflet';

interface LocationPickerProps {
    value?: string; // Encoded location string: "[lat,lng]City, Country"
    onChange: (encodedLocation: string) => void;
    disabled?: boolean;
}

export function LocationPicker({ value, onChange, disabled = false }: LocationPickerProps) {
    // Parse initial coordinates from value or default to Europe
    const initialCoords = value ? parseLocationCoordinates(value) : null;
    const [position, setPosition] = useState<[number, number]>(initialCoords || [55.6761, 12.5683]); // Default: Copenhagen
    const [displayName, setDisplayName] = useState<string>(value ? getLocationDisplayName(value) : '');
    const [isGeocoding, setIsGeocoding] = useState(false);
    const [showMap, setShowMap] = useState(false);
    const [isLocating, setIsLocating] = useState(false);
    const [locationError, setLocationError] = useState<string>('');

    // Reverse geocode when position changes
    const updateLocation = useCallback(async (lat: number, lng: number) => {
        setIsGeocoding(true);
        try {
            const result: GeocodeResult = await reverseGeocode(lat, lng);
            setDisplayName(result.displayName);
            // Encode location with coordinates and update parent
            const encoded = encodeLocation(lat, lng, result.displayName);
            onChange(encoded);
        } catch (error) {
            console.error('Failed to geocode location:', error);
        } finally {
            setIsGeocoding(false);
        }
    }, [onChange]);

    // Handle map click
    const handleMapClick = useCallback((e: LeafletMouseEvent) => {
        if (disabled) return;
        const { lat, lng } = e.latlng;
        setPosition([lat, lng]);
        updateLocation(lat, lng);
        setLocationError(''); // Clear error when user picks a location
    }, [disabled, updateLocation]);

    // Get user's current location
    const handleLocateMe = useCallback(() => {
        if (disabled || !navigator.geolocation) {
            setLocationError('Geolocation is not supported by your browser');
            setShowMap(true);
            return;
        }

        setIsLocating(true);
        setLocationError('');

        navigator.geolocation.getCurrentPosition(
            (position) => {
                const lat = position.coords.latitude;
                const lng = position.coords.longitude;
                setPosition([lat, lng]);
                updateLocation(lat, lng);
                setIsLocating(false);
                setShowMap(true); // Auto-open map when location is found
            },
            (error) => {
                setIsLocating(false);
                setShowMap(true); // Show map so user can pick manually

                // Provide helpful error messages
                switch (error.code) {
                    case error.PERMISSION_DENIED:
                        setLocationError('Location permission denied. Please allow location access or pick your location manually.');
                        break;
                    case error.POSITION_UNAVAILABLE:
                        setLocationError('Location information unavailable. Please pick your location manually.');
                        break;
                    case error.TIMEOUT:
                        setLocationError('Location request timed out. Please pick your location manually.');
                        break;
                    default:
                        setLocationError('Unable to get your location. Please pick your location manually.');
                }
            },
            {
                enableHighAccuracy: false,
                timeout: 10000,
                maximumAge: 300000 // 5 minutes cache
            }
        );
    }, [disabled, updateLocation]);

    // Map click handler component
    function MapClickHandler({ onClick }: { onClick: (e: LeafletMouseEvent) => void }) {
        useMapEvents({
            click: onClick,
        });
        return null;
    }

    return (
        <div className="space-y-4">
            {/* Location Display */}
            <div className="flex items-center gap-2 p-3 rounded-lg border bg-muted/50">
                <MapPin className="size-5 text-muted-foreground shrink-0" />
                <div className="flex-1 min-w-0">
                    {isGeocoding ? (
                        <div className="flex items-center gap-2 text-sm text-muted-foreground">
                            <Loader2 className="size-4 animate-spin" />
                            Finding location...
                        </div>
                    ) : displayName ? (
                        <p className="text-sm font-medium truncate">{displayName}</p>
                    ) : (
                        <p className="text-sm text-muted-foreground">No location selected</p>
                    )}
                </div>
                <div className="flex gap-2">
                    <Button
                        type="button"
                        size="sm"
                        variant="outline"
                        onClick={handleLocateMe}
                        disabled={disabled || isLocating}
                    >
                        {isLocating ? (
                            <Loader2 className="size-4 animate-spin" />
                        ) : (
                            <Navigation className="size-4" />
                        )}
                        <span className="ml-2">Locate Me</span>
                    </Button>
                    <Button
                        type="button"
                        size="sm"
                        variant="outline"
                        onClick={() => {
                            setShowMap(!showMap);
                            if (showMap) {
                                setLocationError(''); // Clear error when hiding map
                            }
                        }}
                        disabled={disabled}
                    >
                        <MapPin className="size-4" />
                        <span className="ml-2">{showMap ? 'Hide Map' : 'Pick Location'}</span>
                    </Button>
                </div>
            </div>

            {/* Location Error Message */}
            {locationError && (
                <div className="flex items-start gap-2 p-3 rounded-lg border border-yellow-200 dark:border-yellow-800 bg-yellow-50 dark:bg-yellow-900/20">
                    <AlertCircle className="size-5 text-yellow-600 dark:text-yellow-500 shrink-0 mt-0.5" />
                    <p className="text-sm text-yellow-800 dark:text-yellow-200">{locationError}</p>
                </div>
            )}

            {/* Interactive Map */}
            {showMap && (
                <Card>
                    <CardHeader>
                        <CardTitle className="text-base">Select Your Location</CardTitle>
                        <CardDescription>
                            Click anywhere on the map to set your approximate city location
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="h-[400px] rounded-lg overflow-hidden border">
                            <Map
                                center={position as LatLngExpression}
                                zoom={10}
                                className="h-full w-full"
                            >
                                {/* Always use light map for better visibility */}
                                <MapTileLayer
                                    url="https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png"
                                    attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors &copy; <a href="https://carto.com/attributions">CARTO</a>'
                                />
                                <MapClickHandler onClick={handleMapClick} />

                                {/* Marker at selected position - Use theme primary color */}
                                <MapMarker
                                    position={position as LatLngExpression}
                                    icon={<MapPin className="size-6 text-primary fill-primary" />}
                                >
                                    <MapPopup>
                                        <div className="p-2">
                                            <p className="font-semibold text-sm">
                                                {displayName || 'Selected Location'}
                                            </p>
                                            <p className="text-xs text-muted-foreground mt-1">
                                                {position[0].toFixed(4)}°, {position[1].toFixed(4)}°
                                            </p>
                                        </div>
                                    </MapPopup>
                                </MapMarker>
                            </Map>
                        </div>
                        <p className="text-xs text-muted-foreground mt-2">
                            💡 Tip: Click on your city to set your location. This shows your approximate area, not your exact address.
                        </p>
                    </CardContent>
                </Card>
            )}
        </div>
    );
}
