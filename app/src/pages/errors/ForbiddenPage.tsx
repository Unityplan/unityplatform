import { Link, useRouter } from "@tanstack/react-router"
import { Home, ArrowLeft, ShieldAlert, Mail } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"

/**
 * 403 Forbidden Page
 * 
 * Displayed when a user tries to access a resource they don't have permission for.
 * Provides context about why access was denied and how to resolve it.
 */
export function ForbiddenPage() {
    const router = useRouter()

    const handleGoBack = () => {
        // Go back in history, or to dashboard if no history
        if (window.history.length > 1) {
            router.history.back()
        } else {
            router.navigate({ to: "/" })
        }
    }

    return (
        <div className="min-h-screen flex items-center justify-center p-4 bg-linear-to-b from-background to-muted/20">
            <Card className="max-w-2xl w-full p-8 md:p-12">
                <div className="flex flex-col items-center text-center space-y-6">
                    {/* Icon */}
                    <div className="relative">
                        <div className="absolute inset-0 bg-red-500/10 rounded-full blur-2xl" />
                        <div className="relative bg-red-500/10 p-6 rounded-full">
                            <ShieldAlert className="size-16 text-red-600 dark:text-red-500" />
                        </div>
                    </div>

                    {/* Error Code */}
                    <div>
                        <h1 className="text-6xl md:text-7xl font-bold text-red-600 dark:text-red-500 mb-2">403</h1>
                        <h2 className="text-2xl md:text-3xl font-semibold">Access Denied</h2>
                    </div>

                    {/* Description */}
                    <div className="space-y-2">
                        <p className="text-muted-foreground text-lg max-w-md">
                            You don't have permission to access this resource.
                        </p>
                        <p className="text-sm text-muted-foreground max-w-md">
                            This could be because you're not logged in, your account doesn't have the required permissions, or the content is restricted.
                        </p>
                    </div>

                    {/* Action Buttons */}
                    <div className="flex flex-col sm:flex-row gap-3 pt-4 w-full sm:w-auto">
                        <Button asChild size="lg" className="gap-2">
                            <Link to="/">
                                <Home className="size-4" />
                                Go to Dashboard
                            </Link>
                        </Button>
                        <Button variant="outline" size="lg" className="gap-2" onClick={handleGoBack}>
                            <ArrowLeft className="size-4" />
                            Go Back
                        </Button>
                    </div>

                    {/* Help Section */}
                    <div className="pt-8 border-t w-full space-y-4">
                        <p className="text-sm font-medium">Why might this happen?</p>
                        <div className="grid gap-3 text-sm text-left">
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Not logged in</p>
                                    <p className="text-muted-foreground">
                                        This content requires authentication. Try logging in first.
                                    </p>
                                </div>
                            </div>
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Insufficient permissions</p>
                                    <p className="text-muted-foreground">
                                        Your account level doesn't allow access to this content.
                                    </p>
                                </div>
                            </div>
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Private content</p>
                                    <p className="text-muted-foreground">
                                        This is private content that you're not authorized to view.
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div className="pt-4">
                            <p className="text-sm text-muted-foreground mb-3">
                                Need help? Contact your community manager or territory administrator.
                            </p>
                            <Button variant="outline" size="sm" className="gap-2">
                                <Mail className="size-4" />
                                Contact Support
                            </Button>
                        </div>
                    </div>
                </div>
            </Card>
        </div>
    )
}
