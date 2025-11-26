import { cn } from '@/lib/utils'
import { ReactNode } from 'react'

interface PageContainerProps {
    children: ReactNode
    className?: string
    /**
     * If true, restricts the content width to a standard container size.
     * Useful for forms or text-heavy pages.
     */
    centered?: boolean
}

/**
 * Standard page container component to ensure consistent padding and width across the application.
 * Use this as the direct child of AppLayout.
 */
export function PageContainer({ children, className, centered = false }: PageContainerProps) {
    return (
        <div
            className={cn(
                "w-full px-4 py-8 md:px-8", // Standard full-width padding
                centered && "container mx-auto max-w-2xl", // Optional centered constraint
                className
            )}
        >
            {children}
        </div>
    )
}
