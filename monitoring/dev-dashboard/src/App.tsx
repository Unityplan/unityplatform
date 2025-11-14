import { ThemeProvider } from "@/components/theme-provider"
import { DashboardTabs } from "@/components/dashboard-tabs"
import { ThemeToggle } from "@/components/theme-toggle"

function App() {
  return (
    <ThemeProvider defaultTheme="light" storageKey="dashboard-theme">
      <div className="min-h-screen bg-background">
        <div className="container mx-auto py-8 px-4">
          <header className="mb-8">
            <div className="flex items-center justify-between">
              <div>
                <h1 className="text-4xl font-bold text-foreground mb-2">
                  Unity Platform
                </h1>
                <p className="text-muted-foreground text-lg">
                  Development Dashboard
                </p>
              </div>
              <ThemeToggle />
            </div>
          </header>

          <DashboardTabs />
        </div>
      </div>
    </ThemeProvider>
  )
}

export default App
