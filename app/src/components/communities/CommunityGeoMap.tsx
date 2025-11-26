import { MapContainer, TileLayer, Marker, Popup, Circle, Polygon } from 'react-leaflet'
import { useQuery } from '@tanstack/react-query'
import { communityService, CommunityType } from '@/api/community'
import { Loader2, Map as MapIcon, Users, Hammer, BookOpen } from 'lucide-react'
import { Link } from '@tanstack/react-router'
import { Icon, DivIcon } from 'leaflet'
import { renderToStaticMarkup } from 'react-dom/server'
import 'leaflet/dist/leaflet.css'

// Fix for default marker icon in React Leaflet
import markerIcon2x from 'leaflet/dist/images/marker-icon-2x.png'
import markerIcon from 'leaflet/dist/images/marker-icon.png'
import markerShadow from 'leaflet/dist/images/marker-shadow.png'

// @ts-expect-error - Leaflet icon fix
delete Icon.Default.prototype._getIconUrl
Icon.Default.mergeOptions({
    iconRetinaUrl: markerIcon2x,
    iconUrl: markerIcon,
    shadowUrl: markerShadow,
})

function getCommunityColor(type: CommunityType) {
    switch (type) {
        case CommunityType.Zone: return '#2563eb' // blue-600
        case CommunityType.Neighborhood: return '#16a34a' // green-600
        case CommunityType.Guild: return '#d97706' // amber-600
        case CommunityType.StudyGroup: return '#9333ea' // purple-600
        default: return '#4b5563' // gray-600
    }
}

function createCustomIcon(type: CommunityType, isTopLevel: boolean = false) {
    let IconComponent;
    let colorClass;
    let borderClass;

    if (isTopLevel) {
        IconComponent = MapIcon;
        colorClass = 'text-red-500';
        borderClass = 'border-red-400';
    } else {
        switch (type) {
            case CommunityType.Zone:
                IconComponent = MapIcon;
                colorClass = 'text-blue-600';
                borderClass = 'border-blue-500';
                break;
            case CommunityType.Neighborhood:
                IconComponent = Users;
                colorClass = 'text-green-600';
                borderClass = 'border-green-500';
                break;
            case CommunityType.Guild:
                IconComponent = Hammer;
                colorClass = 'text-amber-600';
                borderClass = 'border-amber-500';
                break;
            case CommunityType.StudyGroup:
                IconComponent = BookOpen;
                colorClass = 'text-purple-600';
                borderClass = 'border-purple-500';
                break;
            default:
                IconComponent = Users;
                colorClass = 'text-gray-600';
                borderClass = 'border-gray-500';
        }
    }

    const iconHtml = renderToStaticMarkup(
        <div className="relative flex flex-col items-center justify-center drop-shadow-md">
            <div
                className={`relative z-10 flex h-8 w-8 items-center justify-center rounded-full border-2 ${borderClass}`}
                style={{ backgroundColor: '#ffffff' }}
            >
                <IconComponent className={`h-4 w-4 ${colorClass}`} />
            </div>
            <div
                className={`-mt-3.5 h-5 w-5 rotate-45 border-b-2 border-r-2 ${borderClass}`}
                style={{ backgroundColor: '#ffffff' }}
            ></div>
        </div>
    );

    return new DivIcon({
        html: iconHtml,
        className: 'bg-transparent', // Remove default leaflet div icon background
        iconSize: [32, 50],
        iconAnchor: [16, 50], // Tip is at bottom
        popupAnchor: [0, -50],
    });
} export function CommunityGeoMap() {
    const { data: communities, isLoading } = useQuery({
        queryKey: ['communities'],
        queryFn: () => communityService.listCommunities({}),
    })

    if (isLoading) {
        return (
            <div className="flex h-[600px] items-center justify-center rounded-lg border bg-muted/10">
                <Loader2 className="h-8 w-8 animate-spin text-primary" />
            </div>
        )
    }

    // Filter for communities with location data
    const locatedCommunities = communities?.filter(
        (c) => c.location_lat && c.location_lng
    ) || []

    // Center on Denmark (default)
    const center: [number, number] = [56.2639, 9.5018]

    return (
        <div className="h-[600px] lg:h-[800px] w-full overflow-hidden rounded-lg border shadow-sm relative">
            {/* Force light mode styles for Leaflet controls since we use light tiles */}
            <style>{`
                .leaflet-control-attribution {
                    background: rgba(255, 255, 255, 0.8) !important;
                    color: #333 !important;
                }
                .leaflet-control-attribution a {
                    color: #0078A8 !important;
                }
                .leaflet-popup-content-wrapper, .leaflet-popup-tip {
                    background-color: rgba(255, 255, 255, 1) !important;
                    backdrop-filter: blur(2px);
                    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1) !important;
                    color: #000000 !important;
                }
                .leaflet-popup-content {
                    background-color: rgba(255, 255, 255, 1) !important;
                    margin: 16px !important;
                    color: #000000 !important;
                }
                .leaflet-popup-content h3 {
                    color: #000000 !important;
                }
                .leaflet-popup-content p {
                    color: #4b5563 !important;
                }
            `}</style>
            <MapContainer
                center={center}
                zoom={7}
                style={{ height: '100%', width: '100%', background: '#e5e7eb' }}
                className="text-black"
            >
                <TileLayer
                    attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                    url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />

                {locatedCommunities.map((community) => {
                    const position: [number, number] = [community.location_lat!, community.location_lng!]
                    const isTopLevel = community.type === CommunityType.Zone && !community.parent_community_id

                    let color = getCommunityColor(community.type)
                    if (isTopLevel) {
                        color = '#f87171' // Light red (red-400)
                    }

                    // Determine style based on community type
                    let pathOptions = { color: color, fillColor: color, fillOpacity: 0.1 }

                    if (isTopLevel) {
                        // Top level zone: Outline only, transparent fill
                        pathOptions = { color: color, fillColor: 'transparent', fillOpacity: 0 }
                    }

                    return (
                        <div key={community.id}>
                            <Marker position={position} icon={createCustomIcon(community.type, isTopLevel)}>
                                <Popup>
                                    <div className="min-w-[200px]">
                                        <h3 className="font-semibold">{community.name}</h3>
                                        <p className="text-sm text-muted-foreground mb-2">
                                            {community.description}
                                        </p>
                                        <div className="flex items-center justify-between">
                                            <span className="text-xs bg-secondary px-2 py-1 rounded-full">
                                                {community.member_count} members
                                            </span>
                                            <Link
                                                to="/communities/$communityId/dashboard"
                                                params={{ communityId: community.id }}
                                                className="text-sm text-primary hover:underline"
                                            >
                                                View Dashboard
                                            </Link>
                                        </div>
                                    </div>
                                </Popup>
                            </Marker>

                            {community.coverage_area?.type === 'circle' && community.coverage_area.radius && (
                                <Circle
                                    center={position}
                                    radius={community.coverage_area.radius}
                                    pathOptions={pathOptions}
                                />
                            )}

                            {community.coverage_area?.type === 'polygon' && community.coverage_area.coordinates && (
                                <Polygon
                                    positions={community.coverage_area.coordinates.map(p => [p.lat, p.lng])}
                                    pathOptions={pathOptions}
                                />
                            )}
                        </div>
                    )
                })}
            </MapContainer>
        </div>
    )
}
