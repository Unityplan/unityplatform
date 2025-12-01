import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { invitationApi } from '@/api/invitations';
import type { CreateInvitationResponse } from '@/api/invitations';
import type { AxiosResponse } from 'axios';
import { Button } from '@/components/ui/button';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog';
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { toast } from 'sonner';
import { Loader2, Copy, Check } from 'lucide-react';

const formSchema = z.object({
    max_uses: z.coerce
        .number()
        .min(0, 'Must be 0 or greater')
        .max(1000, 'Max 1000 uses'),
    expires_in_days: z.coerce
        .number()
        .min(1, 'Must be at least 1 day')
        .max(365, 'Max 365 days')
        .optional()
        .or(z.literal('')),
    purpose: z.string().max(200, 'Max 200 characters').optional(),
    role: z.enum(['member', 'moderator', 'admin']).default('member'),
});

export function CreateInvitationDialog() {
    const [open, setOpen] = useState(false);
    const [createdToken, setCreatedToken] = useState<{
        token: string;
        url: string;
    } | null>(null);
    const [copied, setCopied] = useState(false);
    const queryClient = useQueryClient();

    const form = useForm<z.infer<typeof formSchema>>({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        resolver: zodResolver(formSchema) as any,
        defaultValues: {
            max_uses: 1,
            expires_in_days: 7,
            purpose: '',
            role: 'member',
        },
    });

    const createMutation = useMutation({
        mutationFn: invitationApi.createInvitation,
        onSuccess: (response: AxiosResponse<CreateInvitationResponse>) => {
            setCreatedToken({
                token: response.data.token,
                url: response.data.invite_url,
            });
            queryClient.invalidateQueries({ queryKey: ['invitations'] });
            toast.success('Invitation created', {
                description: 'You can now share the invitation link.',
            });
        },
        onError: (error: Error & { response?: { data?: { message?: string } } }) => {
            toast.error('Error', {
                description: error.response?.data?.message || 'Failed to create invitation',
            });
        },
    });

    const onSubmit = (values: z.infer<typeof formSchema>) => {
        createMutation.mutate({
            max_uses: values.max_uses,
            expires_in_days: values.expires_in_days === '' ? undefined : Number(values.expires_in_days),
            metadata: {
                ...(values.purpose ? { purpose: values.purpose } : {}),
                role: values.role,
            },
        });
    };

    const copyToClipboard = () => {
        if (createdToken) {
            navigator.clipboard.writeText(createdToken.url);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
            toast.success('Copied!', {
                description: 'Invitation link copied to clipboard.',
            });
        }
    };

    const handleClose = () => {
        setOpen(false);
        setCreatedToken(null);
        form.reset();
    };

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button>Create Invitation</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>Create Invitation</DialogTitle>
                    <DialogDescription>
                        Generate a new invitation link for new users.
                    </DialogDescription>
                </DialogHeader>

                {!createdToken ? (
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                            <FormField
                                control={form.control}
                                name="role"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Role</FormLabel>
                                        <Select onValueChange={field.onChange} defaultValue={field.value}>
                                            <FormControl>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Select a role" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectItem value="member">Member</SelectItem>
                                                <SelectItem value="moderator">Moderator</SelectItem>
                                                <SelectItem value="admin">Admin</SelectItem>
                                            </SelectContent>
                                        </Select>
                                        <FormDescription>
                                            The role assigned to users who join with this invite.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="max_uses"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Max Uses</FormLabel>
                                        <FormControl>
                                            <Input type="number" {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            0 for unlimited uses.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="expires_in_days"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Expires In (Days)</FormLabel>
                                        <FormControl>
                                            <Input type="number" {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            Leave empty for no expiration.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="purpose"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Purpose / Note</FormLabel>
                                        <FormControl>
                                            <Input placeholder="e.g. For new community members" {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            Optional note to remember what this invitation is for.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <DialogFooter>
                                <Button type="submit" disabled={createMutation.isPending}>
                                    {createMutation.isPending && (
                                        <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                    )}
                                    Create
                                </Button>
                            </DialogFooter>
                        </form>
                    </Form>
                ) : (
                    <div className="space-y-4">
                        <div className="p-4 bg-muted rounded-md break-all text-sm font-mono">
                            {createdToken.url}
                        </div>
                        <Button
                            onClick={copyToClipboard}
                            className="w-full"
                            variant="outline"
                        >
                            {copied ? (
                                <>
                                    <Check className="mr-2 h-4 w-4" /> Copied
                                </>
                            ) : (
                                <>
                                    <Copy className="mr-2 h-4 w-4" /> Copy Link
                                </>
                            )}
                        </Button>
                        <DialogFooter>
                            <Button onClick={handleClose}>Done</Button>
                        </DialogFooter>
                    </div>
                )}
            </DialogContent>
        </Dialog>
    );
}
