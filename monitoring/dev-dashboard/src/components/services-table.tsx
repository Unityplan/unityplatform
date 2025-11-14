import { useEffect, useState } from "react"
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { fetchServicesStatus, type Service, type ServiceStatus } from "@/lib/services-api"
import { ExternalLink } from "lucide-react"

export function ServicesTable() {
    const [servicesWithStatus, setServicesWithStatus] = useState<Service[]>([])

    useEffect(() => {
        const loadServices = async () => {
            const services = await fetchServicesStatus()
            setServicesWithStatus(services)
        }

        loadServices()
        const interval = setInterval(loadServices, 30000)

        return () => clearInterval(interval)
    }, [])

    const operational = servicesWithStatus.filter(s => s.status === 'operational').length
    const down = servicesWithStatus.filter(s => s.status === 'down').length
    const notDeployed = servicesWithStatus.filter(s => !s.deployed).length

    return (
        <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card>
                    <CardHeader>
                        <CardTitle className="text-sm font-medium">Operational</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="text-4xl font-bold text-primary">{operational}</div>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader>
                        <CardTitle className="text-sm font-medium">Down</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="text-4xl font-bold text-destructive">{down}</div>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader>
                        <CardTitle className="text-sm font-medium">Not Deployed</CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="text-4xl font-bold text-muted-foreground">{notDeployed}</div>
                    </CardContent>
                </Card>
            </div>

            <Card>
                <CardHeader>
                    <CardTitle>All Services</CardTitle>
                    <CardDescription>Complete list of Unity Platform services and their status</CardDescription>
                </CardHeader>
                <CardContent>
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead className="w-[50px]"></TableHead>
                                <TableHead>Service</TableHead>
                                <TableHead>Category</TableHead>
                                <TableHead>Port</TableHead>
                                <TableHead>Description</TableHead>
                                <TableHead>CPU %</TableHead>
                                <TableHead>Memory</TableHead>
                                <TableHead>Network I/O</TableHead>
                                <TableHead>Disk I/O</TableHead>
                                <TableHead>Status</TableHead>
                                <TableHead className="text-right">Actions</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {servicesWithStatus.map((service) => (
                                <TableRow key={service.name}>
                                    <TableCell className="text-2xl">{service.icon}</TableCell>
                                    <TableCell className="font-medium whitespace-nowrap">{service.name}</TableCell>
                                    <TableCell className="whitespace-nowrap">{service.category}</TableCell>
                                    <TableCell className="font-mono text-sm whitespace-nowrap">{service.port}</TableCell>
                                    <TableCell className="text-sm text-muted-foreground">{service.description}</TableCell>
                                    <TableCell className="font-mono text-sm whitespace-nowrap">
                                        {service.resources?.cpu_usage_percent !== undefined
                                            ? `${service.resources.cpu_usage_percent.toFixed(2)}%`
                                            : '—'}
                                    </TableCell>
                                    <TableCell className="font-mono text-sm whitespace-nowrap">
                                        {service.resources?.memory_usage_mb !== undefined && service.resources?.memory_percent !== undefined
                                            ? `${service.resources.memory_usage_mb.toFixed(0)} MB (${service.resources.memory_percent.toFixed(1)}%)`
                                            : '—'}
                                    </TableCell>
                                    <TableCell className="font-mono text-sm whitespace-nowrap">
                                        {service.resources?.network_rx_mb !== undefined && service.resources?.network_tx_mb !== undefined
                                            ? `↓${service.resources.network_rx_mb.toFixed(1)} / ↑${service.resources.network_tx_mb.toFixed(1)} MB`
                                            : '—'}
                                    </TableCell>
                                    <TableCell className="font-mono text-sm whitespace-nowrap">
                                        {service.resources?.block_read_mb !== undefined && service.resources?.block_write_mb !== undefined
                                            ? `R:${service.resources.block_read_mb.toFixed(1)} / W:${service.resources.block_write_mb.toFixed(1)} MB`
                                            : '—'}
                                    </TableCell>
                                    <TableCell className="whitespace-nowrap">{getStatusBadge(service.status || 'checking', service.deployed)}</TableCell>
                                    <TableCell className="text-right whitespace-nowrap">
                                        {service.deployed && (
                                            <a
                                                href={service.url}
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                className="inline-flex items-center gap-1 text-sm text-primary hover:underline"
                                            >
                                                Open
                                                <ExternalLink className="h-3 w-3" />
                                            </a>
                                        )}
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </CardContent>
            </Card>
        </div>
    )
}

function getStatusBadge(status: ServiceStatus, deployed: boolean) {
    if (!deployed) {
        return <Badge variant="outline">Not Deployed</Badge>
    }

    switch (status) {
        case 'operational':
            return <Badge variant="default">Operational</Badge>
        case 'down':
            return <Badge variant="destructive">Down</Badge>
        case 'checking':
            return <Badge variant="secondary">Checking...</Badge>
        default:
            return <Badge variant="outline">Unknown</Badge>
    }
}
