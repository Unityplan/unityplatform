import { useState } from 'react';
import { Pencil, Plus, Trash2 } from 'lucide-react';
import type {
    LanguageProficiency,
    ProficiencyLevel,
    CreateLanguageProficiencyRequest,
    UpdateLanguageProficiencyRequest,
} from '@/api/users';
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
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { LanguagePicker } from '@/components/LanguagePicker';

interface LanguageProficiencyManagerProps {
    languages: LanguageProficiency[];
    onAdd: (language: CreateLanguageProficiencyRequest) => Promise<void>;
    onUpdate: (id: string, language: UpdateLanguageProficiencyRequest) => Promise<void>;
    onDelete: (id: string) => Promise<void>;
    disabled?: boolean;
}

const PROFICIENCY_LEVELS: ProficiencyLevel[] = ['none', 'basic', 'intermediate', 'fluent', 'native'];

const PROFICIENCY_COLORS: Record<ProficiencyLevel, string> = {
    none: 'bg-gray-100 text-gray-800',
    basic: 'bg-blue-100 text-blue-800',
    intermediate: 'bg-green-100 text-green-800',
    fluent: 'bg-purple-100 text-purple-800',
    native: 'bg-orange-100 text-orange-800',
};

// Calculate average proficiency score for sorting
const calculateAverage = (lang: LanguageProficiency): number => {
    const levels: Record<ProficiencyLevel, number> = {
        none: 0,
        basic: 1,
        intermediate: 2,
        fluent: 3,
        native: 4,
    };
    return (
        (levels[lang.spokenLevel] +
            levels[lang.writtenLevel] +
            levels[lang.readingLevel] +
            levels[lang.listeningLevel]) /
        4
    );
};

export function LanguageProficiencyManager({
    languages: languagesProp,
    onAdd,
    onUpdate,
    onDelete,
    disabled = false,
}: LanguageProficiencyManagerProps) {
    // Ensure languages is always an array
    const languages = Array.isArray(languagesProp) ? languagesProp : [];

    const [dialogOpen, setDialogOpen] = useState(false);
    const [editingLanguage, setEditingLanguage] = useState<LanguageProficiency | null>(null);
    const [formData, setFormData] = useState({
        languageCode: '',
        languageName: '',
        spokenLevel: 'intermediate' as ProficiencyLevel,
        writtenLevel: 'intermediate' as ProficiencyLevel,
        readingLevel: 'intermediate' as ProficiencyLevel,
        listeningLevel: 'intermediate' as ProficiencyLevel,
    });

    // Sort languages by average proficiency (best first)
    const sortedLanguages = [...languages].sort((a, b) => calculateAverage(b) - calculateAverage(a));

    const handleOpenDialog = (language?: LanguageProficiency) => {
        if (language) {
            setEditingLanguage(language);
            setFormData({
                languageCode: language.languageCode,
                languageName: language.languageName,
                spokenLevel: language.spokenLevel,
                writtenLevel: language.writtenLevel,
                readingLevel: language.readingLevel,
                listeningLevel: language.listeningLevel,
            });
        } else {
            setEditingLanguage(null);
            setFormData({
                languageCode: '',
                languageName: '',
                spokenLevel: 'intermediate',
                writtenLevel: 'intermediate',
                readingLevel: 'intermediate',
                listeningLevel: 'intermediate',
            });
        }
        setDialogOpen(true);
    };

    const handleSave = async () => {
        try {
            if (editingLanguage) {
                await onUpdate(editingLanguage.id, {
                    spokenLevel: formData.spokenLevel,
                    writtenLevel: formData.writtenLevel,
                    readingLevel: formData.readingLevel,
                    listeningLevel: formData.listeningLevel,
                });
            } else {
                await onAdd({
                    languageCode: formData.languageCode,
                    languageName: formData.languageName,
                    spokenLevel: formData.spokenLevel,
                    writtenLevel: formData.writtenLevel,
                    readingLevel: formData.readingLevel,
                    listeningLevel: formData.listeningLevel,
                    displayOrder: languages.length,
                    isPreferred: false,
                    showOnProfile: true,
                });
            }
            setDialogOpen(false);
        } catch (err) {
            console.error('Failed to save language:', err);
        }
    };

    const handleDelete = async () => {
        if (editingLanguage) {
            try {
                await onDelete(editingLanguage.id);
                setDialogOpen(false);
            } catch (err) {
                console.error('Failed to delete language:', err);
            }
        }
    };

    const usedLanguageCodes = languages.map((l) => l.languageCode);

    return (
        <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
                <div>
                    <CardTitle>Language Proficiencies</CardTitle>
                    <CardDescription>
                        Manage your language skills across speaking, writing, reading, and listening
                    </CardDescription>
                </div>
                <Button
                    type="button"
                    size="sm"
                    onClick={() => handleOpenDialog()}
                    disabled={disabled}
                >
                    <Plus className="h-4 w-4 mr-2" />
                    Add Language
                </Button>
            </CardHeader>
            <CardContent>
                {sortedLanguages.length === 0 ? (
                    <div className="text-center py-8 text-muted-foreground">
                        No languages added yet. Click "Add Language" to get started.
                    </div>
                ) : (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Language</TableHead>
                                <TableHead className="text-center">Speaking</TableHead>
                                <TableHead className="text-center">Writing</TableHead>
                                <TableHead className="text-center">Reading</TableHead>
                                <TableHead className="text-center">Listening</TableHead>
                                <TableHead className="w-[50px]"></TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {sortedLanguages.map((language) => (
                                <TableRow key={language.id}>
                                    <TableCell className="font-medium">
                                        {language.languageName}
                                        {language.isPreferred && (
                                            <Badge variant="secondary" className="ml-2 text-xs">
                                                Preferred
                                            </Badge>
                                        )}
                                    </TableCell>
                                    <TableCell className="text-center">
                                        <Badge
                                            variant="outline"
                                            className={PROFICIENCY_COLORS[language.spokenLevel]}
                                        >
                                            {language.spokenLevel}
                                        </Badge>
                                    </TableCell>
                                    <TableCell className="text-center">
                                        <Badge
                                            variant="outline"
                                            className={PROFICIENCY_COLORS[language.writtenLevel]}
                                        >
                                            {language.writtenLevel}
                                        </Badge>
                                    </TableCell>
                                    <TableCell className="text-center">
                                        <Badge
                                            variant="outline"
                                            className={PROFICIENCY_COLORS[language.readingLevel]}
                                        >
                                            {language.readingLevel}
                                        </Badge>
                                    </TableCell>
                                    <TableCell className="text-center">
                                        <Badge
                                            variant="outline"
                                            className={PROFICIENCY_COLORS[language.listeningLevel]}
                                        >
                                            {language.listeningLevel}
                                        </Badge>
                                    </TableCell>
                                    <TableCell>
                                        <Button
                                            type="button"
                                            variant="ghost"
                                            size="icon"
                                            onClick={() => handleOpenDialog(language)}
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
                    title={editingLanguage ? 'Edit Language Proficiency' : 'Add Language'}
                    description={
                        editingLanguage
                            ? 'Update your proficiency levels for this language'
                            : 'Select a language and set your proficiency levels'
                    }
                >
                    <div className="space-y-4 py-4">
                        {!editingLanguage && (
                            <div className="space-y-2">
                                <Label>Language</Label>
                                <LanguagePicker
                                    value={formData.languageCode}
                                    onValueChange={(code, name) => {
                                        setFormData({ ...formData, languageCode: code, languageName: name });
                                    }}
                                    excludeCodes={usedLanguageCodes}
                                    disabled={disabled}
                                />
                            </div>
                        )}

                        <div className="grid grid-cols-2 gap-4">
                            <div className="space-y-2">
                                <Label htmlFor="spoken">Speaking</Label>
                                <Select
                                    value={formData.spokenLevel}
                                    onValueChange={(value) =>
                                        setFormData({ ...formData, spokenLevel: value as ProficiencyLevel })
                                    }
                                    disabled={disabled}
                                >
                                    <SelectTrigger id="spoken">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {PROFICIENCY_LEVELS.map((level) => (
                                            <SelectItem key={level} value={level} className="capitalize">
                                                {level}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="written">Writing</Label>
                                <Select
                                    value={formData.writtenLevel}
                                    onValueChange={(value) =>
                                        setFormData({ ...formData, writtenLevel: value as ProficiencyLevel })
                                    }
                                    disabled={disabled}
                                >
                                    <SelectTrigger id="written">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {PROFICIENCY_LEVELS.map((level) => (
                                            <SelectItem key={level} value={level} className="capitalize">
                                                {level}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="reading">Reading</Label>
                                <Select
                                    value={formData.readingLevel}
                                    onValueChange={(value) =>
                                        setFormData({ ...formData, readingLevel: value as ProficiencyLevel })
                                    }
                                    disabled={disabled}
                                >
                                    <SelectTrigger id="reading">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {PROFICIENCY_LEVELS.map((level) => (
                                            <SelectItem key={level} value={level} className="capitalize">
                                                {level}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                            </div>

                            <div className="space-y-2">
                                <Label htmlFor="listening">Listening</Label>
                                <Select
                                    value={formData.listeningLevel}
                                    onValueChange={(value) =>
                                        setFormData({ ...formData, listeningLevel: value as ProficiencyLevel })
                                    }
                                    disabled={disabled}
                                >
                                    <SelectTrigger id="listening">
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {PROFICIENCY_LEVELS.map((level) => (
                                            <SelectItem key={level} value={level} className="capitalize">
                                                {level}
                                            </SelectItem>
                                        ))}
                                    </SelectContent>
                                </Select>
                            </div>
                        </div>

                        <div className="flex justify-between pt-4">
                            {editingLanguage ? (
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
                                        <Button
                                            type="button"
                                            onClick={handleSave}
                                            disabled={disabled}
                                        >
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
                                        <Button
                                            type="button"
                                            onClick={handleSave}
                                            disabled={disabled || !formData.languageCode}
                                        >
                                            Add Language
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
