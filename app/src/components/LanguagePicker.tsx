import { useState } from 'react';
import { Check, ChevronsUpDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from '@/components/ui/command';
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from '@/components/ui/popover';

interface Language {
    code: string;
    name: string;
}

interface LanguagePickerProps {
    value?: string;
    onValueChange: (code: string, name: string) => void;
    disabled?: boolean;
    excludeCodes?: string[];
}

// Temporary mock data - will be replaced with API call
const MOCK_LANGUAGES: Language[] = [
    { code: 'eng', name: 'English' },
    { code: 'dan', name: 'Danish' },
    { code: 'nor', name: 'Norwegian' },
    { code: 'swe', name: 'Swedish' },
    { code: 'deu', name: 'German' },
    { code: 'fra', name: 'French' },
    { code: 'spa', name: 'Spanish' },
    { code: 'ita', name: 'Italian' },
    { code: 'nld', name: 'Dutch' },
    { code: 'pol', name: 'Polish' },
    { code: 'rus', name: 'Russian' },
    { code: 'por', name: 'Portuguese' },
    { code: 'jpn', name: 'Japanese' },
    { code: 'zho', name: 'Chinese' },
    { code: 'ara', name: 'Arabic' },
    { code: 'hin', name: 'Hindi' },
];

export function LanguagePicker({
    value,
    onValueChange,
    disabled = false,
    excludeCodes = [],
}: LanguagePickerProps) {
    const [open, setOpen] = useState(false);
    const [searchQuery, setSearchQuery] = useState('');

    const availableLanguages = MOCK_LANGUAGES.filter(
        (lang) => !excludeCodes.includes(lang.code)
    );

    const selectedLanguage = availableLanguages.find((lang) => lang.code === value);

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={open}
                    disabled={disabled}
                    className="w-full justify-between"
                >
                    {selectedLanguage ? selectedLanguage.name : 'Select language...'}
                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-full p-0">
                <Command>
                    <CommandInput
                        placeholder="Search language..."
                        value={searchQuery}
                        onValueChange={setSearchQuery}
                    />
                    <CommandList>
                        <CommandEmpty>No language found.</CommandEmpty>
                        <CommandGroup>
                            {availableLanguages.map((language) => (
                                <CommandItem
                                    key={language.code}
                                    value={language.name}
                                    onSelect={() => {
                                        onValueChange(language.code, language.name);
                                        setOpen(false);
                                        setSearchQuery('');
                                    }}
                                >
                                    <Check
                                        className={cn(
                                            'mr-2 h-4 w-4',
                                            value === language.code ? 'opacity-100' : 'opacity-0'
                                        )}
                                    />
                                    {language.name}
                                </CommandItem>
                            ))}
                        </CommandGroup>
                    </CommandList>
                </Command>
            </PopoverContent>
        </Popover>
    );
}
