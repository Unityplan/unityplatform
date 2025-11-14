import { Home, User, Settings, Shield } from "lucide-react"
import { Link, useRouterState } from "@tanstack/react-router"

import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@/components/ui/sidebar"

// Menu items
const mainItems = [
    {
        title: "Dashboard",
        path: "/dashboard",
        icon: Home,
    },
    {
        title: "Profile",
        path: "/profile",
        icon: User,
    },
]

const settingsItems = [
    {
        title: "Settings",
        path: "/settings",
        icon: Settings,
    },
    {
        title: "Privacy",
        path: "/settings/privacy",
        icon: Shield,
    },
    {
        title: "Edit Profile",
        path: "/profile/edit",
        icon: User,
    },
]

export function AppSidebar() {
    const router = useRouterState()
    const pathname = router.location.pathname

    return (
        <Sidebar>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>Main</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {mainItems.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild isActive={pathname === item.path}>
                                        <Link to={item.path}>
                                            <item.icon />
                                            <span>{item.title}</span>
                                        </Link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
                <SidebarGroup>
                    <SidebarGroupLabel>Settings</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {settingsItems.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild isActive={pathname === item.path}>
                                        <Link to={item.path}>
                                            <item.icon />
                                            <span>{item.title}</span>
                                        </Link>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    )
}
