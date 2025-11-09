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
    <div className="min-h-screen pb-30">
      {breadcrumbs && (
        <div className="sticky top-0 z-10 bg-background/90 backdrop-blur-sm">
          <div className="px-4 py-4 sm:px-6">
            <div className="min-w-0">{breadcrumbs}</div>
          </div>
        </div>
      )}
      <div className="px-4 sm:px-6">
        <div className="mx-auto max-w-6xl">{children}</div>
      </div>
    </div>
  );
}
