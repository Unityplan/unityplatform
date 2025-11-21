import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { invitationApi } from '@/api/invitations';
import type { Invitation } from '@/api/invitations';
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { format } from 'date-fns';
import { MoreHorizontal, Loader2, Trash2, Users, Copy } from 'lucide-react';
import { toast } from 'sonner';
import { InvitationUsesDialog } from './InvitationUsesDialog';

export function InvitationList() {
    const [page, setPage] = useState(1);
    const [selectedInvitationId, setSelectedInvitationId] = useState<string | null>(null);
    const [showUsesDialog, setShowUsesDialog] = useState(false);
    const queryClient = useQueryClient(); const { data, isLoading, error } = useQuery({
        queryKey: ['invitations', page],
        queryFn: () => invitationApi.listMyInvitations({ page, limit: 10 }),
    });

    const revokeMutation = useMutation({
        mutationFn: invitationApi.revokeInvitation,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['invitations'] });
            toast.success('Invitation revoked', {
                description: 'The invitation token is no longer valid.',
            });
        },
        onError: () => {
            toast.error('Error', {
                description: 'Failed to revoke invitation.',
            });
        },
    });

    const handleCopyLink = (token: string) => {
        const baseUrl = window.location.origin;
        const url = `${baseUrl}/register?token=${token}`;
        navigator.clipboard.writeText(url);
        toast.success('Copied!', {
            description: 'Invitation link copied to clipboard.',
        });
    }; const getStatusBadge = (invitation: Invitation) => {
        switch (invitation.status) {
            case 'active':
                return <Badge variant="default" className="bg-green-500">Active</Badge>;
            case 'used':
                return <Badge variant="secondary">Fully Used</Badge>;
            case 'expired':
                return <Badge variant="outline" className="text-yellow-600 border-yellow-600">Expired</Badge>;
            case 'revoked':
                return <Badge variant="destructive">Revoked</Badge>;
            default:
                return <Badge variant="outline">{invitation.status}</Badge>;
        }
    };

    if (isLoading) {
        return (
            <div className="flex justify-center p-8">
                <Loader2 className="h-8 w-8 animate-spin" />
            </div>
        );
    }

    if (error) {
        return (
            <div className="text-destructive p-4 border border-destructive rounded-md bg-destructive/10">
                Failed to load invitations. Please try again later.
            </div>
        );
    }

    return (
        <div className="space-y-4">
            <div className="rounded-md border">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Status</TableHead>
                            <TableHead>Uses</TableHead>
                            <TableHead>Expires</TableHead>
                            <TableHead>Created</TableHead>
                            <TableHead className="text-right">Actions</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {data?.data.invitations.length === 0 ? (
                            <TableRow>
                                <TableCell colSpan={5} className="text-center h-24 text-muted-foreground">
                                    No invitations found. Create one to get started.
                                </TableCell>
                            </TableRow>
                        ) : (
                            data?.data.invitations.map((invitation: Invitation) => (
                                <TableRow key={invitation.id}>
                                    <TableCell>{getStatusBadge(invitation)}</TableCell>
                                    <TableCell>
                                        {invitation.uses_count} /{' '}
                                        {invitation.max_uses === 0 ? '∞' : invitation.max_uses}
                                    </TableCell>
                                    <TableCell>
                                        {invitation.expires_at
                                            ? format(new Date(invitation.expires_at), 'PP')
                                            : 'Never'}
                                    </TableCell>
                                    <TableCell>
                                        {format(new Date(invitation.created_at), 'PP')}
                                    </TableCell>
                                    <TableCell className="text-right">
                                        <DropdownMenu>
                                            <DropdownMenuTrigger asChild>
                                                <Button variant="ghost" className="h-8 w-8 p-0">
                                                    <span className="sr-only">Open menu</span>
                                                    <MoreHorizontal className="h-4 w-4" />
                                                </Button>
                                            </DropdownMenuTrigger>
                                            <DropdownMenuContent align="end">
                                                <DropdownMenuLabel>Actions</DropdownMenuLabel>
                                                <DropdownMenuItem onClick={() => handleCopyLink(invitation.token)}>
                                                    <Copy className="mr-2 h-4 w-4" />
                                                    Copy Link
                                                </DropdownMenuItem>
                                                <DropdownMenuItem
                                                    onClick={() => {
                                                        setSelectedInvitationId(invitation.id);
                                                        setShowUsesDialog(true);
                                                    }}
                                                >
                                                    <Users className="mr-2 h-4 w-4" />
                                                    View Usage
                                                </DropdownMenuItem>
                                                <DropdownMenuSeparator />
                                                <DropdownMenuItem
                                                    className="text-destructive focus:text-destructive"
                                                    disabled={invitation.status === 'revoked'}
                                                    onClick={() => revokeMutation.mutate(invitation.id)}
                                                >
                                                    <Trash2 className="mr-2 h-4 w-4" />
                                                    Revoke
                                                </DropdownMenuItem>
                                            </DropdownMenuContent>
                                        </DropdownMenu>
                                    </TableCell>
                                </TableRow>
                            ))
                        )}
                    </TableBody>
                </Table>
            </div>

            {/* Pagination Controls (Simple) */}
            <div className="flex items-center justify-end space-x-2 py-4">
                <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setPage((p) => Math.max(1, p - 1))}
                    disabled={page === 1}
                >
                    Previous
                </Button>
                <div className="text-sm text-muted-foreground">
                    Page {page}
                </div>
                <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setPage((p) => p + 1)}
                    disabled={!data?.data.pagination || data.data.invitations.length < 10}
                >
                    Next
                </Button>
            </div>

            <InvitationUsesDialog
                invitationId={selectedInvitationId}
                open={showUsesDialog}
                onOpenChange={setShowUsesDialog}
            />
        </div>
    );
}
