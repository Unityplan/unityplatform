import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useAuthStore } from '@/stores/authStore';

export function DashboardPage() {
    const { user } = useAuthStore();

    return (
        <div className="w-full px-4 py-10 space-y-8 md:px-8">
            <div className="flex items-center justify-between space-y-2">
                <div>
                    <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
                    <p className="text-muted-foreground">
                        Welcome back, {user?.username || 'User'}.
                    </p>
                </div>
            </div>
        </div>
    );
}
