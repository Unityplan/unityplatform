import { useState } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Lock, LogOut } from 'lucide-react';
import { toast } from 'sonner';

/**
 * SessionLockScreen Component
 * 
 * Displayed when the user's session is locked due to inactivity.
 * Provides options to:
 * 1. Unlock with password re-authentication
 * 2. Logout and return to login page
 * 
 * **Security Features:**
 * - Requires password re-entry (no stored password reuse)
 * - Option to fully logout for security
 * - Prevents unauthorized access after inactivity
 * - Shows locked time for transparency
 */
export function SessionLockScreen() {
    const { user, unlockSession, logout } = useAuthStore();
    const [password, setPassword] = useState('');
    const [isUnlocking, setIsUnlocking] = useState(false);

    const handleUnlock = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!password.trim()) {
            toast.error('Please enter your password');
            return;
        }

        if (!user?.email) {
            toast.error('User information not available');
            return;
        }

        setIsUnlocking(true);

        try {
            // Re-authenticate with current credentials
            await unlockSession(user.email, password);
            toast.success('Session unlocked successfully');
        } catch (error) {
            console.error('Unlock failed:', error);
            toast.error('Incorrect password. Please try again.');
            setPassword('');
        } finally {
            setIsUnlocking(false);
        }
    };

    const handleLogout = async () => {
        try {
            await logout();
            toast.success('Logged out successfully');
        } catch (error) {
            console.error('Logout failed:', error);
            toast.error('Logout failed. Please try again.');
        }
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-background/95 backdrop-blur-sm">
            <Card className="w-full max-w-md">
                <CardHeader className="space-y-1 text-center">
                    <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-muted">
                        <Lock className="h-8 w-8 text-muted-foreground" />
                    </div>
                    <CardTitle className="text-2xl font-bold">Session Locked</CardTitle>
                    <CardDescription>
                        Your session has been locked due to inactivity.
                        {user && (
                            <span className="mt-2 block text-sm">
                                Signed in as <span className="font-medium">{user.email}</span>
                            </span>
                        )}
                    </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                    <form onSubmit={handleUnlock} className="space-y-4">
                        <div className="space-y-2">
                            <Label htmlFor="password">Password</Label>
                            <Input
                                id="password"
                                type="password"
                                placeholder="Enter your password"
                                value={password}
                                onChange={(e) => setPassword(e.target.value)}
                                disabled={isUnlocking}
                                autoFocus
                                required
                            />
                        </div>

                        <div className="space-y-2">
                            <Button type="submit" className="w-full" disabled={isUnlocking}>
                                {isUnlocking ? 'Unlocking...' : 'Unlock Session'}
                            </Button>

                            <Button
                                type="button"
                                variant="outline"
                                className="w-full"
                                onClick={handleLogout}
                                disabled={isUnlocking}
                            >
                                <LogOut className="mr-2 h-4 w-4" />
                                Logout
                            </Button>
                        </div>
                    </form>

                    <div className="rounded-lg bg-muted p-3 text-center text-sm text-muted-foreground">
                        <p>For your security, we lock your session after 10 minutes of inactivity.</p>
                    </div>
                </CardContent>
            </Card>
        </div>
    );
}
