import { useRouter, Link } from "@tanstack/react-router"
import { Home, Settings as SettingsIcon, Shield, Users, Monitor, Clock, Trash2 } from "lucide-react"
import { AppLayout } from "@/components/layouts/AppLayout"
import { Card } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Switch } from "@/components/ui/switch"
import { Button } from "@/components/ui/button"
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { useState, useEffect } from "react"

interface Session {
    id: string
    device: string
    location: string
    lastActive: string
    current: boolean
}

export function SecuritySettingsPage() {
    const router = useRouter()

    // Recovery settings
    const [friendRecoveryEnabled, setFriendRecoveryEnabled] = useState(false)
    const [managerRecoveryEnabled, setManagerRecoveryEnabled] = useState(false)
    const [recoveryFriends, setRecoveryFriends] = useState<string[]>([])

    // Sessions
    const [sessions, setSessions] = useState<Session[]>([
        {
            id: "1",
            device: "Chrome on Linux",
            location: "Copenhagen, Denmark",
            lastActive: "Active now",
            current: true
        },
        {
            id: "2",
            device: "Firefox on Windows",
            location: "Aarhus, Denmark",
            lastActive: "2 hours ago",
            current: false
        },
        {
            id: "3",
            device: "Safari on iPhone",
            location: "Odense, Denmark",
            lastActive: "1 day ago",
            current: false
        }
    ])

    // Login history
    const [loginHistory] = useState([
        { date: "2025-11-10 14:30", location: "Copenhagen, Denmark", device: "Chrome on Linux", status: "Success" },
        { date: "2025-11-09 09:15", location: "Copenhagen, Denmark", device: "Chrome on Linux", status: "Success" },
        { date: "2025-11-08 18:45", location: "Aarhus, Denmark", device: "Firefox on Windows", status: "Success" },
        { date: "2025-11-08 08:20", location: "Unknown", device: "Unknown Browser", status: "Failed" },
        { date: "2025-11-07 12:00", location: "Odense, Denmark", device: "Safari on iPhone", status: "Success" },
    ])

    // Load saved settings
    useEffect(() => {
        const savedFriendRecovery = localStorage.getItem("friendRecoveryEnabled") === "true"
        const savedManagerRecovery = localStorage.getItem("managerRecoveryEnabled") === "true"
        const savedFriends = localStorage.getItem("recoveryFriends")

        setFriendRecoveryEnabled(savedFriendRecovery)
        setManagerRecoveryEnabled(savedManagerRecovery)
        if (savedFriends) {
            setRecoveryFriends(JSON.parse(savedFriends))
        }
    }, [])

    const handleFriendRecoveryToggle = (checked: boolean) => {
        setFriendRecoveryEnabled(checked)
        localStorage.setItem("friendRecoveryEnabled", String(checked))

        if (checked && recoveryFriends.length === 0) {
            // Mock: Add some default friends
            const defaultFriends = ["Alice Johnson", "Bob Smith"]
            setRecoveryFriends(defaultFriends)
            localStorage.setItem("recoveryFriends", JSON.stringify(defaultFriends))
        }
    }

    const handleManagerRecoveryToggle = (checked: boolean) => {
        setManagerRecoveryEnabled(checked)
        localStorage.setItem("managerRecoveryEnabled", String(checked))
    }

    const handleRevokeSession = (sessionId: string) => {
        if (confirm("Are you sure you want to revoke this session?")) {
            setSessions(sessions.filter(s => s.id !== sessionId))
            alert("Session revoked successfully")
        }
    }

    const handleRevokeAllSessions = () => {
        if (confirm("This will sign you out of all devices except this one. Continue?")) {
            setSessions(sessions.filter(s => s.current))
            alert("All other sessions have been revoked")
        }
    }

    const handleCancel = () => {
        router.history.back()
    }

    return (
        <AppLayout
            breadcrumbs={
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/"><Home className="size-4" /></Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbLink asChild>
                                <Link to="/settings" className="flex items-center gap-1">
                                    <SettingsIcon className="size-4" />
                                    <span>Settings</span>
                                </Link>
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>Security</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                <div>
                    <h1 className="text-2xl font-bold">Security Settings</h1>
                    <p className="text-muted-foreground mt-1">
                        Manage account recovery, active sessions, and login history
                    </p>
                </div>

                {/* Account Recovery */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Shield className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Account Recovery</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Friend-Based Recovery</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Allow trusted friends to help you recover your account
                                    </p>
                                </div>
                                <Switch
                                    checked={friendRecoveryEnabled}
                                    onCheckedChange={handleFriendRecoveryToggle}
                                />
                            </div>

                            {friendRecoveryEnabled && recoveryFriends.length > 0 && (
                                <div className="rounded-lg border p-4">
                                    <p className="text-sm font-medium mb-2">Trusted Friends</p>
                                    <div className="space-y-2">
                                        {recoveryFriends.map((friend, index) => (
                                            <div key={index} className="flex items-center justify-between py-2 border-b last:border-0">
                                                <div className="flex items-center gap-2">
                                                    <Users className="size-4 text-muted-foreground" />
                                                    <span className="text-sm">{friend}</span>
                                                </div>
                                                <Button
                                                    variant="ghost"
                                                    size="sm"
                                                    onClick={() => {
                                                        const updated = recoveryFriends.filter((_, i) => i !== index)
                                                        setRecoveryFriends(updated)
                                                        localStorage.setItem("recoveryFriends", JSON.stringify(updated))
                                                    }}
                                                >
                                                    <Trash2 className="size-4" />
                                                </Button>
                                            </div>
                                        ))}
                                    </div>
                                    <Button variant="outline" size="sm" className="mt-3">
                                        Add Friend
                                    </Button>
                                </div>
                            )}

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Manager-Assisted Recovery</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Allow community/territory managers to assist with account recovery
                                    </p>
                                </div>
                                <Switch
                                    checked={managerRecoveryEnabled}
                                    onCheckedChange={handleManagerRecoveryToggle}
                                />
                            </div>

                            {managerRecoveryEnabled && (
                                <div className="rounded-lg bg-muted p-4">
                                    <p className="text-sm text-muted-foreground">
                                        Manager-assisted recovery is enabled. Community or territory managers can help you recover your account if needed.
                                    </p>
                                </div>
                            )}
                        </div>
                    </div>
                </Card>

                {/* Active Sessions */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center justify-between">
                            <div className="flex items-center gap-2">
                                <Monitor className="size-5 text-muted-foreground" />
                                <h2 className="text-lg font-semibold">Active Sessions</h2>
                            </div>
                            <Button
                                variant="outline"
                                size="sm"
                                onClick={handleRevokeAllSessions}
                            >
                                Revoke All Others
                            </Button>
                        </div>

                        <div className="space-y-3">
                            {sessions.map((session) => (
                                <div
                                    key={session.id}
                                    className="flex items-center justify-between p-4 rounded-lg border"
                                >
                                    <div className="flex items-center gap-3">
                                        <Monitor className="size-5 text-muted-foreground" />
                                        <div>
                                            <p className="text-sm font-medium flex items-center gap-2">
                                                {session.device}
                                                {session.current && (
                                                    <span className="text-xs bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 px-2 py-0.5 rounded">
                                                        Current
                                                    </span>
                                                )}
                                            </p>
                                            <p className="text-xs text-muted-foreground">
                                                {session.location} • {session.lastActive}
                                            </p>
                                        </div>
                                    </div>
                                    {!session.current && (
                                        <Button
                                            variant="ghost"
                                            size="sm"
                                            onClick={() => handleRevokeSession(session.id)}
                                        >
                                            Revoke
                                        </Button>
                                    )}
                                </div>
                            ))}
                        </div>

                        <p className="text-sm text-muted-foreground">
                            These are all the devices that are currently signed into your account. Remove any unfamiliar sessions.
                        </p>
                    </div>
                </Card>

                {/* Login History */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Clock className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Login History</h2>
                        </div>

                        <div className="space-y-2">
                            {loginHistory.map((login, index) => (
                                <div
                                    key={index}
                                    className="flex items-center justify-between py-3 border-b last:border-0"
                                >
                                    <div>
                                        <p className="text-sm font-medium">{login.date}</p>
                                        <p className="text-xs text-muted-foreground">
                                            {login.device} • {login.location}
                                        </p>
                                    </div>
                                    <span
                                        className={`text-xs px-2 py-1 rounded ${login.status === "Success"
                                                ? "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400"
                                                : "bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400"
                                            }`}
                                    >
                                        {login.status}
                                    </span>
                                </div>
                            ))}
                        </div>

                        <p className="text-sm text-muted-foreground">
                            Recent login attempts to your account. If you notice any suspicious activity, consider changing your password immediately.
                        </p>
                    </div>
                </Card>

                {/* Action Buttons */}
                <div className="flex gap-2">
                    <Button variant="outline" onClick={handleCancel}>
                        Back to Settings
                    </Button>
                </div>
            </div>
        </AppLayout>
    )
}
