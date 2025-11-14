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
import { services, type Service, type ServiceStatus } from "@/lib/services"
import { ExternalLink } from "lucide-react"

export function ServicesTable() {
  const [servicesWithStatus, setServicesWithStatus] = useState<Service[]>([])

  useEffect(() => {
    const checkServices = async () => {
      const updated = await Promise.all(
        services.map(async (service) => ({
          ...service,
          status: await checkServiceStatus(service),
        }))
      )
      setServicesWithStatus(updated)
    }

    checkServices()
    const interval = setInterval(checkServices, 30000)

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
            <div className="text-2xl font-bold text-green-600 dark:text-green-400">{operational}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle className="text-sm font-medium">Down</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-red-600 dark:text-red-400">{down}</div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle className="text-sm font-medium">Not Deployed</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-gray-600 dark:text-gray-400">{notDeployed}</div>
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
                <TableHead>Status</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {servicesWithStatus.map((service) => (
                <TableRow key={service.name}>
                  <TableCell className="text-2xl">{service.icon}</TableCell>
                  <TableCell className="font-medium">{service.name}</TableCell>
                  <TableCell>{service.category}</TableCell>
                  <TableCell className="font-mono text-sm">{service.port}</TableCell>
                  <TableCell className="text-sm text-muted-foreground">{service.description}</TableCell>
                  <TableCell>{getStatusBadge(service.status || 'checking', service.deployed)}</TableCell>
                  <TableCell className="text-right">
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

async function checkServiceStatus(service: Service): Promise<ServiceStatus> {
  if (!service.deployed) {
    return 'not-deployed'
  }

  try {
    const controller = new AbortController()
    const timeout = setTimeout(() => controller.abort(), 3000)

    await fetch(service.url, {
      method: 'HEAD',
      mode: 'no-cors',
      signal: controller.signal,
    })

    clearTimeout(timeout)
    return 'operational'
  } catch {
    return 'down'
  }
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
