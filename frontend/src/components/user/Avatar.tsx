import { cn } from '@/lib/utils';
import { User } from 'lucide-react';

export type AvatarSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl';

interface AvatarProps {
    src?: string | null;
    alt?: string;
    fallback?: string;
    size?: AvatarSize;
    className?: string;
    showOnlineStatus?: boolean;
    isOnline?: boolean;
}

const sizeClasses: Record<AvatarSize, string> = {
    xs: 'h-6 w-6 text-xs',
    sm: 'h-8 w-8 text-sm',
    md: 'h-10 w-10 text-base',
    lg: 'h-12 w-12 text-lg',
    xl: 'h-16 w-16 text-xl',
    '2xl': 'h-24 w-24 text-2xl',
    '3xl': 'h-32 w-32 text-3xl',
    '4xl': 'h-48 w-48 text-4xl',
};

const statusSizeClasses: Record<AvatarSize, string> = {
    xs: 'h-1.5 w-1.5 border',
    sm: 'h-2 w-2 border',
    md: 'h-2.5 w-2.5 border-2',
    lg: 'h-3 w-3 border-2',
    xl: 'h-4 w-4 border-2',
    '2xl': 'h-5 w-5 border-2',
    '3xl': 'h-6 w-6 border-2',
    '4xl': 'h-8 w-8 border-2',
};

/**
 * Avatar component with fallback to initials
 * 
 * Supports multiple sizes, online status indicator, and graceful image loading
 * 
 * @example
 * ```tsx
 * <Avatar 
 *   src={user.avatar_url} 
 *   alt={user.username}
 *   fallback={user.username}
 *   size="md"
 *   showOnlineStatus
 *   isOnline={true}
 * />
 * ```
 */
export function Avatar({
    src,
    alt = '',
    fallback = '',
    size = 'md',
    className,
    showOnlineStatus = false,
    isOnline = false,
}: AvatarProps) {
    // Extract initials from fallback (first 2 characters of first 2 words)
    const getInitials = (text: string): string => {
        if (!text) return '';
        const words = text.trim().split(/\s+/);
        if (words.length === 1) {
            return words[0].substring(0, 2).toUpperCase();
        }
        return (words[0][0] + words[1][0]).toUpperCase();
    };

    const initials = getInitials(fallback || alt);

    return (
        <div className={cn('relative inline-block', className)}>
            <div
                className={cn(
                    'relative flex items-center justify-center overflow-hidden rounded-full bg-muted',
                    sizeClasses[size]
                )}
            >
                {src ? (
                    <img
                        src={src}
                        alt={alt}
                        className="h-full w-full object-cover"
                        onError={(e) => {
                            // Hide broken image and show fallback
                            e.currentTarget.style.display = 'none';
                        }}
                    />
                ) : initials ? (
                    <span className="font-medium text-muted-foreground select-none">
                        {initials}
                    </span>
                ) : (
                    <User className="h-1/2 w-1/2 text-muted-foreground" />
                )}
            </div>

            {/* Online status indicator */}
            {showOnlineStatus && (
                <span
                    className={cn(
                        'absolute bottom-0 right-0 rounded-full border-background',
                        statusSizeClasses[size],
                        isOnline ? 'bg-green-500' : 'bg-gray-400'
                    )}
                    aria-label={isOnline ? 'Online' : 'Offline'}
                />
            )}
        </div>
    );
}
