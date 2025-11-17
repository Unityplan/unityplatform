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
import { toast } from "sonner"

interface NotificationSettings {
    // Email notifications
    emailDigest: boolean              // Daily activity summary
    emailMessages: boolean            // Direct messages
    emailFollowers: boolean           // New followers
    emailMentions: boolean            // When mentioned
    emailCommunity: boolean           // Community announcements
    emailBadges: boolean              // Badge achievements
    emailCourses: boolean             // Course updates
    emailForums: boolean              // Forum activity
    emailPlatformUpdates: boolean     // Platform news/updates

    // In-app notifications
    inappMessages: boolean            // Direct messages
    inappFollowers: boolean           // New followers
    inappMentions: boolean            // @mentions in posts/comments
    inappCommunity: boolean           // Community activity
    inappBadges: boolean              // Badge achievements
    inappCourses: boolean             // Course updates
    inappForums: boolean              // Forum replies
    inappLikes: boolean               // Likes/reactions on content

    // Push notifications
    pushEnabled: boolean              // Master toggle for push
    pushMessages: boolean             // Direct messages
    pushFollowers: boolean            // New followers
    pushMentions: boolean             // @mentions
    pushCommunity: boolean            // Important community updates
    pushBadges: boolean               // Badge achievements
    pushCourses: boolean              // Course deadlines/updates
}

export function NotificationSettingsPage() {
    const router = useRouter()

    const [settings, setSettings] = useState<NotificationSettings>({
        // Email notifications
        emailDigest: true,
        emailMessages: true,
        emailFollowers: true,
        emailMentions: true,
        emailCommunity: false,
        emailBadges: true,
        emailCourses: true,
        emailForums: false,
        emailPlatformUpdates: true,
        // In-app notifications
        inappMessages: true,
        inappFollowers: true,
        inappMentions: true,
        inappCommunity: true,
        inappBadges: true,
        inappCourses: true,
        inappForums: true,
        inappLikes: false,
        // Push notifications
        pushEnabled: false,
        pushMessages: false,
        pushFollowers: false,
        pushMentions: false,
        pushCommunity: false,
        pushBadges: false,
        pushCourses: false,
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
        toast.success("Notification preferences saved successfully")
    }

    const handleBack = () => {
        router.navigate({ to: "/settings" })
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
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-2xl font-bold">Notification Preferences</h1>
                        <p className="text-muted-foreground mt-1">
                            Choose how and when you want to be notified
                        </p>
                    </div>
                    <div className="flex gap-2">
                        <Button variant="outline" onClick={handleBack}>
                            Back to Settings
                        </Button>
                        <Button onClick={handleSave}>
                            Save
                        </Button>
                    </div>
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
                                    <Label>Mentions</Label>
                                    <p className="text-sm text-muted-foreground">
                                        When someone mentions you in a post or comment
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailMentions}
                                    onCheckedChange={handleToggle("emailMentions")}
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
                                    <Label>Badge Achievements</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Get notified when you earn a new badge
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailBadges}
                                    onCheckedChange={handleToggle("emailBadges")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Course Updates</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Updates about courses you're enrolled in
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailCourses}
                                    onCheckedChange={handleToggle("emailCourses")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Forum Activity</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Replies to your forum posts and topics
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.emailForums}
                                    onCheckedChange={handleToggle("emailForums")}
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
                                    checked={settings.emailPlatformUpdates}
                                    onCheckedChange={handleToggle("emailPlatformUpdates")}
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
                                    checked={settings.inappMessages}
                                    onCheckedChange={handleToggle("inappMessages")}
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
                                    checked={settings.inappFollowers}
                                    onCheckedChange={handleToggle("inappFollowers")}
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
                                    checked={settings.inappMentions}
                                    onCheckedChange={handleToggle("inappMentions")}
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
                                    checked={settings.inappCommunity}
                                    onCheckedChange={handleToggle("inappCommunity")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Badge Achievements</Label>
                                    <p className="text-sm text-muted-foreground">
                                        When you earn a new badge
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inappBadges}
                                    onCheckedChange={handleToggle("inappBadges")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Course Updates</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Updates about your enrolled courses
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inappCourses}
                                    onCheckedChange={handleToggle("inappCourses")}
                                />
                            </div>

                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Forum Replies</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Replies to your forum posts and topics
                                    </p>
                                </div>
                                <Switch
                                    checked={settings.inappForums}
                                    onCheckedChange={handleToggle("inappForums")}
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
                                    checked={settings.inappLikes}
                                    onCheckedChange={handleToggle("inappLikes")}
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
                                            <Label>Mentions</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Push notifications when someone mentions you
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushMentions}
                                            onCheckedChange={handleToggle("pushMentions")}
                                        />
                                    </div>

                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>Community Activity</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Important updates from your communities
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushCommunity}
                                            onCheckedChange={handleToggle("pushCommunity")}
                                        />
                                    </div>

                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>Badge Achievements</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Push notifications when you earn a badge
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushBadges}
                                            onCheckedChange={handleToggle("pushBadges")}
                                        />
                                    </div>

                                    <div className="flex items-center justify-between">
                                        <div className="space-y-0.5">
                                            <Label>Course Updates</Label>
                                            <p className="text-sm text-muted-foreground">
                                                Important course deadlines and updates
                                            </p>
                                        </div>
                                        <Switch
                                            checked={settings.pushCourses}
                                            onCheckedChange={handleToggle("pushCourses")}
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

                {/* Action Buttons - Bottom */}
                <div className="flex justify-end gap-2">
                    <Button variant="outline" onClick={handleBack}>
                        Back to Settings
                    </Button>
                    <Button onClick={handleSave}>
                        Save
                    </Button>
                </div>
            </div>
        </AppLayout>
    )
}
