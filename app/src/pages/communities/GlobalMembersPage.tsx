import { AppLayout } from '@/components/layouts/AppLayout'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbSeparator,
    BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { Link } from '@tanstack/react-router'
import { Home, Users } from 'lucide-react'

export function GlobalMembersPage() {
    const breadcrumbs = (
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
                    <BreadcrumbLink asChild>
                        <Link to="/communities">Communities</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>Members</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <div className="w-full px-4 py-8 md:px-8">
                <div className="mb-8">
                    <h1 className="text-3xl font-bold tracking-tight">Community Members</h1>
                    <p className="text-muted-foreground">
                        Browse and search for members across all communities.
                    </p>
                </div>

                <div className="flex h-[400px] flex-col items-center justify-center rounded-lg border border-dashed">
                    <Users className="mb-4 h-12 w-12 text-muted-foreground" />
                    <h3 className="text-lg font-semibold">Member Directory Coming Soon</h3>
                    <p className="text-muted-foreground">
                        This feature will allow you to find and connect with other members.
                    </p>
                </div>
            </div>
        </AppLayout>
    )
}
