import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { useAuthStore } from '@/stores/authStore';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { AppLayout } from '@/components/layouts/AppLayout';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link, useRouter } from '@tanstack/react-router';
import { Home } from 'lucide-react';

export const Route = createFileRoute('/dashboard')({
    component: Dashboard,
});

function Dashboard() {
    const { user } = useAuthStore();
    const router = useRouter();

    const handleLogout = () => {
        useAuthStore.getState().logout();
        router.navigate({ to: '/login' });
    };

    return (
        <AuthGuard>
            <AppLayout
                breadcrumbs={
                    <Breadcrumb>
                        <BreadcrumbList>
                            <BreadcrumbItem>
                                <BreadcrumbLink asChild>
                                    <Link to="/">
                                        <Home className="size-4" />
                                    </Link>
                                </BreadcrumbLink>
                            </BreadcrumbItem>
                            <BreadcrumbSeparator />
                            <BreadcrumbItem>
                                <BreadcrumbPage>Dashboard</BreadcrumbPage>
                            </BreadcrumbItem>
                        </BreadcrumbList>
                    </Breadcrumb>
                }
            >
                <div className="space-y-6 py-6">
                    {/* Welcome Header */}
                    <div>
                        <h1 className="text-3xl font-bold">Dashboard</h1>
                        <p className="text-muted-foreground">Welcome back, {user?.username}!</p>
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
                                <Button onClick={() => window.location.href = '/profile/edit'}>
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
                                className="w-full justify-start"
                                onClick={() => window.location.href = '/profile'}
                            >
                                View Profile
                            </Button>
                            <Button
                                className="w-full justify-start"
                                onClick={() => window.location.href = '/profile/edit'}
                            >
                                Edit Profile
                            </Button>
                            <Button
                                className="w-full justify-start"
                                onClick={handleLogout}
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
            </AppLayout>
        </AuthGuard>
    );
}
