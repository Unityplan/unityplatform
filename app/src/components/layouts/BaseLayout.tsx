import { clsx } from 'clsx';
import type React from 'react';

interface BaseLayoutProps {
    children: React.ReactNode;
    className?: string;
}

/**
 * Base layout wrapper that provides font classes and basic structure.
 * Use this as the root wrapper for all pages.
 */
export function BaseLayout({ children, className }: BaseLayoutProps) {
    return (
        <div className={clsx('min-h-screen font-sans antialiased', className)}>
            <div className="isolate">{children}</div>
        </div>
    );
}
