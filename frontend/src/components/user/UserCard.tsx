import { Avatar } from './Avatar';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import { UserPlus, UserMinus, MoreHorizontal } from 'lucide-react';

interface UserCardProps {
  user: {
    id: string;
    username: string;
    full_name?: string | null;
    avatar_url?: string | null;
    bio?: string | null;
    is_verified?: boolean;
  };
  showActions?: boolean;
  isFollowing?: boolean;
  onFollow?: () => void;
  onUnfollow?: () => void;
  className?: string;
  compact?: boolean;
}

/**
 * UserCard component for displaying user information
 * 
 * Used in connection lists, search results, suggestions, etc.
 * Supports follow/unfollow actions and verified badge
 * 
 * @example
 * ```tsx
 * <UserCard 
 *   user={user}
 *   showActions
 *   isFollowing={false}
 *   onFollow={() => handleFollow(user.id)}
 * />
 * ```
 */
export function UserCard({
  user,
  showActions = true,
  isFollowing = false,
  onFollow,
  onUnfollow,
  className,
  compact = false,
}: UserCardProps) {
  const handleActionClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    
    if (isFollowing && onUnfollow) {
      onUnfollow();
    } else if (!isFollowing && onFollow) {
      onFollow();
    }
  };

  const truncateBio = (text: string | null | undefined, maxLength: number = 100): string => {
    if (!text) return '';
    if (text.length <= maxLength) return text;
    return text.substring(0, maxLength).trim() + '...';
  };

  return (
    <Card className={cn('hover:bg-muted/50 transition-colors', className)}>
      <CardContent className={cn('p-4', compact && 'p-3')}>
        <div className="flex items-start gap-3">
          {/* Avatar */}
          <a href={`/profile/${user.username}`} className="flex-shrink-0">
            <Avatar
              src={user.avatar_url}
              alt={user.username}
              fallback={user.full_name || user.username}
              size={compact ? 'md' : 'lg'}
              showOnlineStatus={!compact}
            />
          </a>

          {/* User Info */}
          <div className="flex-1 min-w-0">
            <a 
              href={`/profile/${user.username}`}
              className="hover:underline"
            >
              <div className="flex items-center gap-2">
                <h3 className="font-semibold text-foreground truncate">
                  {user.full_name || user.username}
                </h3>
                {user.is_verified && (
                  <Badge variant="secondary" className="text-xs">
                    ✓ Verified
                  </Badge>
                )}
              </div>
              <p className="text-sm text-muted-foreground">@{user.username}</p>
            </a>

            {!compact && user.bio && (
              <p className="mt-2 text-sm text-muted-foreground">
                {truncateBio(user.bio, 120)}
              </p>
            )}
          </div>

          {/* Actions */}
          {showActions && (
            <div className="flex items-center gap-2 flex-shrink-0">
              <Button
                size={compact ? 'sm' : 'default'}
                onClick={handleActionClick}
                className={cn(
                  'gap-2',
                  isFollowing && 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
                )}
              >
                {isFollowing ? (
                  <>
                    <UserMinus className="h-4 w-4" />
                    {!compact && 'Unfollow'}
                  </>
                ) : (
                  <>
                    <UserPlus className="h-4 w-4" />
                    {!compact && 'Follow'}
                  </>
                )}
              </Button>

              {!compact && (
                <Button 
                  size="icon"
                  className="bg-transparent text-foreground hover:bg-muted"
                >
                  <MoreHorizontal className="h-4 w-4" />
                </Button>
              )}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}
