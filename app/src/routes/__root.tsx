import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { NotFoundPage } from '@/pages/errors/NotFoundPage';
import { Toaster } from '@/components/ui/sonner';

export const Route = createRootRoute({
    component: () => (
        <>
            <Outlet />
            <Toaster />
            {/* Show router devtools in development */}
            {import.meta.env.DEV && <TanStackRouterDevtools />}
        </>
    ),
    notFoundComponent: NotFoundPage,
});
