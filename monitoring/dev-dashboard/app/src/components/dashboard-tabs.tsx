import { useEffect, useState } from "react"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { IssuesView } from "./issues-view"
import { DashboardView } from "./dashboard-view"
import { ServicesTable } from "./services-table"
import { AlertCircle, LayoutDashboard, Table } from "lucide-react"
import { fetchServicesStatus, type Service } from "@/lib/services-api"

export function DashboardTabs() {
    const [services, setServices] = useState<Service[]>([])

    useEffect(() => {
        const loadServices = async () => {
            const loadedServices = await fetchServicesStatus()
            setServices(loadedServices)
        }

        loadServices()
        const interval = setInterval(loadServices, 30000) // Refresh every 30 seconds

        return () => clearInterval(interval)
    }, [])

    return (
        <Tabs defaultValue="issues" className="w-full">
            <TabsList>
                <TabsTrigger value="issues">
                    <AlertCircle className="mr-2 h-4 w-4" />
                    Issues
                </TabsTrigger>
                <TabsTrigger value="dashboard">
                    <LayoutDashboard className="mr-2 h-4 w-4" />
                    Dashboard
                </TabsTrigger>
                <TabsTrigger value="services">
                    <Table className="mr-2 h-4 w-4" />
                    All Services
                </TabsTrigger>
            </TabsList>

            <TabsContent value="issues" className="mt-6">
                <IssuesView services={services} />
            </TabsContent>

            <TabsContent value="dashboard" className="mt-6">
                <DashboardView />
            </TabsContent>

            <TabsContent value="services" className="mt-6">
                <ServicesTable />
            </TabsContent>
        </Tabs>
    )
}
