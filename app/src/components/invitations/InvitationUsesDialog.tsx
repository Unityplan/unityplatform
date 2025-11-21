import { useQuery } from '@tanstack/react-query';
import { invitationApi } from '@/api/invitations';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/components/ui/table';
import { format } from 'date-fns';
import { Loader2 } from 'lucide-react';

interface InvitationUsesDialogProps {
    invitationId: string | null;
    open: boolean;
    onOpenChange: (open: boolean) => void;
}

export function InvitationUsesDialog({
    invitationId,
    open,
    onOpenChange,
}: InvitationUsesDialogProps) {
    const { data, isLoading, error } = useQuery({
        queryKey: ['invitation-uses', invitationId],
        queryFn: () => invitationApi.getInvitationUses(invitationId!),
        enabled: !!invitationId && open,
    });

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className="sm:max-w-[600px]">
                <DialogHeader>
                    <DialogTitle>Invitation Usage History</DialogTitle>
                    <DialogDescription>
                        See who has used this invitation token.
                    </DialogDescription>
                </DialogHeader>

                {isLoading ? (
                    <div className="flex justify-center p-8">
                        <Loader2 className="h-8 w-8 animate-spin" />
                    </div>
                ) : error ? (
                    <div className="text-destructive p-4">
                        Failed to load usage history.
                    </div>
                ) : (
                    <div className="max-h-[400px] overflow-y-auto">
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead>User</TableHead>
                                    <TableHead>Used At</TableHead>
                                    <TableHead>IP Address</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {data?.data.uses.length === 0 ? (
                                    <TableRow>
                                        <TableCell colSpan={3} className="text-center text-muted-foreground">
                                            No uses yet.
                                        </TableCell>
                                    </TableRow>
                                ) : (
                                    data?.data.uses.map((use, index) => (
                                        <TableRow key={index}>
                                            <TableCell>
                                                <div className="font-medium">{use.username}</div>
                                                <div className="text-xs text-muted-foreground">{use.user_id}</div>
                                            </TableCell>
                                            <TableCell>
                                                {format(new Date(use.used_at), 'PP p')}
                                            </TableCell>
                                            <TableCell className="font-mono text-xs">
                                                {use.ip_address || 'Unknown'}
                                            </TableCell>
                                        </TableRow>
                                    ))
                                )}
                            </TableBody>
                        </Table>
                    </div>
                )}
            </DialogContent>
        </Dialog>
    );
}
