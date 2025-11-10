import { Link } from "@tanstack/react-router"
import { Home, RefreshCw, AlertTriangle, Mail } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { useState } from "react"

/**
 * 500 Server Error Page
 * 
 * Displayed when something goes wrong on the server or an unexpected error occurs.
 * Provides options to retry, report the issue, or navigate away.
 */
export function ServerErrorPage() {
    const [isRefreshing, setIsRefreshing] = useState(false)

    const handleRefresh = () => {
        setIsRefreshing(true)
        window.location.reload()
    }

    const handleReportIssue = () => {
        // TODO: Implement error reporting
        alert("Error reporting coming soon! For now, please contact support directly.")
    }

    return (
        <div className="min-h-screen flex items-center justify-center p-4 bg-linear-to-b from-background to-muted/20">
            <Card className="max-w-2xl w-full p-8 md:p-12">
                <div className="flex flex-col items-center text-center space-y-6">
                    {/* Icon */}
                    <div className="relative">
                        <div className="absolute inset-0 bg-yellow-500/10 rounded-full blur-2xl" />
                        <div className="relative bg-yellow-500/10 p-6 rounded-full">
                            <AlertTriangle className="size-16 text-yellow-600 dark:text-yellow-500" />
                        </div>
                    </div>

                    {/* Error Code */}
                    <div>
                        <h1 className="text-6xl md:text-7xl font-bold text-yellow-600 dark:text-yellow-500 mb-2">500</h1>
                        <h2 className="text-2xl md:text-3xl font-semibold">Something Went Wrong</h2>
                    </div>

                    {/* Description */}
                    <div className="space-y-2">
                        <p className="text-muted-foreground text-lg max-w-md">
                            We encountered an unexpected error while processing your request.
                        </p>
                        <p className="text-sm text-muted-foreground max-w-md">
                            Our team has been notified and is working to fix the issue. Please try again in a few moments.
                        </p>
                    </div>

                    {/* Action Buttons */}
                    <div className="flex flex-col sm:flex-row gap-3 pt-4 w-full sm:w-auto">
                        <Button
                            size="lg"
                            className="gap-2"
                            onClick={handleRefresh}
                            disabled={isRefreshing}
                        >
                            <RefreshCw className={`size-4 ${isRefreshing ? 'animate-spin' : ''}`} />
                            {isRefreshing ? 'Refreshing...' : 'Refresh Page'}
                        </Button>
                        <Button asChild variant="outline" size="lg" className="gap-2">
                            <Link to="/">
                                <Home className="size-4" />
                                Go to Dashboard
                            </Link>
                        </Button>
                    </div>

                    {/* Help Section */}
                    <div className="pt-8 border-t w-full space-y-4">
                        <p className="text-sm font-medium">What you can do:</p>
                        <div className="grid gap-3 text-sm text-left">
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Wait a moment and try again</p>
                                    <p className="text-muted-foreground">
                                        This might be a temporary issue that resolves itself.
                                    </p>
                                </div>
                            </div>
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Clear your browser cache</p>
                                    <p className="text-muted-foreground">
                                        Sometimes cached data can cause conflicts.
                                    </p>
                                </div>
                            </div>
                            <div className="flex items-start gap-3 p-3 rounded-lg bg-muted/50">
                                <div className="size-2 rounded-full bg-primary mt-1.5 shrink-0" />
                                <div>
                                    <p className="font-medium">Check your internet connection</p>
                                    <p className="text-muted-foreground">
                                        Make sure you have a stable connection to the internet.
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div className="pt-4">
                            <p className="text-sm text-muted-foreground mb-3">
                                If the problem persists, please report it to our support team.
                            </p>
                            <div className="flex flex-col sm:flex-row gap-2 justify-center">
                                <Button
                                    variant="outline"
                                    size="sm"
                                    className="gap-2"
                                    onClick={handleReportIssue}
                                >
                                    <AlertTriangle className="size-4" />
                                    Report Issue
                                </Button>
                                <Button variant="outline" size="sm" className="gap-2">
                                    <Mail className="size-4" />
                                    Contact Support
                                </Button>
                            </div>
                        </div>

                        {/* Technical Details (Optional - can be hidden by default) */}
                        <details className="pt-4">
                            <summary className="text-xs text-muted-foreground cursor-pointer hover:text-foreground">
                                Technical details
                            </summary>
                            <div className="mt-3 p-3 bg-muted rounded-lg text-left">
                                <p className="text-xs font-mono text-muted-foreground">
                                    Error ID: {Math.random().toString(36).substring(7).toUpperCase()}<br />
                                    Time: {new Date().toISOString()}<br />
                                    Browser: {navigator.userAgent}
                                </p>
                            </div>
                        </details>
                    </div>
                </div>
            </Card>
        </div>
    )
}
