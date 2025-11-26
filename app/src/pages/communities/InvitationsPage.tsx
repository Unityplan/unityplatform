import { CreateInvitationDialog } from '@/components/invitations/CreateInvitationDialog';
import { InvitationList } from '@/components/invitations/InvitationList';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb';
import { Link } from '@tanstack/react-router';
import { AppLayout } from '@/components/layouts/AppLayout';

export function InvitationsPage() {
    const breadcrumbs = (
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/communities">Communities</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>Invitations</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    );

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="w-full px-4 py-10 space-y-8 md:px-8">
                <div className="flex items-center justify-between space-y-2">
                    <div>
                        <h1 className="text-3xl font-bold tracking-tight">Invitations</h1>
                        <p className="text-muted-foreground">
                            Manage invitations for your territory.
                        </p>
                    </div>
                    <CreateInvitationDialog />
                </div>

                <Card>
                    <CardHeader>
                        <CardTitle>Your Invitations</CardTitle>
                        <CardDescription>
                            List of invitations you have created.
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <InvitationList />
                    </CardContent>
                </Card>
            </div>
        </AppLayout>
    );
}
