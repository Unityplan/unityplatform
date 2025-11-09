import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { useAuthStore } from '@/stores/authStore';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

export const Route = createFileRoute('/dashboard')({
    component: Dashboard,
});

function Dashboard() {
    const { user } = useAuthStore();

    return (
        <AuthGuard>
            <div className="min-h-screen bg-background p-4">
                <div className="mx-auto max-w-6xl space-y-6">
                    {/* Header */}
                    <div className="flex items-center justify-between">
                        <div>
                            <h1 className="text-3xl font-bold">Dashboard</h1>
                            <p className="text-muted-foreground">Welcome back, {user?.username}!</p>
                        </div>
                        <Button onClick={() => window.location.href = '/profile'}>
                            View Profile
                        </Button>
                    </div>

                    {/* Quick Stats */}
                    <div className="grid gap-4 md:grid-cols-3">
                        <Card>
                            <CardHeader>
                                <CardTitle>Territory</CardTitle>
                                <CardDescription>Your current territory</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p className="text-2xl font-bold uppercase">{user?.territory_code}</p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>Account Status</CardTitle>
                                <CardDescription>Your account details</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p className="text-2xl font-bold">Active</p>
                            </CardContent>
                        </Card>

                        <Card>
                            <CardHeader>
                                <CardTitle>Profile</CardTitle>
                                <CardDescription>Manage your profile</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <Button variant="outline" onClick={() => window.location.href = '/profile/edit'}>
                                    Edit Profile
                                </Button>
                            </CardContent>
                        </Card>
                    </div>

                    {/* Quick Actions */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Quick Actions</CardTitle>
                            <CardDescription>Common tasks and navigation</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-2">
                            <Button
                                variant="outline"
                                className="w-full justify-start"
                                onClick={() => window.location.href = '/profile'}
                            >
                                View Profile
                            </Button>
                            <Button
                                variant="outline"
                                className="w-full justify-start"
                                onClick={() => window.location.href = '/profile/edit'}
                            >
                                Edit Profile
                            </Button>
                            <Button
                                variant="outline"
                                className="w-full justify-start"
                                onClick={() => {
                                    useAuthStore.getState().logout();
                                    window.location.href = '/login';
                                }}
                            >
                                Sign Out
                            </Button>
                        </CardContent>
                    </Card>

                    {/* Welcome Message */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Welcome to UnityPlan</CardTitle>
                            <CardDescription>Getting started with the platform</CardDescription>
                        </CardHeader>
                        <CardContent className="space-y-4">
                            <p className="text-foreground">
                                This is your personal dashboard. From here you can:
                            </p>
                            <ul className="list-disc list-inside space-y-2 text-muted-foreground">
                                <li>Manage your profile and privacy settings</li>
                                <li>View and edit your personal information</li>
                                <li>Access courses and learning materials (coming soon)</li>
                                <li>Participate in community forums (coming soon)</li>
                                <li>Earn and display badges (coming soon)</li>
                            </ul>
                            <p className="text-sm text-muted-foreground italic">
                                More features will be available as the platform continues to develop.
                            </p>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </AuthGuard>
    );
}
