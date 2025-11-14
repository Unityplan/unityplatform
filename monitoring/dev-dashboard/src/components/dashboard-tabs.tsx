import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { DashboardView } from "./dashboard-view"
import { ServicesTable } from "./services-table"
import { LayoutDashboard, Table } from "lucide-react"

export function DashboardTabs() {
  return (
    <Tabs defaultValue="dashboard" className="w-full">
      <TabsList>
        <TabsTrigger value="dashboard">
          <LayoutDashboard className="mr-2 h-4 w-4" />
          Dashboard
        </TabsTrigger>
        <TabsTrigger value="services">
          <Table className="mr-2 h-4 w-4" />
          Services
        </TabsTrigger>
      </TabsList>
      
      <TabsContent value="dashboard" className="mt-6">
        <DashboardView />
      </TabsContent>
      
      <TabsContent value="services" className="mt-6">
        <ServicesTable />
      </TabsContent>
    </Tabs>
  )
}
