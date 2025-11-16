import { useState, useEffect } from 'react';
import { Pencil, Plus, Trash2, ExternalLink, Globe } from 'lucide-react';
import type { ProfileLink, CreateProfileLinkRequest, UpdateProfileLinkRequest } from '@/api/users';
import { fetchFavicon } from '@/api/utility';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from '@/components/ui/table';
import { Badge } from '@/components/ui/badge';
import { ResponsiveDialog } from '@/components/ui/responsive-dialog';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Switch } from '@/components/ui/switch';

interface ProfileLinksManagerProps {
    links: ProfileLink[];
    onAdd: (link: CreateProfileLinkRequest) => Promise<void>;
    onUpdate: (id: string, link: UpdateProfileLinkRequest) => Promise<void>;
    onDelete: (id: string) => Promise<void>;
    disabled?: boolean;
    maxLinks?: number;
}

export function ProfileLinksManager({
    links,
    onAdd,
    onUpdate,
    onDelete,
    disabled = false,
    maxLinks = 10,
}: ProfileLinksManagerProps) {
    const [dialogOpen, setDialogOpen] = useState(false);
    const [editingLink, setEditingLink] = useState<ProfileLink | null>(null);
    const [formData, setFormData] = useState({
        label: '',
        url: '',
        icon: '',
        isVisible: true,
    });
    const [formErrors, setFormErrors] = useState({
        label: '',
        url: '',
    });

    // Store favicon URLs for each link
    const [faviconUrls, setFaviconUrls] = useState<Record<string, string>>({});

    // Fetch favicons for all links
    useEffect(() => {
        const loadFavicons = async () => {
            // Cleanup old URLs before fetching new ones
            Object.values(faviconUrls).forEach(url => {
                if (url.startsWith('blob:')) {
                    URL.revokeObjectURL(url);
                }
            });

            const newFaviconUrls: Record<string, string> = {};

            for (const link of links) {
                try {
                    const faviconUrl = await fetchFavicon(link.url, 32);
                    newFaviconUrls[link.id] = faviconUrl;
                } catch (error) {
                    console.error(`Failed to fetch favicon for ${link.url}:`, error);
                    // Don't set a URL if fetch fails, component will show fallback
                }
            }

            setFaviconUrls(newFaviconUrls);
        };

        if (links.length > 0) {
            loadFavicons();
        } else {
            // Cleanup when no links
            Object.values(faviconUrls).forEach(url => {
                if (url.startsWith('blob:')) {
                    URL.revokeObjectURL(url);
                }
            });
            setFaviconUrls({});
        }

        // Cleanup on unmount
        return () => {
            Object.values(faviconUrls).forEach(url => {
                if (url.startsWith('blob:')) {
                    URL.revokeObjectURL(url);
                }
            });
        };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [links]);

    // Sort links by displayOrder
    const sortedLinks = [...links].sort((a, b) => a.displayOrder - b.displayOrder);

    const validateForm = (): boolean => {
        const errors = { label: '', url: '' };
        let isValid = true;

        if (!formData.label.trim()) {
            errors.label = 'Label is required';
            isValid = false;
        }

        if (!formData.url.trim()) {
            errors.url = 'URL is required';
            isValid = false;
        } else {
            try {
                new URL(formData.url);
            } catch {
                errors.url = 'Please enter a valid URL';
                isValid = false;
            }
        }

        setFormErrors(errors);
        return isValid;
    };

    const handleOpenDialog = (link?: ProfileLink) => {
        if (link) {
            setEditingLink(link);
            setFormData({
                label: link.label,
                url: link.url,
                icon: link.icon || '',
                isVisible: link.isVisible,
            });
        } else {
            setEditingLink(null);
            setFormData({
                label: '',
                url: '',
                icon: '',
                isVisible: true,
            });
        }
        setFormErrors({ label: '', url: '' });
        setDialogOpen(true);
    };

    const handleSave = async () => {
        if (!validateForm()) return;

        try {
            if (editingLink) {
                await onUpdate(editingLink.id, {
                    label: formData.label,
                    url: formData.url,
                    icon: formData.icon || null,
                    isVisible: formData.isVisible,
                });
            } else {
                await onAdd({
                    label: formData.label,
                    url: formData.url,
                    icon: formData.icon || null,
                    displayOrder: links.length,
                    isVisible: formData.isVisible,
                });
            }
            setDialogOpen(false);
        } catch (err) {
            console.error('Failed to save link:', err);
        }
    };

    const handleDelete = async () => {
        if (editingLink) {
            try {
                await onDelete(editingLink.id);
                setDialogOpen(false);
            } catch (err) {
                console.error('Failed to delete link:', err);
            }
        }
    };

    const canAddMore = links.length < maxLinks;

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
                <div>
                    <CardTitle>Profile Links</CardTitle>
                    <CardDescription>
                        Add links to your website, social media, or other online profiles (max {maxLinks})
                    </CardDescription>
                </div>
                <Button
                    type="button"
                    size="sm"
                    onClick={() => handleOpenDialog()}
                    disabled={disabled || !canAddMore}
                >
                    <Plus className="h-4 w-4 mr-2" />
                    Add Link
                </Button>
            </CardHeader>
            <CardContent>
                {sortedLinks.length === 0 ? (
                    <div className="text-center py-8 text-muted-foreground">
                        No links added yet. Click "Add Link" to get started.
                    </div>
                ) : (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead className="w-[50px]">Icon</TableHead>
                                <TableHead>Label</TableHead>
                                <TableHead>URL</TableHead>
                                <TableHead className="w-[100px]">Visibility</TableHead>
                                <TableHead className="w-[50px]"></TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {sortedLinks.map((link) => (
                                <TableRow key={link.id}>
                                    <TableCell>
                                        {faviconUrls[link.id] ? (
                                            <img
                                                src={faviconUrls[link.id]}
                                                alt={`${link.label} icon`}
                                                className="h-6 w-6 rounded"
                                                onError={(e) => {
                                                    // Fallback to globe icon if image fails to load
                                                    e.currentTarget.style.display = 'none';
                                                    const parent = e.currentTarget.parentElement;
                                                    if (parent && !parent.querySelector('svg')) {
                                                        const fallback = document.createElement('div');
                                                        fallback.innerHTML = '<svg class="h-6 w-6 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" stroke-width="2"/><path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" stroke-width="2"/></svg>';
                                                        parent.appendChild(fallback.firstChild!);
                                                    }
                                                }}
                                            />
                                        ) : (
                                            <Globe className="h-6 w-6 text-muted-foreground" />
                                        )}
                                    </TableCell>
                                    <TableCell className="font-medium">{link.label}</TableCell>
                                    <TableCell>
                                        <a
                                            href={link.url}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            className="text-primary hover:underline flex items-center gap-1 max-w-[300px] truncate"
                                        >
                                            {link.url}
                                            <ExternalLink className="h-3 w-3 shrink-0" />
                                        </a>
                                    </TableCell>
                                    <TableCell>
                                        {link.isVisible ? (
                                            <Badge variant="outline" className="bg-green-100 text-green-800">
                                                Visible
                                            </Badge>
                                        ) : (
                                            <Badge variant="outline" className="bg-gray-100 text-gray-800">
                                                Hidden
                                            </Badge>
                                        )}
                                    </TableCell>
                                    <TableCell>
                                        <Button
                                            type="button"
                                            variant="ghost"
                                            size="icon"
                                            onClick={() => handleOpenDialog(link)}
                                            disabled={disabled}
                                        >
                                            <Pencil className="h-4 w-4" />
                                        </Button>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                )}

                <ResponsiveDialog
                    open={dialogOpen}
                    onOpenChange={setDialogOpen}
                    title={editingLink ? 'Edit Link' : 'Add Link'}
                    description={
                        editingLink
                            ? 'Update the details of this profile link'
                            : 'Add a new link to your profile'
                    }
                >
                    <div className="space-y-4 py-4">
                        <div className="space-y-2">
                            <Label htmlFor="label">Label *</Label>
                            <Input
                                id="label"
                                placeholder="e.g., GitHub, LinkedIn, Portfolio"
                                value={formData.label}
                                onChange={(e) => {
                                    setFormData({ ...formData, label: e.target.value });
                                    if (formErrors.label) setFormErrors({ ...formErrors, label: '' });
                                }}
                                disabled={disabled}
                            />
                            {formErrors.label && (
                                <p className="text-sm text-destructive">{formErrors.label}</p>
                            )}
                        </div>

                        <div className="space-y-2">
                            <Label htmlFor="url">URL *</Label>
                            <Input
                                id="url"
                                type="url"
                                placeholder="https://example.com"
                                value={formData.url}
                                onChange={(e) => {
                                    setFormData({ ...formData, url: e.target.value });
                                    if (formErrors.url) setFormErrors({ ...formErrors, url: '' });
                                }}
                                disabled={disabled}
                            />
                            {formErrors.url && (
                                <p className="text-sm text-destructive">{formErrors.url}</p>
                            )}
                        </div>

                        <div className="flex items-center space-x-2">
                            <Switch
                                id="isVisible"
                                checked={formData.isVisible}
                                onCheckedChange={(checked) =>
                                    setFormData({ ...formData, isVisible: checked })
                                }
                                disabled={disabled}
                            />
                            <Label htmlFor="isVisible" className="cursor-pointer">
                                Show on public profile
                            </Label>
                        </div>

                        <div className="flex justify-between pt-4">
                            {editingLink ? (
                                <>
                                    <Button
                                        type="button"
                                        variant="destructive"
                                        onClick={handleDelete}
                                        disabled={disabled}
                                    >
                                        <Trash2 className="h-4 w-4 mr-2" />
                                        Delete
                                    </Button>
                                    <div className="space-x-2">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() => setDialogOpen(false)}
                                        >
                                            Cancel
                                        </Button>
                                        <Button type="button" onClick={handleSave} disabled={disabled}>
                                            Save Changes
                                        </Button>
                                    </div>
                                </>
                            ) : (
                                <>
                                    <div></div>
                                    <div className="space-x-2">
                                        <Button
                                            type="button"
                                            variant="outline"
                                            onClick={() => setDialogOpen(false)}
                                        >
                                            Cancel
                                        </Button>
                                        <Button type="button" onClick={handleSave} disabled={disabled}>
                                            Add Link
                                        </Button>
                                    </div>
                                </>
                            )}
                        </div>
                    </div>
                </ResponsiveDialog>
            </CardContent>
        </Card>
    );
}
