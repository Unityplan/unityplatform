import { Avatar } from './Avatar';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { 
  MapPin, 
  Calendar, 
  Link as LinkIcon, 
  UserPlus,
  UserMinus,
  MessageCircle,
  Settings,
  MoreHorizontal
} from 'lucide-react';
import { format } from 'date-fns';

interface ProfileHeaderProps {
  user: {
    id: string;
    username: string;
    full_name?: string | null;
    avatar_url?: string | null;
    bio?: string | null;
    location?: string | null;
    website?: string | null;
    created_at: string;
    is_verified?: boolean;
  };
  stats?: {
    following: number;
    followers: number;
    posts?: number;
  };
  isOwnProfile?: boolean;
  isFollowing?: boolean;
  onFollow?: () => void;
  onUnfollow?: () => void;
  onMessage?: () => void;
  onEditProfile?: () => void;
  className?: string;
}

/**
 * ProfileHeader component - Social network style profile header
 * 
 * Features:
 * - Banner background (placeholder gradient)
 * - Large avatar with verified badge
 * - User info (name, username, bio, location, website, join date)
 * - Stats (followers, following, posts)
 * - Action buttons (follow/message/edit based on viewer)
 * 
 * @example
 * ```tsx
 * <ProfileHeader 
 *   user={user}
 *   stats={{ followers: 245, following: 189 }}
 *   isOwnProfile={false}
 *   isFollowing={true}
 *   onFollow={handleFollow}
 *   onMessage={handleMessage}
 * />
 * ```
 */
export function ProfileHeader({
  user,
  stats = { following: 0, followers: 0, posts: 0 },
  isOwnProfile = false,
  isFollowing = false,
  onFollow,
  onUnfollow,
  onMessage,
  onEditProfile,
  className,
}: ProfileHeaderProps) {
  const joinDate = format(new Date(user.created_at), 'MMMM yyyy');

  const handleFollowClick = () => {
    if (isFollowing && onUnfollow) {
      onUnfollow();
    } else if (!isFollowing && onFollow) {
      onFollow();
    }
  };

  return (
    <Card className={cn('overflow-hidden', className)}>
      {/* Banner */}
      <div className="h-48 bg-gradient-to-r from-primary/20 via-accent/20 to-secondary/20" />

      {/* Profile Content */}
      <div className="px-6 pb-6">
        {/* Avatar & Action Buttons Row */}
        <div className="flex items-end justify-between -mt-16 mb-4">
          {/* Avatar */}
          <div className="relative">
            <Avatar
              src={user.avatar_url}
              alt={user.username}
              fallback={user.full_name || user.username}
              size="2xl"
              className="ring-4 ring-background"
              showOnlineStatus
              isOnline={false}
            />
          </div>

          {/* Action Buttons */}
          <div className="flex items-center gap-2 mb-2">
            {isOwnProfile ? (
              <Button
                size="default"
                onClick={onEditProfile}
                className="gap-2"
              >
                <Settings className="h-4 w-4" />
                Edit Profile
              </Button>
            ) : (
              <>
                <Button
                  size="default"
                  onClick={handleFollowClick}
                  className={cn(
                    'gap-2',
                    isFollowing && 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
                  )}
                >
                  {isFollowing ? (
                    <>
                      <UserMinus className="h-4 w-4" />
                      Unfollow
                    </>
                  ) : (
                    <>
                      <UserPlus className="h-4 w-4" />
                      Follow
                    </>
                  )}
                </Button>
                <Button
                  size="default"
                  onClick={onMessage}
                  className="gap-2 bg-secondary text-secondary-foreground hover:bg-secondary/80"
                >
                  <MessageCircle className="h-4 w-4" />
                  Message
                </Button>
                <Button 
                  size="icon"
                  className="bg-transparent text-foreground hover:bg-muted"
                >
                  <MoreHorizontal className="h-4 w-4" />
                </Button>
              </>
            )}
          </div>
        </div>

        {/* User Info */}
        <div className="space-y-3">
          {/* Name & Username */}
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-2xl font-bold text-foreground">
                {user.full_name || user.username}
              </h1>
              {user.is_verified && (
                <Badge variant="secondary" className="text-sm">
                  ✓ Verified
                </Badge>
              )}
            </div>
            <p className="text-muted-foreground">@{user.username}</p>
          </div>

          {/* Bio */}
          {user.bio && (
            <p className="text-foreground">{user.bio}</p>
          )}

          {/* Metadata */}
          <div className="flex flex-wrap gap-4 text-sm text-muted-foreground">
            {user.location && (
              <div className="flex items-center gap-1">
                <MapPin className="h-4 w-4" />
                <span>{user.location}</span>
              </div>
            )}
            {user.website && (
              <div className="flex items-center gap-1">
                <LinkIcon className="h-4 w-4" />
                <a 
                  href={user.website} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="text-primary hover:underline"
                >
                  {user.website.replace(/^https?:\/\/(www\.)?/, '')}
                </a>
              </div>
            )}
            <div className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              <span>Joined {joinDate}</span>
            </div>
          </div>

          {/* Stats */}
          <div className="flex gap-6 pt-2">
            <a href={`/profile/${user.username}/following`} className="hover:underline">
              <span className="font-semibold text-foreground">{stats.following}</span>
              <span className="text-muted-foreground ml-1">Following</span>
            </a>
            <a href={`/profile/${user.username}/followers`} className="hover:underline">
              <span className="font-semibold text-foreground">{stats.followers}</span>
              <span className="text-muted-foreground ml-1">Followers</span>
            </a>
            {stats.posts !== undefined && stats.posts > 0 && (
              <div>
                <span className="font-semibold text-foreground">{stats.posts}</span>
                <span className="text-muted-foreground ml-1">Posts</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </Card>
  );
}
