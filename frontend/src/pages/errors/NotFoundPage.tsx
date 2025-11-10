import { Link } from "@tanstack/react-router"
import { Home, Search, ArrowLeft, FileQuestion } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"

/**
 * 404 Not Found Page
 * 
 * Displayed when a user navigates to a route that doesn't exist.
 * Provides helpful navigation options to get back on track.
 */
export function NotFoundPage() {
    return (
        <div className="min-h-screen flex items-center justify-center p-4 bg-linear-to-b from-background to-muted/20">
            <Card className="max-w-2xl w-full p-8 md:p-12">
                <div className="flex flex-col items-center text-center space-y-6">
                    {/* Icon */}
                    <div className="relative">
                        <div className="absolute inset-0 bg-primary/10 rounded-full blur-2xl" />
                        <div className="relative bg-primary/10 p-6 rounded-full">
                            <FileQuestion className="size-16 text-primary" />
                        </div>
                    </div>

                    {/* Error Code */}
                    <div>
                        <h1 className="text-6xl md:text-7xl font-bold text-primary mb-2">404</h1>
                        <h2 className="text-2xl md:text-3xl font-semibold">Page Not Found</h2>
                    </div>

                    {/* Description */}
                    <p className="text-muted-foreground text-lg max-w-md">
                        Sorry, we couldn't find the page you're looking for. It might have been moved, deleted, or never existed.
                    </p>

                    {/* Action Buttons */}
                    <div className="flex flex-col sm:flex-row gap-3 pt-4 w-full sm:w-auto">
                        <Button asChild size="lg" className="gap-2">
                            <Link to="/">
                                <Home className="size-4" />
                                Go to Dashboard
                            </Link>
                        </Button>
                        <Button asChild variant="outline" size="lg" className="gap-2">
                            <Link to="/" onClick={() => window.history.back()}>
                                <ArrowLeft className="size-4" />
                                Go Back
                            </Link>
                        </Button>
                    </div>

                    {/* Help Section */}
                    <div className="pt-8 border-t w-full">
                        <p className="text-sm text-muted-foreground mb-4">
                            Looking for something specific?
                        </p>
                        <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 text-sm">
                            <Link
                                to="/profile"
                                className="flex items-center gap-2 p-3 rounded-lg hover:bg-muted transition-colors"
                            >
                                <Home className="size-4 text-muted-foreground" />
                                <span>Your Profile</span>
                            </Link>
                            <Link
                                to="/settings"
                                className="flex items-center gap-2 p-3 rounded-lg hover:bg-muted transition-colors"
                            >
                                <Home className="size-4 text-muted-foreground" />
                                <span>Settings</span>
                            </Link>
                            <button
                                onClick={() => {
                                    // TODO: Implement search functionality
                                    alert("Search functionality coming soon!")
                                }}
                                className="flex items-center gap-2 p-3 rounded-lg hover:bg-muted transition-colors"
                            >
                                <Search className="size-4 text-muted-foreground" />
                                <span>Search</span>
                            </button>
                        </div>
                    </div>
                </div>
            </Card>
        </div>
    )
}
