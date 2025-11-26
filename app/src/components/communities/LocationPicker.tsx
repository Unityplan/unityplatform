import { MapContainer, TileLayer, Marker, Circle, useMapEvents, Polygon } from 'react-leaflet'
import { useState, useEffect } from 'react'
import L from 'leaflet'
import 'leaflet/dist/leaflet.css'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group'
import { Slider } from '@/components/ui/slider'
import type { CoverageArea } from '@/api/community'

// Fix Leaflet marker icon
import icon from 'leaflet/dist/images/marker-icon.png'
import iconShadow from 'leaflet/dist/images/marker-shadow.png'

const DefaultIcon = L.icon({
    iconUrl: icon,
    shadowUrl: iconShadow,
    iconSize: [25, 41],
    iconAnchor: [12, 41],
})

L.Marker.prototype.options.icon = DefaultIcon

interface LocationPickerProps {
    value?: {
        lat?: number
        lng?: number
        coverage?: CoverageArea
    }
    onChange: (value: { lat: number; lng: number; coverage: CoverageArea }) => void
}

function MapEvents({ onMapClick }: { onMapClick: (latlng: L.LatLng) => void }) {
    useMapEvents({
        click(e) {
            onMapClick(e.latlng)
        },
    })
    return null
}

export function LocationPicker({ value, onChange }: LocationPickerProps) {
    const [center, setCenter] = useState<L.LatLng | null>(
        value?.lat && value?.lng ? new L.LatLng(value.lat, value.lng) : null
    )
    const [coverageType, setCoverageType] = useState<'circle' | 'polygon'>(
        value?.coverage?.type || 'circle'
    )
    const [radius, setRadius] = useState<number>(
        value?.coverage?.type === 'circle' ? value.coverage.radius || 1000 : 1000
    )
    const [polygonPoints, setPolygonPoints] = useState<L.LatLng[]>(
        value?.coverage?.type === 'polygon' && value.coverage.coordinates
            ? value.coverage.coordinates.map(p => new L.LatLng(p.lat, p.lng))
            : []
    )

    // Default map center (Copenhagen)
    const defaultCenter = new L.LatLng(55.6761, 12.5683)

    useEffect(() => {
        if (value?.lat && value?.lng) {
            setCenter(new L.LatLng(value.lat, value.lng))
        }
        if (value?.coverage) {
            setCoverageType(value.coverage.type)
            if (value.coverage.type === 'circle') {
                setRadius(value.coverage.radius || 1000)
            } else if (value.coverage.type === 'polygon' && value.coverage.coordinates) {
                setPolygonPoints(value.coverage.coordinates.map(p => new L.LatLng(p.lat, p.lng)))
            }
        }
    }, [value])

    const handleMapClick = (latlng: L.LatLng) => {
        if (coverageType === 'polygon') {
            const newPoints = [...polygonPoints, latlng]
            setPolygonPoints(newPoints)
            updateValue(center, 'polygon', 0, newPoints)
        } else {
            setCenter(latlng)
            updateValue(latlng, 'circle', radius, [])
        }
    }

    const updateValue = (
        newCenter: L.LatLng | null,
        type: 'circle' | 'polygon',
        newRadius: number,
        points: L.LatLng[]
    ) => {
        if (!newCenter && type === 'circle') return

        const coverage: CoverageArea = type === 'circle'
            ? {
                type: 'circle',
                center: newCenter ? { lat: newCenter.lat, lng: newCenter.lng } : undefined,
                radius: newRadius
            }
            : {
                type: 'polygon',
                coordinates: points.map(p => ({ lat: p.lat, lng: p.lng }))
            }

        onChange({
            lat: newCenter ? newCenter.lat : (points.length > 0 ? points[0].lat : 0),
            lng: newCenter ? newCenter.lng : (points.length > 0 ? points[0].lng : 0),
            coverage
        })
    }

    const handleRadiusChange = (val: number[]) => {
        setRadius(val[0])
        updateValue(center, 'circle', val[0], [])
    }

    const clearPolygon = () => {
        setPolygonPoints([])
        updateValue(center, 'polygon', 0, [])
    }

    return (
        <div className="space-y-4">
            <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                <RadioGroup
                    value={coverageType}
                    onValueChange={(v) => {
                        const type = v as 'circle' | 'polygon'
                        setCoverageType(type)
                        updateValue(center, type, radius, polygonPoints)
                    }}
                    className="flex items-center space-x-4"
                >
                    <div className="flex items-center space-x-2">
                        <RadioGroupItem value="circle" id="circle" />
                        <Label htmlFor="circle">Circle (Radius)</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                        <RadioGroupItem value="polygon" id="polygon" />
                        <Label htmlFor="polygon">Polygon (Custom Shape)</Label>
                    </div>
                </RadioGroup>

                {coverageType === 'circle' && (
                    <div className="flex items-center gap-4 w-full sm:w-1/2">
                        <Label>Radius: {radius}m</Label>
                        <Slider
                            value={[radius]}
                            onValueChange={handleRadiusChange}
                            max={10000}
                            step={100}
                            className="flex-1"
                        />
                    </div>
                )}

                {coverageType === 'polygon' && (
                    <Button type="button" variant="outline" size="sm" onClick={clearPolygon}>
                        Clear Points
                    </Button>
                )}
            </div>

            <div className="h-[400px] w-full rounded-md border overflow-hidden">
                <MapContainer
                    center={center || defaultCenter}
                    zoom={11}
                    style={{ height: '100%', width: '100%' }}
                >
                    <TileLayer
                        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                    />
                    <MapEvents onMapClick={handleMapClick} />

                    {center && coverageType === 'circle' && (
                        <>
                            <Marker position={center} />
                            <Circle center={center} radius={radius} />
                        </>
                    )}

                    {coverageType === 'polygon' && polygonPoints.length > 0 && (
                        <>
                            {polygonPoints.map((p, i) => (
                                <Marker key={i} position={p} opacity={0.5} />
                            ))}
                            <Polygon positions={polygonPoints} />
                        </>
                    )}
                </MapContainer>
            </div>
            <p className="text-xs text-muted-foreground">
                {coverageType === 'circle'
                    ? "Click on the map to set the center point."
                    : "Click on the map to add points to the polygon."}
            </p>
        </div>
    )
}
