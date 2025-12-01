import { AppLayout } from '@/components/layouts/AppLayout'
import { PageContainer } from '@/components/layouts/PageContainer'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbSeparator,
    BreadcrumbPage,
} from '@/components/ui/breadcrumb'
import { Link } from '@tanstack/react-router'
import { Home } from 'lucide-react'
import { Button } from '@/components/ui/button'

/**
 * Template page component showing the standard structure for new pages.
 * Copy this file when creating a new page to ensure consistency.
 */
export function TemplatePage() {
    // 1. Define Breadcrumbs
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
                        <Link to="/parent-route">Parent</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>Current Page</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    )

    return (
        <AppLayout breadcrumbs={breadcrumbs}>
            <PageContainer>
                {/* 2. Page Header */}
                <div className="mb-8 flex items-center justify-between">
                    <div>
                        <h1 className="text-3xl font-bold tracking-tight">Page Title</h1>
                        <p className="text-muted-foreground">
                            A descriptive subtitle for this page.
                        </p>
                    </div>
                    <div className="flex gap-2">
                        <Button variant="outline">Secondary Action</Button>
                        <Button>Primary Action</Button>
                    </div>
                </div>

                {/* 3. Page Content */}
                <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                    {/* Content goes here */}
                    <div className="rounded-lg border bg-card p-6 shadow-sm">
                        <h2 className="text-lg font-semibold">Section 1</h2>
                        <p className="mt-2 text-sm text-muted-foreground">Content...</p>
                    </div>
                </div>
            </PageContainer>
        </AppLayout>
    )
}
