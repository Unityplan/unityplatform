import { useState } from 'react';
import type { KeyboardEvent } from 'react';
import { X } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';

interface TagInputProps {
    value: string[];
    onChange: (tags: string[]) => void;
    placeholder?: string;
    disabled?: boolean;
    maxTags?: number;
}

export function TagInput({
    value = [],
    onChange,
    placeholder = 'Type and press Enter to add',
    disabled = false,
    maxTags,
}: TagInputProps) {
    const [inputValue, setInputValue] = useState('');

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            addTag();
        } else if (e.key === 'Backspace' && inputValue === '' && value.length > 0) {
            // Remove last tag on backspace when input is empty
            removeTag(value.length - 1);
        }
    };

    const addTag = () => {
        const trimmedValue = inputValue.trim();
        if (trimmedValue === '') return;
        if (value.includes(trimmedValue)) {
            setInputValue('');
            return;
        }
        if (maxTags && value.length >= maxTags) {
            return;
        }

        onChange([...value, trimmedValue]);
        setInputValue('');
    };

    const removeTag = (index: number) => {
        onChange(value.filter((_, i) => i !== index));
    };

    return (
        <div className="flex flex-wrap gap-2 p-2 border rounded-md bg-background focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2">
            {value.map((tag, index) => (
                <Badge
                    key={index}
                    variant="default"
                    className="pl-2 pr-1 py-1 gap-1"
                >
                    <span>{tag}</span>
                    <button
                        type="button"
                        onClick={() => removeTag(index)}
                        disabled={disabled}
                        className="ml-1 rounded-full hover:bg-primary-foreground/20 p-0.5"
                    >
                        <X className="h-3 w-3" />
                    </button>
                </Badge>
            ))}
            <Input
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={handleKeyDown}
                onBlur={addTag}
                placeholder={value.length === 0 ? placeholder : ''}
                disabled={disabled}
                className="flex-1 min-w-[120px] border-0 focus-visible:ring-0 focus-visible:ring-offset-0 shadow-none px-0"
            />
        </div>
    );
}
