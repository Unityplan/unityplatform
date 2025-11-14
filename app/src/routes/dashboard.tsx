import { createFileRoute } from '@tanstack/react-router';
import { AuthGuard } from '@/components/AuthGuard';
import { useAuthStore } from '@/stores/authStore';
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
import { Link } from '@tanstack/react-router';
import { Home } from 'lucide-react';

export const Route = createFileRoute('/dashboard')({
    component: Dashboard,
});

function Dashboard() {
    const { user } = useAuthStore();

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

                    {/* Welcome Message */}
                    <Card>
                        <CardHeader>
                            <CardTitle>Welcome to Unity Platform</CardTitle>
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
