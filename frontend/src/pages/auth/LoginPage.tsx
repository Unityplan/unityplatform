import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useAuthStore } from '@/stores/authStore';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select';
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { CenteredLayout } from '@/components/layouts/CenteredLayout';
import { Logo } from '@/components/ui/logo';

// Login form validation schema
const loginSchema = z.object({
    username: z.string().min(3, 'Username is required (3-50 characters)'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
    territory_code: z.string().min(2, 'Please select a territory'),
});

type LoginFormValues = z.infer<typeof loginSchema>;

// Available territories
const TERRITORIES = [
    { code: 'dk', name: 'Denmark' },
    { code: 'no', name: 'Norway' },
    { code: 'se', name: 'Sweden' },
    { code: 'eu', name: 'Europe' },
];

export function LoginPage() {
    const { login, isLoading, error } = useAuthStore();
    const [showPassword, setShowPassword] = useState(false);

    const form = useForm<LoginFormValues>({
        resolver: zodResolver(loginSchema),
        defaultValues: {
            username: '',
            password: '',
            territory_code: 'dk', // Default to Denmark
        },
    });

    const onSubmit = async (data: LoginFormValues) => {
        try {
            await login(data);
            // TODO: Redirect to dashboard once routing is set up
            console.log('Login successful!');
        } catch (err) {
            // Error is already handled by the store
            console.error('Login failed:', err);
        }
    };

    return (
        <CenteredLayout>
            <div className="mb-8 flex justify-center">
                <Logo className="text-gray-950 dark:text-white" />
            </div>
            <Card className="w-full max-w-md">
                <CardHeader>
                    <CardTitle>Welcome Back</CardTitle>
                    <CardDescription>Sign in to your UnityPlan account</CardDescription>
                </CardHeader>
                <CardContent>
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                            {/* Territory Selection */}
                            <FormField
                                control={form.control}
                                name="territory_code"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Territory</FormLabel>
                                        <Select onValueChange={field.onChange} defaultValue={field.value}>
                                            <FormControl>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Select your territory" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                {TERRITORIES.map((territory) => (
                                                    <SelectItem key={territory.code} value={territory.code}>
                                                        {territory.name}
                                                    </SelectItem>
                                                ))}
                                            </SelectContent>
                                        </Select>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            {/* Username Field */}
                            <FormField
                                control={form.control}
                                name="username"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Username</FormLabel>
                                        <FormControl>
                                            <Input
                                                type="text"
                                                placeholder="Enter your username"
                                                autoComplete="username"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            {/* Password Field */}
                            <FormField
                                control={form.control}
                                name="password"
                                render={({ field }) => (
                                    <FormItem>
                                        <div className="flex items-center justify-between">
                                            <FormLabel>Password</FormLabel>
                                            <button
                                                type="button"
                                                onClick={() => setShowPassword(!showPassword)}
                                                className="text-sm text-muted-foreground hover:text-primary"
                                            >
                                                {showPassword ? 'Hide' : 'Show'}
                                            </button>
                                        </div>
                                        <FormControl>
                                            <Input
                                                type={showPassword ? 'text' : 'password'}
                                                placeholder="Enter your password"
                                                autoComplete="current-password"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            {/* Error Message */}
                            {error && (
                                <div className="rounded-md bg-destructive/15 p-3 text-sm text-destructive">
                                    {error}
                                </div>
                            )}

                            {/* Submit Button */}
                            <Button type="submit" className="w-full" disabled={isLoading}>
                                {isLoading ? 'Signing in...' : 'Sign In'}
                            </Button>
                        </form>
                    </Form>
                </CardContent>
                <CardFooter className="flex flex-col space-y-2">
                    <div className="text-sm text-muted-foreground">
                        <a href="/reset-password" className="hover:text-primary hover:underline">
                            Forgot your password?
                        </a>
                    </div>
                </CardFooter>
            </Card>
        </CenteredLayout>
    );
}
