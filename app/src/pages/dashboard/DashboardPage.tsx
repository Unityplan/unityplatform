import { CreateInvitationDialog } from '@/components/invitations/CreateInvitationDialog';
import { InvitationList } from '@/components/invitations/InvitationList';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { useAuthStore } from '@/stores/authStore';

export function DashboardPage() {
    const { user } = useAuthStore();

    return (
        <div className="container mx-auto py-10 space-y-8">
            <div className="flex justify-between items-center">
                <div>
                    <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
                    <p className="text-muted-foreground">
                        Welcome back, {user?.username || 'User'}.
                    </p>
                </div>
            </div>

            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">
                            Total Invitations
                        </CardTitle>
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">--</div>
                        <p className="text-xs text-muted-foreground">
                            +0 from last month
                        </p>
                    </CardContent>
                </Card>
                {/* Add more stats cards here */}
            </div>

            <div className="grid gap-4 md:grid-cols-1">
                <Card className="col-span-1">
                    <CardHeader className="flex flex-row items-center justify-between">
                        <div>
                            <CardTitle>Invitation Management</CardTitle>
                            <CardDescription>
                                Manage invitations for your territory.
                            </CardDescription>
                        </div>
                        <CreateInvitationDialog />
                    </CardHeader>
                    <CardContent>
                        <InvitationList />
                    </CardContent>
                </Card>
            </div>
        </div>
    );
}
