import { useEffect, useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { fetchServicesStatus, type Service, type ServiceStatus, type ServiceCategory } from "@/lib/services-api"
import { ExternalLink } from "lucide-react"

export function DashboardView() {
    const [servicesWithStatus, setServicesWithStatus] = useState<Service[]>([])

    useEffect(() => {
        const loadServices = async () => {
            const services = await fetchServicesStatus()
            setServicesWithStatus(services)
        }

        loadServices()
        const interval = setInterval(loadServices, 30000) // Refresh every 30 seconds

        return () => clearInterval(interval)
    }, [])

    const categories: ServiceCategory[] = ['Application', 'Backend Services', 'Infrastructure', 'Development Tools', 'Observability']

    return (
        <div className="space-y-8">
            {categories.map((category) => {
                const categoryServices = servicesWithStatus.filter((s) => s.category === category)
                if (categoryServices.length === 0) return null

                return (
                    <div key={category}>
                        <h2 className="text-2xl font-semibold mb-4 flex items-center gap-2">
                            {getCategoryIcon(category)}
                            {category}
                        </h2>
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                            {categoryServices.map((service) => (
                                <ServiceCard key={service.name} service={service} />
                            ))}
                        </div>
                    </div>
                )
            })}
        </div>
    )
}

function ServiceCard({ service }: { service: Service }) {
    const statusBadge = getStatusBadge(service.status || 'checking', service.deployed)

    return (
        <Card className="hover:shadow-lg transition-shadow">
            <CardHeader>
                <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2">
                        <span className="text-2xl">{service.icon}</span>
                        <CardTitle className="text-lg">{service.name}</CardTitle>
                    </div>
                    {statusBadge}
                </div>
                <CardDescription className="text-sm">{service.description}</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">Port: {service.port}</span>
                    {service.deployed && (
                        <a
                            href={service.url}
                            target="_blank"
                            rel="noopener noreferrer"
                            className="flex items-center gap-1 text-primary hover:underline"
                        >
                            Open
                            <ExternalLink className="h-3 w-3" />
                        </a>
                    )}
                </div>
            </CardContent>
        </Card>
    )
}

function getStatusBadge(status: ServiceStatus, deployed: boolean) {
    if (!deployed) {
        return <Badge variant="outline">Not Deployed</Badge>
    }

    switch (status) {
        case 'operational':
            return <Badge variant="success">Operational</Badge>
        case 'down':
            return <Badge variant="destructive">Down</Badge>
        case 'checking':
            return <Badge variant="secondary">Checking...</Badge>
        default:
            return <Badge variant="outline">Unknown</Badge>
    }
}

function getCategoryIcon(category: ServiceCategory): string {
    const icons: Record<ServiceCategory, string> = {
        'Application': '⚛️',
        'Backend Services': '🦀',
        'Infrastructure': '🗄️',
        'Development Tools': '🛠️',
        'Observability': '📊',
    }
    return icons[category]
}
