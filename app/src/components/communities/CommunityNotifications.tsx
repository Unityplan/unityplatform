import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Bell, UserPlus, Flag, MessageSquare, Check, X } from 'lucide-react'

interface Notification {
    id: string
    type: 'join_request' | 'report' | 'mention'
    title: string
    description: string
    time: string
    read: boolean
}

const notifications: Notification[] = [
    {
        id: '1',
        type: 'join_request',
        title: 'New Join Request',
        description: 'Alice wants to join "Rust Developers"',
        time: '10 min ago',
        read: false,
    },
    {
        id: '2',
        type: 'report',
        title: 'Content Reported',
        description: 'A comment in "General" was reported for spam',
        time: '1 hour ago',
        read: false,
    },
    {
        id: '3',
        type: 'join_request',
        title: 'New Join Request',
        description: 'Bob wants to join "Rust Developers"',
        time: '2 hours ago',
        read: true,
    },
    {
        id: '4',
        type: 'mention',
        title: 'Mentioned in Discussion',
        description: 'Charlie mentioned you in "Project Ideas"',
        time: '1 day ago',
        read: true,
    },
]

export function CommunityNotifications() {
    return (
        <Card className="h-full">
            <CardHeader>
                <div className="flex items-center justify-between">
                    <div className="space-y-1">
                        <CardTitle className="flex items-center gap-2">
                            <Bell className="h-5 w-5 text-primary" />
                            Notifications
                        </CardTitle>
                        <CardDescription>
                            Alerts requiring your attention
                        </CardDescription>
                    </div>
                    <Badge variant="secondary">2 New</Badge>
                </div>
            </CardHeader>
            <CardContent>
                <div className="h-[400px] overflow-y-auto pr-4">
                    <div className="space-y-4">
                        {notifications.map((notification) => (
                            <div
                                key={notification.id}
                                className={`flex items-start gap-4 rounded-lg border p-4 transition-colors ${!notification.read ? 'bg-muted/50' : ''
                                    }`}
                            >
                                <div className={`rounded-full p-2 ${notification.type === 'join_request' ? 'bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400' :
                                        notification.type === 'report' ? 'bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400' :
                                            'bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400'
                                    }`}>
                                    {notification.type === 'join_request' && <UserPlus className="h-4 w-4" />}
                                    {notification.type === 'report' && <Flag className="h-4 w-4" />}
                                    {notification.type === 'mention' && <MessageSquare className="h-4 w-4" />}
                                </div>
                                <div className="flex-1 space-y-1">
                                    <div className="flex items-center justify-between">
                                        <p className="text-sm font-medium leading-none">
                                            {notification.title}
                                        </p>
                                        <span className="text-xs text-muted-foreground">
                                            {notification.time}
                                        </span>
                                    </div>
                                    <p className="text-sm text-muted-foreground">
                                        {notification.description}
                                    </p>
                                    {notification.type === 'join_request' && (
                                        <div className="mt-2 flex gap-2">
                                            <Button size="sm" variant="default" className="h-7 text-xs">
                                                <Check className="mr-1 h-3 w-3" /> Approve
                                            </Button>
                                            <Button size="sm" variant="outline" className="h-7 text-xs">
                                                <X className="mr-1 h-3 w-3" /> Deny
                                            </Button>
                                        </div>
                                    )}
                                    {notification.type === 'report' && (
                                        <div className="mt-2 flex gap-2">
                                            <Button size="sm" variant="destructive" className="h-7 text-xs">
                                                Review Content
                                            </Button>
                                        </div>
                                    )}
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </CardContent>
        </Card>
    )
}
