import { useRouter, Link } from "@tanstack/react-router"
import { Home, Settings as SettingsIcon, Bell, Mail, Smartphone } from "lucide-react"
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

interface NotificationSettings {
    // Email notifications
    emailDigest: boolean
    emailMessages: boolean
    emailFollowers: boolean
    emailCommunity: boolean
    emailUpdates: boolean

    // In-app notifications
    inAppMessages: boolean
    inAppFollowers: boolean
    inAppCommunity: boolean
    inAppMentions: boolean
    inAppLikes: boolean

    // Push notifications
    pushEnabled: boolean
    pushMessages: boolean
    pushFollowers: boolean
    pushCommunity: boolean
}

export function NotificationSettingsPage() {
    const router = useRouter()

    const [settings, setSettings] = useState<NotificationSettings>({
        emailDigest: true,
        emailMessages: true,
        emailFollowers: true,
        emailCommunity: false,
        emailUpdates: true,
        inAppMessages: true,
        inAppFollowers: true,
        inAppCommunity: true,
        inAppMentions: true,
        inAppLikes: false,
        pushEnabled: false,
        pushMessages: false,
        pushFollowers: false,
        pushCommunity: false,
    })

    // Load saved settings
    useEffect(() => {
        const saved = localStorage.getItem("notificationSettings")
        if (saved) {
            setSettings(JSON.parse(saved))
        }
    }, [])

    const handleToggle = (key: keyof NotificationSettings) => (checked: boolean) => {
        const newSettings = { ...settings, [key]: checked }
        setSettings(newSettings)
        localStorage.setItem("notificationSettings", JSON.stringify(newSettings))
    }

    const handleSave = () => {
        localStorage.setItem("notificationSettings", JSON.stringify(settings))
        alert("Notification preferences saved successfully")
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
                            <BreadcrumbPage>Notifications</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                <div>
                    <h1 className="text-2xl font-bold">Notification Preferences</h1>
                    <p className="text-muted-foreground mt-1">
                        Choose how and when you want to be notified
                    </p>
                </div>

                {/* Email Notifications */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Mail className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Email Notifications</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Daily Digest</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Receive a daily summary of activity
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailDigest}
                                    onCheckedChange={handleToggle("emailDigest")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>New Messages</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Get notified when you receive a direct message
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailMessages}
                                    onCheckedChange={handleToggle("emailMessages")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>New Followers</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Get notified when someone follows you
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailFollowers}
                                    onCheckedChange={handleToggle("emailFollowers")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Community Updates</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Important announcements from your communities
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailCommunity}
                                    onCheckedChange={handleToggle("emailCommunity")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Platform Updates</Label>
                                    <p className="text-sm text-muted-foreground">
                                        News and updates about Unity Platform
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailUpdates}
                                    onCheckedChange={handleToggle("emailUpdates")}
                                />
                            </div>
                        </div>
                    </div>
                </Card>

                {/* In-App Notifications */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Bell className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">In-App Notifications</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Direct Messages</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Show notifications for new messages
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inAppMessages}
                                    onCheckedChange={handleToggle("inAppMessages")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>New Followers</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Show notifications when someone follows you
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inAppFollowers}
                                    onCheckedChange={handleToggle("inAppFollowers")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Community Activity</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Updates from communities you're part of
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inAppCommunity}
                                    onCheckedChange={handleToggle("inAppCommunity")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Mentions</Label>
                                    <p className="text-sm text-muted-foreground">
                                        When someone mentions you in a post or comment
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inAppMentions}
                                    onCheckedChange={handleToggle("inAppMentions")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Likes & Reactions</Label>
                                    <p className="text-sm text-muted-foreground">
                                        When someone likes or reacts to your content
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inAppLikes}
                                    onCheckedChange={handleToggle("inAppLikes")}
                                />
                            </div>
                        </div>
                    </div>
                </Card>

                {/* Push Notifications */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Smartphone className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Push Notifications</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Enable Push Notifications</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Receive notifications on your device
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.pushEnabled}
                                    onCheckedChange={handleToggle("pushEnabled")}
                                />
                            </div>

                            {settings.pushEnabled && (
                                <>
                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>Messages</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Push notifications for new messages
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushMessages}
                                            onCheckedChange={handleToggle("pushMessages")}
                                        />
                                    </div>

                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>New Followers</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Push notifications for new followers
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushFollowers}
                                            onCheckedChange={handleToggle("pushFollowers")}
                                        />
                                    </div>

                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>Community Activity</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Push notifications for community updates
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushCommunity}
                                            onCheckedChange={handleToggle("pushCommunity")}
                                        />
                                    </div>
                                </>
                            )}

                            {!settings.pushEnabled && (
                                <div className="rounded-lg bg-muted p-4">
                                    <p className="text-sm text-muted-foreground">
                                        Enable push notifications to receive alerts on your device even when you're not using Unity Platform.
                                    </p>
                                </div>
                            )}
                        </div>
                    </div>
                </Card>

                {/* Action Buttons */}
                <div className="flex gap-2">
                    <Button onClick={handleSave}>
                        Save Preferences
                    </Button>
                    <Button variant="outline" onClick={handleCancel}>
                        Cancel
                    </Button>
                </div>
            </div>
        </AppLayout>
    )
}
