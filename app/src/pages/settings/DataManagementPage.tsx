import { useRouter, Link } from "@tanstack/react-router"
import { Home, Settings as SettingsIcon, Download, Trash2, FileText, AlertTriangle } from "lucide-react"
import { AppLayout } from "@/components/layouts/AppLayout"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { useState } from "react"
import { toast } from "sonner"

export function DataManagementPage() {
    const router = useRouter()
    const [isExporting, setIsExporting] = useState(false)
    const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)
    const [deleteConfirmText, setDeleteConfirmText] = useState("")

    const handleExportData = () => {
        setIsExporting(true)

        // Mock: Simulate export process
        setTimeout(() => {
            setIsExporting(false)
            toast.success("Your data export has been prepared. Download link sent to your email.")
        }, 2000)
    }

    const handleDeleteAccount = () => {
        if (deleteConfirmText !== "DELETE") {
            toast.error("Please type DELETE to confirm")
            return
        }

        // Mock: Simulate account deletion
        if (confirm("This action cannot be undone. Are you absolutely sure?")) {
            toast.success("Account deletion request submitted. You will receive a confirmation email.")
            router.navigate({ to: "/login" })
        }
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
                            <BreadcrumbPage>Data Management</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            }
        >
            <div className="space-y-6 py-6">
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-2xl font-bold">Data Management</h1>
                        <p className="text-muted-foreground mt-1">
                            Export your data or delete your account
                        </p>
                    </div>
                    <Button variant="outline" onClick={handleBack}>
                        Back to Settings
                    </Button>
                </div>

                {/* GDPR Data Export */}
                <Card className="p-6">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Download className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-semibold">Export Your Data</h2>
                        </div>

                        <div className="space-y-4">
                            <p className="text-sm text-muted-foreground">
                                Download a copy of all your personal data stored in Unity Platform. This includes your profile information, posts, messages, and activity history.
                            </p>

                            <div className="rounded-lg border p-4 space-y-3">
                                <div className="flex items-start gap-3">
                                    <FileText className="size-5 text-muted-foreground mt-0.5" />
                                    <div className="flex-1">
                                        <p className="text-sm font-medium">What's included in the export:</p>
                                        <ul className="text-sm text-muted-foreground mt-2 space-y-1 list-disc list-inside">
                                            <li>Profile information and settings</li>
                                            <li>Posts and comments</li>
                                            <li>Direct messages</li>
                                            <li>Community memberships</li>
                                            <li>Connections (followers/following)</li>
                                            <li>Activity history</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>

                            <div className="rounded-lg bg-blue-50 dark:bg-blue-950/30 border border-blue-200 dark:border-blue-800 p-4">
                                <p className="text-sm text-blue-900 dark:text-blue-200">
                                    <strong>Note:</strong> The export will be prepared in the background and sent to your email address when ready. This may take up to 24 hours depending on the amount of data.
                                </p>
                            </div>

                            <Button onClick={handleExportData} disabled={isExporting}>
                                {isExporting ? (
                                    <>
                                        <span className="mr-2">Preparing Export...</span>
                                        <span className="inline-block animate-spin">⏳</span>
                                    </>
                                ) : (
                                    <>
                                        <Download className="size-4 mr-2" />
                                        Request Data Export
                                    </>
                                )}
                            </Button>
                        </div>
                    </div>
                </Card>

                {/* Account Deletion */}
                <Card className="p-6 border-red-200 dark:border-red-900">
                    <div className="space-y-4">
                        <div className="flex items-center gap-2">
                            <Trash2 className="size-5 text-red-600 dark:text-red-500" />
                            <h2 className="text-lg font-semibold text-red-600 dark:text-red-500">Delete Account</h2>
                        </div>

                        <div className="space-y-4">
                            <div className="rounded-lg bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-800 p-4">
                                <div className="flex items-start gap-3">
                                    <AlertTriangle className="size-5 text-red-600 dark:text-red-500 mt-0.5 shrink-0" />
                                    <div>
                                        <p className="text-sm font-medium text-red-900 dark:text-red-200">
                                            Warning: This action cannot be undone
                                        </p>
                                        <p className="text-sm text-red-700 dark:text-red-300 mt-2">
                                            Deleting your account will permanently remove:
                                        </p>
                                        <ul className="text-sm text-red-700 dark:text-red-300 mt-2 space-y-1 list-disc list-inside ml-4">
                                            <li>Your profile and all personal information</li>
                                            <li>All posts, comments, and messages</li>
                                            <li>Community memberships and roles</li>
                                            <li>Connections and followers</li>
                                            <li>All activity history</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>

                            {!showDeleteConfirm ? (
                                <Button
                                    variant="destructive"
                                    onClick={() => setShowDeleteConfirm(true)}
                                >
                                    <Trash2 className="size-4 mr-2" />
                                    Delete My Account
                                </Button>
                            ) : (
                                <div className="space-y-4 rounded-lg border border-red-200 dark:border-red-800 p-4">
                                    <p className="text-sm font-medium">
                                        Please type <code className="bg-muted px-1 py-0.5 rounded">DELETE</code> to confirm:
                                    </p>
                                    <input
                                        type="text"
                                        value={deleteConfirmText}
                                        onChange={(e) => setDeleteConfirmText(e.target.value)}
                                        className="w-full max-w-md px-3 py-2 border rounded-md bg-background"
                                        placeholder="Type DELETE"
                                    />
                                    <div className="flex gap-2">
                                        <Button
                                            variant="destructive"
                                            onClick={handleDeleteAccount}
                                            disabled={deleteConfirmText !== "DELETE"}
                                        >
                                            Confirm Deletion
                                        </Button>
                                        <Button
                                            variant="outline"
                                            onClick={() => {
                                                setShowDeleteConfirm(false)
                                                setDeleteConfirmText("")
                                            }}
                                        >
                                            Cancel
                                        </Button>
                                    </div>
                                </div>
                            )}

                            <div className="rounded-lg bg-muted p-4">
                                <p className="text-sm text-muted-foreground">
                                    <strong>Alternative:</strong> If you just need a break, consider deactivating your account temporarily instead. Contact support for assistance.
                                </p>
                            </div>
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
