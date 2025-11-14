import type React from 'react';

interface CenteredLayoutProps {
    breadcrumbs?: React.ReactNode;
    children: React.ReactNode;
}

/**
 * Centered page layout - perfect for auth pages, simple content pages
 * Provides a centered container with optional breadcrumbs/header area
 */
export function CenteredLayout({ breadcrumbs, children }: CenteredLayoutProps) {
    return (
        <div className="relative flex min-h-screen items-center justify-center p-4 bg-background">
            {/* Subtle organic pattern overlay */}
            <div
                className="absolute inset-0 -z-10 opacity-[0.02] dark:opacity-[0.03]"
                style={{
                    backgroundImage: `radial-gradient(circle at 25% 25%, currentColor 1px, transparent 1px),
                                     radial-gradient(circle at 75% 75%, currentColor 1px, transparent 1px)`,
                    backgroundSize: '50px 50px'
                }}
            />

            <div className="w-full">
                {breadcrumbs && (
                    <div className="mb-8">
                        {breadcrumbs}
                    </div>
                )}
                <div className="flex justify-center">
                    {children}
                </div>
            </div>
        </div>
    );
}
