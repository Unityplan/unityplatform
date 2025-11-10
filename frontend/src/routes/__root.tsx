import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { NotFoundPage } from '@/pages/errors/NotFoundPage';

export const Route = createRootRoute({
    component: () => (
        <>
            <Outlet />
            {/* Show router devtools in development */}
            {import.meta.env.DEV && <TanStackRouterDevtools />}
        </>
    ),
    notFoundComponent: NotFoundPage,
});
