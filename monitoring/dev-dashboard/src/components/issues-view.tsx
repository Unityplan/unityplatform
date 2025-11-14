import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ExternalLink, AlertCircle, CheckCircle } from "lucide-react"
import { type Service } from "@/lib/services-api"

interface IssuesViewProps {
    services: Service[]
}

export function IssuesView({ services }: IssuesViewProps) {
    const downServices = services.filter(s => s.status === 'down')
    const notDeployedServices = services.filter(s => s.status === 'not-deployed')
    const operationalCount = services.filter(s => s.status === 'operational').length
    const totalCount = services.length

    if (downServices.length === 0 && notDeployedServices.length === 0) {
        return (
            <div className="flex flex-col items-center justify-center py-12">
                <CheckCircle className="h-16 w-16 text-primary mb-4" />
                <h2 className="text-2xl font-semibold mb-2">All Systems Operational</h2>
                <p className="text-muted-foreground">
                    {operationalCount} of {totalCount} services are running normally
                </p>
            </div>
        )
    }

    return (
        <div className="space-y-6">
            {/* Summary Cards */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card>
                    <CardHeader className="pb-2">
                        <CardDescription>Operational</CardDescription>
                        <CardTitle className="text-4xl font-bold text-primary">{operationalCount}</CardTitle>
                    </CardHeader>
                </Card>
                <Card>
                    <CardHeader className="pb-2">
                        <CardDescription>Down</CardDescription>
                        <CardTitle className="text-4xl font-bold text-destructive">{downServices.length}</CardTitle>
                    </CardHeader>
                </Card>
                <Card>
                    <CardHeader className="pb-2">
                        <CardDescription>Not Deployed</CardDescription>
                        <CardTitle className="text-4xl font-bold text-muted-foreground">{notDeployedServices.length}</CardTitle>
                    </CardHeader>
                </Card>
            </div>

            {/* Down Services */}
            {downServices.length > 0 && (
                <Card className="border-destructive">
                    <CardHeader>
                        <div className="flex items-center gap-2">
                            <AlertCircle className="h-5 w-5 text-destructive" />
                            <CardTitle className="text-destructive">
                                Services Down ({downServices.length})
                            </CardTitle>
                        </div>
                        <CardDescription>
                            These services are not responding and need attention
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="space-y-2">
                            {downServices.map((service) => (
                                <ServiceRow key={service.name} service={service} status="down" />
                            ))}
                        </div>
                    </CardContent>
                </Card>
            )}

            {/* Not Deployed Services */}
            {notDeployedServices.length > 0 && (
                <Card>
                    <CardHeader>
                        <CardTitle className="text-gray-600 dark:text-gray-400">
                            Not Deployed ({notDeployedServices.length})
                        </CardTitle>
                        <CardDescription>
                            These services are planned but not yet deployed
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="space-y-2">
                            {notDeployedServices.map((service) => (
                                <ServiceRow key={service.name} service={service} status="not-deployed" />
                            ))}
                        </div>
                    </CardContent>
                </Card>
            )}
        </div>
    )
}

function ServiceRow({ service, status }: { service: Service; status: 'down' | 'not-deployed' }) {
    return (
        <div className="flex items-center justify-between p-3 rounded-lg border bg-card hover:bg-accent/50 transition-colors">
            <div className="flex items-center gap-3 flex-1">
                <span className="text-2xl">{service.icon}</span>
                <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                        <p className="font-medium truncate">{service.name}</p>
                        {status === 'down' ? (
                            <Badge variant="destructive" className="shrink-0">Down</Badge>
                        ) : (
                            <Badge variant="outline" className="shrink-0">Not Deployed</Badge>
                        )}
                    </div>
                    <p className="text-sm text-muted-foreground truncate">{service.description}</p>
                </div>
            </div>
            <div className="flex items-center gap-3 ml-3 shrink-0">
                <span className="text-sm text-muted-foreground">Port: {service.port}</span>
                {service.deployed && (
                    <a
                        href={service.url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="flex items-center gap-1 text-sm text-primary hover:underline"
                    >
                        Open
                        <ExternalLink className="h-3 w-3" />
                    </a>
                )}
            </div>
        </div>
    )
}
