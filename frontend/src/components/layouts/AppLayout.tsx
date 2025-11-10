import type React from "react"
import { SidebarProvider, SidebarInset, SidebarTrigger } from "@/components/ui/sidebar"
import { AppSidebar } from "@/components/app-sidebar"
import { ModeToggle } from "@/components/mode-toggle"
import { Separator } from "@/components/ui/separator"
import { Button } from "@/components/ui/button"
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import { Bell, User, Settings, LogOut, Shield, HelpCircle, Maximize2, Minimize2 } from "lucide-react"
import { useAuthStore } from "@/stores/authStore"
import { useState, useEffect } from "react"
import { useRouter } from "@tanstack/react-router"

interface AppLayoutProps {
    children: React.ReactNode
    breadcrumbs?: React.ReactNode
}

export function AppLayout({ breadcrumbs, children }: AppLayoutProps) {
    const { user } = useAuthStore()
    const [isOnline, setIsOnline] = useState(true)
    const [isFullWidth, setIsFullWidth] = useState(() => {
        // Load from localStorage on initial render
        if (typeof localStorage !== 'undefined') {
            const saved = localStorage.getItem("wideContentView")
            console.log('Initial wideContentView from localStorage:', saved)
            return saved === "true"
        }
        return false
    })
    const router = useRouter()

    // Sync with localStorage changes and custom events
    useEffect(() => {
        const handleStorageChange = () => {
            const savedWideContentView = localStorage.getItem("wideContentView") === "true"
            console.log('Storage change detected, new value:', savedWideContentView)
            setIsFullWidth(savedWideContentView)
        }

        const handleWideContentViewChange = (event: Event) => {
            const customEvent = event as CustomEvent<{ value: boolean }>
            console.log('Custom event received, new value:', customEvent.detail.value)
            setIsFullWidth(customEvent.detail.value)
        }

        window.addEventListener('storage', handleStorageChange)
        window.addEventListener('wideContentViewChange', handleWideContentViewChange)

        return () => {
            window.removeEventListener('storage', handleStorageChange)
            window.removeEventListener('wideContentViewChange', handleWideContentViewChange)
        }
    }, [])

    const handleLogout = () => {
        useAuthStore.getState().logout()
        router.navigate({ to: '/login' })
    }

    return (
        <SidebarProvider defaultOpen={false}>
            <AppSidebar />
            <SidebarInset>
                <header className="flex h-16 shrink-0 items-center gap-2 px-4">
                    <SidebarTrigger className="-ml-1 bg-transparent hover:bg-accent text-foreground" />
                    <Separator orientation="vertical" className="mr-2 h-4" />
                    {breadcrumbs}
                    <div className="ml-auto flex items-center gap-2">
                        {/* Notifications */}
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <Button size="icon" className="relative bg-transparent hover:bg-accent text-foreground">
                                    <Bell className="size-5" />
                                    <span className="absolute top-1 right-1 size-2 rounded-full bg-destructive" />
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end" className="w-80 text-base border border-border shadow-sm">
                                <DropdownMenuLabel className="text-base">Notifications</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem>
                                    <div className="flex flex-col gap-1">
                                        <p className="text-base font-medium">No new notifications</p>
                                        <p className="text-sm text-muted-foreground">You're all caught up!</p>
                                    </div>
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>

                        {/* Online Status Avatar */}
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <Button size="icon" className="relative bg-transparent hover:bg-accent text-foreground">
                                    <Avatar className="size-8">
                                        <AvatarFallback>
                                            {user?.username?.charAt(0).toUpperCase() || 'U'}
                                        </AvatarFallback>
                                    </Avatar>
                                    {isOnline && (
                                        <span className="absolute top-1 right-1 size-2 rounded-full bg-green-500" />
                                    )}
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end" className="w-56 text-base border border-border shadow-sm">
                                <DropdownMenuLabel className="text-base">
                                    <div className="flex flex-col">
                                        <span>{user?.username}</span>
                                        <span className="text-sm font-normal text-muted-foreground">
                                            {user?.email}
                                        </span>
                                    </div>
                                </DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem onClick={() => setIsOnline(!isOnline)}>
                                    <div className="flex items-center gap-2 w-full">
                                        <div className={`size-2 rounded-full ${isOnline ? 'bg-green-500' : 'bg-gray-400'}`} />
                                        <span>{isOnline ? 'Online' : 'Offline'}</span>
                                    </div>
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem onClick={() => window.location.href = '/profile'}>
                                    <User className="mr-2 size-5" />
                                    <span>Profile</span>
                                </DropdownMenuItem>
                                <DropdownMenuItem onClick={() => window.location.href = '/settings'}>
                                    <Settings className="mr-2 size-5" />
                                    <span>Settings</span>
                                </DropdownMenuItem>
                                <DropdownMenuItem onClick={() => window.location.href = '/settings/privacy'}>
                                    <Shield className="mr-2 size-5" />
                                    <span>Privacy</span>
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    <HelpCircle className="mr-2 size-5" />
                                    <span>Help & Support</span>
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem onClick={handleLogout}>
                                    <LogOut className="mr-2 size-5" />
                                    <span>Log out</span>
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>

                        <Separator orientation="vertical" className="h-6" />

                        {/* Full Width Toggle */}
                        <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => {
                                const newValue = !isFullWidth
                                console.log('Button clicked, toggling from', isFullWidth, 'to', newValue)
                                setIsFullWidth(newValue)
                                localStorage.setItem("wideContentView", String(newValue))
                            }}
                            title={isFullWidth ? "Constrain width" : "Full width"}
                        >
                            {isFullWidth ? <Minimize2 className="size-5" /> : <Maximize2 className="size-5" />}
                        </Button>

                        <Separator orientation="vertical" className="h-6" />
                        <ModeToggle />
                    </div>
                </header>
                <div className="flex flex-1 flex-col gap-4 p-4">
                    <div className={isFullWidth ? "w-full" : "mx-auto w-full max-w-7xl"} key={isFullWidth ? 'full' : 'constrained'}>
                        {children}
                    </div>
                </div>
            </SidebarInset>
        </SidebarProvider>
    )
}
