import { useRouter, Link } from "@tanstack/react-router"
import { Home, Settings as SettingsIcon, Mail, Lock, Shield, Eye, EyeOff } from "lucide-react"
import { AppLayout } from "@/components/layouts/AppLayout"
import { Card } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Switch } from "@/components/ui/switch"
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

export function AccountSettingsPage() {
    const router = useRouter()

    // Email settings
    const [email, setEmail] = useState("")
    const [newEmail, setNewEmail] = useState("")
    const [emailVerified, setEmailVerified] = useState(true)

    // Password settings
    const [currentPassword, setCurrentPassword] = useState("")
    const [newPassword, setNewPassword] = useState("")
    const [confirmPassword, setConfirmPassword] = useState("")
    const [showCurrentPassword, setShowCurrentPassword] = useState(false)
    const [showNewPassword, setShowNewPassword] = useState(false)
    const [showConfirmPassword, setShowConfirmPassword] = useState(false)

    // TOTP settings
    const [totpEnabled, setTotpEnabled] = useState(false)
    const [showTotpSetup, setShowTotpSetup] = useState(false)

    // Load saved settings
    useEffect(() => {
        const savedEmail = localStorage.getItem("userEmail") || "user@example.com"
        const savedTotpEnabled = localStorage.getItem("totpEnabled") === "true"
        setEmail(savedEmail)
        setTotpEnabled(savedTotpEnabled)
    }, [])

    const handleEmailChange = () => {
        if (!newEmail) {
            toast.error("Please enter a new email address")
            return
        }

        // Mock: Save to localStorage
        localStorage.setItem("userEmail", newEmail)
        setEmail(newEmail)
        setNewEmail("")
        setEmailVerified(false)
        toast.success("Verification email sent to " + newEmail)
    }

    const handlePasswordChange = () => {
        if (!currentPassword || !newPassword || !confirmPassword) {
            toast.error("Please fill in all password fields")
            return
        }

        if (newPassword !== confirmPassword) {
            toast.error("New passwords do not match")
            return
        }

        if (newPassword.length < 8) {
            toast.error("Password must be at least 8 characters")
            return
        }

        // Mock: Clear fields
        setCurrentPassword("")
        setNewPassword("")
        setConfirmPassword("")
        toast.success("Password changed successfully")
    }

    const handleTotpToggle = (checked: boolean) => {
        if (checked) {
            setShowTotpSetup(true)
        } else {
            if (confirm("Are you sure you want to disable two-factor authentication?")) {
                setTotpEnabled(false)
                localStorage.setItem("totpEnabled", "false")
                toast.success("Two-factor authentication disabled")
            }
        }
    }

    const handleTotpSetup = () => {
        // Mock: Enable TOTP
        setTotpEnabled(true)
        setShowTotpSetup(false)
        localStorage.setItem("totpEnabled", "true")
        toast.success("Two-factor authentication enabled successfully")
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
                            <BreadcrumbPage>Account</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-2xl font-bold">Account Settings</h1>
                        <p className="text-muted-foreground mt-1">
                            Manage your email, password, and security settings
                        </p>
                    </div>
                    <Button variant="outline" onClick={handleBack}>
                        Back to Settings
                    </Button>
                </div>

                {/* Email Management */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Mail className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Email Address</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="space-y-2">
                                <Label htmlFor="current-email">Current Email</Label>
                                <div className="flex items-center gap-2">
                                    <Input
                                        id="current-email"
                                        type="email"
                                        value={email}
                                        disabled
                                        className="max-w-md"
                                    />
                                    {emailVerified ? (
                                        <span className="text-sm text-green-600 dark:text-green-500 flex items-center gap-1">
                                            <Shield className="size-4" />
                                            Verified
                                        </span>
                                    ) : (
                                        <span className="text-sm text-yellow-600 dark:text-yellow-500">
                                            Unverified
                                        </span>
                                    )}
                                </div>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="new-email">New Email Address</Label>
                                <div className="flex items-center gap-2">
                                    <Input
                                        id="new-email"
                                        type="email"
                                        placeholder="Enter new email"
                                        value={newEmail}
                                        onChange={(e) => setNewEmail(e.target.value)}
                                        className="max-w-md"
                                    />
                                    <Button onClick={handleEmailChange}>
                                        Change Email
                                    </Button>
                                </div>
                                <p className="text-sm text-muted-foreground">
                                    A verification email will be sent to your new address
                                </p>
                            </div>
                        </div>
                    </div>
                </Card>

                {/* Password Change */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Lock className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Change Password</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="space-y-2">
                                <Label htmlFor="current-password">Current Password</Label>
                                <div className="relative max-w-md">
                                    <Input
                                        id="current-password"
                                        type={showCurrentPassword ? "text" : "password"}
                                        placeholder="Enter current password"
                                        value={currentPassword}
                                        onChange={(e) => setCurrentPassword(e.target.value)}
                                    />
                                    <button
                                        type="button"
                                        onClick={() => setShowCurrentPassword(!showCurrentPassword)}
                                        className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                    >
                                        {showCurrentPassword ? (
                                            <EyeOff className="size-4" />
                                        ) : (
                                            <Eye className="size-4" />
                                        )}
                                    </button>
                                </div>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="new-password">New Password</Label>
                                <div className="relative max-w-md">
                                    <Input
                                        id="new-password"
                                        type={showNewPassword ? "text" : "password"}
                                        placeholder="Enter new password"
                                        value={newPassword}
                                        onChange={(e) => setNewPassword(e.target.value)}
                                    />
                                    <button
                                        type="button"
                                        onClick={() => setShowNewPassword(!showNewPassword)}
                                        className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                    >
                                        {showNewPassword ? (
                                            <EyeOff className="size-4" />
                                        ) : (
                                            <Eye className="size-4" />
                                        )}
                                    </button>
                                </div>
                                <p className="text-sm text-muted-foreground">
                                    Minimum 8 characters
                                </p>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="confirm-password">Confirm New Password</Label>
                                <div className="relative max-w-md">
                                    <Input
                                        id="confirm-password"
                                        type={showConfirmPassword ? "text" : "password"}
                                        placeholder="Confirm new password"
                                        value={confirmPassword}
                                        onChange={(e) => setConfirmPassword(e.target.value)}
                                    />
                                    <button
                                        type="button"
                                        onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                                        className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                                    >
                                        {showConfirmPassword ? (
                                            <EyeOff className="size-4" />
                                        ) : (
                                            <Eye className="size-4" />
                                        )}
                                    </button>
                                </div>
                            </div>

                            <Button onClick={handlePasswordChange}>
                                Update Password
                            </Button>
                        </div>
                    </div>
                </Card>

                {/* Two-Factor Authentication */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Shield className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Two-Factor Authentication (TOTP)</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between">
                                <div className="space-y-0.5">
                                    <Label>Enable Two-Factor Authentication</Label>
                                    <p className="text-sm text-muted-foreground">
                                        Add an extra layer of security to your account
                                    </p>
                                </div>
                                <Switch
                                    checked={totpEnabled}
                                    onCheckedChange={handleTotpToggle}
                                />
                            </div>

                            {totpEnabled && (
                                <div className="rounded-lg bg-muted p-4">
                                    <p className="text-sm text-muted-foreground">
                                        Two-factor authentication is currently enabled. You'll be asked for a code from your authenticator app when signing in.
                                    </p>
                                </div>
                            )}

                            {showTotpSetup && !totpEnabled && (
                                <div className="rounded-lg border p-4 space-y-4">
                                    <p className="text-sm font-medium">Set up Two-Factor Authentication</p>
                                    <div className="space-y-2">
                                        <p className="text-sm text-muted-foreground">
                                            1. Install an authenticator app (Google Authenticator, Authy, etc.)
                                        </p>
                                        <p className="text-sm text-muted-foreground">
                                            2. Scan the QR code below with your authenticator app
                                        </p>
                                        <div className="flex justify-center p-6 bg-muted rounded-lg">
                                            <div className="size-32 bg-background border-2 border-dashed border-muted-foreground/50 rounded flex items-center justify-center">
                                                <p className="text-sm text-muted-foreground text-center">
                                                    QR Code<br />Placeholder
                                                </p>
                                            </div>
                                        </div>
                                        <p className="text-sm text-muted-foreground">
                                            3. Enter the 6-digit code from your app
                                        </p>
                                        <Input
                                            type="text"
                                            placeholder="000000"
                                            maxLength={6}
                                            className="max-w-[150px] text-center text-lg tracking-widest"
                                        />
                                    </div>
                                    <div className="flex gap-2">
                                        <Button onClick={handleTotpSetup}>
                                            Enable 2FA
                                        </Button>
                                        <Button variant="outline" onClick={() => setShowTotpSetup(false)}>
                                            Cancel
                                        </Button>
                                    </div>
                                </div>
                            )}
                        </div>
                    </div>
                </Card>

                {/* Action Buttons - Bottom */}
                <div className="flex justify-end">
                    <Button variant="outline" onClick={handleBack}>
                        Back to Settings
                    </Button>
                </div>
            </div>
        </AppLayout>
    )
}
