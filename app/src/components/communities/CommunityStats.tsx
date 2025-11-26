import {
    Area,
    AreaChart,
    Bar,
    BarChart,
    CartesianGrid,
    ResponsiveContainer,
    Tooltip,
    XAxis,
    YAxis,
} from 'recharts'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Users, Activity, TrendingUp } from 'lucide-react'

const growthData = [
    { month: 'Jan', members: 120 },
    { month: 'Feb', members: 132 },
    { month: 'Mar', members: 145 },
    { month: 'Apr', members: 160 },
    { month: 'May', members: 185 },
    { month: 'Jun', members: 210 },
]

const activityData = [
    { day: 'Mon', posts: 12, comments: 45 },
    { day: 'Tue', posts: 18, comments: 52 },
    { day: 'Wed', posts: 15, comments: 48 },
    { day: 'Thu', posts: 22, comments: 60 },
    { day: 'Fri', posts: 28, comments: 75 },
    { day: 'Sat', posts: 10, comments: 30 },
    { day: 'Sun', posts: 8, comments: 25 },
]

export function CommunityStats() {
    return (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
            <div className="col-span-4">
                <Card className="h-full">
                    <CardHeader>
                        <CardTitle className="flex items-center gap-2">
                            <TrendingUp className="h-5 w-5 text-primary" />
                            Community Growth
                        </CardTitle>
                        <CardDescription>
                            Total members over the last 6 months
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="h-[300px]">
                            <ResponsiveContainer width="100%" height="100%">
                                <AreaChart data={growthData}>
                                    <defs>
                                        <linearGradient id="colorMembers" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="5%" stopColor="#2563eb" stopOpacity={0.8} />
                                            <stop offset="95%" stopColor="#2563eb" stopOpacity={0} />
                                        </linearGradient>
                                    </defs>
                                    <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                                    <XAxis dataKey="month" className="text-xs" />
                                    <YAxis className="text-xs" />
                                    <Tooltip
                                        contentStyle={{
                                            backgroundColor: 'hsl(var(--card))',
                                            borderColor: 'hsl(var(--border))',
                                            borderRadius: 'var(--radius)',
                                        }}
                                    />
                                    <Area
                                        type="monotone"
                                        dataKey="members"
                                        stroke="#2563eb"
                                        fillOpacity={1}
                                        fill="url(#colorMembers)"
                                    />
                                </AreaChart>
                            </ResponsiveContainer>
                        </div>
                    </CardContent>
                </Card>
            </div>

            <div className="col-span-3">
                <Card className="h-full">
                    <CardHeader>
                        <CardTitle className="flex items-center gap-2">
                            <Activity className="h-5 w-5 text-primary" />
                            Weekly Activity
                        </CardTitle>
                        <CardDescription>
                            Posts and comments this week
                        </CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div className="h-[300px]">
                            <ResponsiveContainer width="100%" height="100%">
                                <BarChart data={activityData}>
                                    <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                                    <XAxis dataKey="day" className="text-xs" />
                                    <YAxis className="text-xs" />
                                    <Tooltip
                                        cursor={{ fill: 'hsl(var(--muted)/0.2)' }}
                                        contentStyle={{
                                            backgroundColor: 'hsl(var(--card))',
                                            borderColor: 'hsl(var(--border))',
                                            borderRadius: 'var(--radius)',
                                        }}
                                    />
                                    <Bar dataKey="posts" fill="#2563eb" radius={[4, 4, 0, 0]} name="Posts" />
                                    <Bar dataKey="comments" fill="#16a34a" radius={[4, 4, 0, 0]} name="Comments" />
                                </BarChart>
                            </ResponsiveContainer>
                        </div>
                    </CardContent>
                </Card>
            </div>

            <div className="col-span-7 grid grid-cols-1 gap-4 md:grid-cols-3">
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Total Members</CardTitle>
                        <Users className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">210</div>
                        <p className="text-xs text-muted-foreground">
                            +12% from last month
                        </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Active Users</CardTitle>
                        <Activity className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">85</div>
                        <p className="text-xs text-muted-foreground">
                            +4% from last week
                        </p>
                    </CardContent>
                </Card>
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">New Join Requests</CardTitle>
                        <Users className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">12</div>
                        <p className="text-xs text-muted-foreground">
                            Requires attention
                        </p>
                    </CardContent>
                </Card>
            </div>
        </div>
    )
}
