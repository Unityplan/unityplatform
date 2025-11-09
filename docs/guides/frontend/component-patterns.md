# Component Patterns Guide

Best practices and patterns for building React components in UnityPlan.

**Stack:** React 18, shadcn/ui, TailwindCSS  
**Last Updated:** November 9, 2025

---

## 📋 Table of Contents

- [Component Types](#component-types)
- [Composition Patterns](#composition-patterns)
- [Form Components](#form-components)
- [Data Display Components](#data-display-components)
- [Layout Components](#layout-components)
- [Reusable Patterns](#reusable-patterns)
- [Performance Patterns](#performance-patterns)
- [Accessibility](#accessibility)

---

## Component Types

### Presentation Components (Dumb/Stateless)

**Purpose:** Display data, emit events, no business logic

```tsx
// ✅ Good: Pure presentation component
interface UserCardProps {
  user: User;
  onEdit?: () => void;
}

export function UserCard({ user, onEdit }: UserCardProps) {
  return (
    <Card>
      <CardHeader>
        <div className="flex items-center gap-4">
          <Avatar>
            <AvatarImage src={user.avatar_url} />
            <AvatarFallback>{user.username[0].toUpperCase()}</AvatarFallback>
          </Avatar>
          <div>
            <CardTitle>{user.username}</CardTitle>
            <p className="text-sm text-muted-foreground">{user.email}</p>
          </div>
        </div>
      </CardHeader>
      {onEdit && (
        <CardFooter>
          <Button onClick={onEdit} variant="outline">
            Edit Profile
          </Button>
        </CardFooter>
      )}
    </Card>
  );
}
```

### Container Components (Smart/Stateful)

**Purpose:** Fetch data, manage state, handle logic

```tsx
// ✅ Good: Container handles data fetching and state
export function UserProfileContainer({ userId }: { userId: string }) {
  const { data: user, isLoading, error } = useUser(userId);
  const navigate = useNavigate();

  if (isLoading) return <Skeleton className="h-32 w-full" />;
  if (error) return <ErrorMessage error={error} />;
  if (!user) return <NotFound />;

  return <UserCard user={user} onEdit={() => navigate(`/profile/edit`)} />;
}
```

### Compound Components

**Purpose:** Related components that work together

```tsx
// ✅ Compound component pattern
interface ProfileProps {
  children: React.ReactNode;
}

export function Profile({ children }: ProfileProps) {
  return <div className="profile-container">{children}</div>;
}

Profile.Header = function ProfileHeader({ user }: { user: User }) {
  return (
    <div className="flex items-center gap-4">
      <Avatar>
        <AvatarImage src={user.avatar_url} />
        <AvatarFallback>{user.username[0]}</AvatarFallback>
      </Avatar>
      <div>
        <h2>{user.username}</h2>
        <p>{user.email}</p>
      </div>
    </div>
  );
};

Profile.Stats = function ProfileStats({ stats }: { stats: UserStats }) {
  return (
    <div className="grid grid-cols-3 gap-4">
      <div>
        <p className="text-2xl font-bold">{stats.posts}</p>
        <p className="text-sm text-muted-foreground">Posts</p>
      </div>
      {/* More stats */}
    </div>
  );
};

// Usage
<Profile>
  <Profile.Header user={user} />
  <Profile.Stats stats={stats} />
</Profile>
```

---

## Composition Patterns

### Render Props

**Purpose:** Share logic between components

```tsx
interface DataFetcherProps<T> {
  url: string;
  children: (data: T, isLoading: boolean, error: Error | null) => React.ReactNode;
}

function DataFetcher<T>({ url, children }: DataFetcherProps<T>) {
  const [data, setData] = useState<T | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    fetch(url)
      .then(res => res.json())
      .then(setData)
      .catch(setError)
      .finally(() => setIsLoading(false));
  }, [url]);

  return <>{children(data, isLoading, error)}</>;
}

// Usage
<DataFetcher<User> url="/api/users/1">
  {(user, isLoading, error) => {
    if (isLoading) return <Spinner />;
    if (error) return <ErrorMessage error={error} />;
    return <UserCard user={user} />;
  }}
</DataFetcher>
```

### Custom Hooks (Preferred over Render Props)

**Purpose:** Reusable logic with cleaner syntax

```tsx
// ✅ Better: Custom hook
function useDataFetch<T>(url: string) {
  const [data, setData] = useState<T | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    fetch(url)
      .then(res => res.json())
      .then(setData)
      .catch(setError)
      .finally(() => setIsLoading(false));
  }, [url]);

  return { data, isLoading, error };
}

// Usage - much cleaner!
function UserProfile({ userId }: { userId: string }) {
  const { data: user, isLoading, error } = useDataFetch<User>(`/api/users/${userId}`);
  
  if (isLoading) return <Spinner />;
  if (error) return <ErrorMessage error={error} />;
  return <UserCard user={user} />;
}
```

### Higher-Order Components (HOCs)

**Purpose:** Wrap components with additional behavior

```tsx
// HOC for authentication
function withAuth<P extends object>(Component: React.ComponentType<P>) {
  return function AuthenticatedComponent(props: P) {
    const { user, isLoading } = useAuthStore();
    const navigate = useNavigate();

    useEffect(() => {
      if (!isLoading && !user) {
        navigate('/login');
      }
    }, [user, isLoading, navigate]);

    if (isLoading) return <Spinner />;
    if (!user) return null;

    return <Component {...props} />;
  };
}

// Usage
const ProtectedProfile = withAuth(ProfilePage);
```

---

## Form Components

### Basic Form with react-hook-form + zod

```tsx
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

const loginSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
});

type LoginFormData = z.infer<typeof loginSchema>;

export function LoginForm({ onSubmit }: { onSubmit: (data: LoginFormData) => void }) {
  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
  });

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div>
        <Label htmlFor="email">Email</Label>
        <Input
          id="email"
          type="email"
          {...register('email')}
          aria-invalid={errors.email ? 'true' : 'false'}
        />
        {errors.email && (
          <p className="text-sm text-destructive mt-1">{errors.email.message}</p>
        )}
      </div>

      <div>
        <Label htmlFor="password">Password</Label>
        <Input
          id="password"
          type="password"
          {...register('password')}
          aria-invalid={errors.password ? 'true' : 'false'}
        />
        {errors.password && (
          <p className="text-sm text-destructive mt-1">{errors.password.message}</p>
        )}
      </div>

      <Button type="submit" className="w-full">
        Sign In
      </Button>
    </form>
  );
}
```

### Form with shadcn Form Components

```tsx
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';

export function ProfileEditForm() {
  const form = useForm<ProfileFormData>({
    resolver: zodResolver(profileSchema),
    defaultValues: {
      username: '',
      email: '',
      bio: '',
    },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input placeholder="johndoe" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="bio"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Bio</FormLabel>
              <FormControl>
                <Textarea
                  placeholder="Tell us about yourself"
                  className="resize-none"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button type="submit">Save Changes</Button>
      </form>
    </Form>
  );
}
```

### File Upload Component

```tsx
interface AvatarUploadProps {
  currentAvatarUrl?: string;
  onUpload: (file: File) => void;
}

export function AvatarUpload({ currentAvatarUrl, onUpload }: AvatarUploadProps) {
  const [preview, setPreview] = useState<string | undefined>(currentAvatarUrl);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    // Validate file type
    if (!file.type.startsWith('image/')) {
      toast.error('Please upload an image file');
      return;
    }

    // Validate file size (max 5MB)
    if (file.size > 5 * 1024 * 1024) {
      toast.error('File size must be less than 5MB');
      return;
    }

    // Create preview
    const reader = new FileReader();
    reader.onloadend = () => {
      setPreview(reader.result as string);
    };
    reader.readAsDataURL(file);

    // Call upload handler
    onUpload(file);
  };

  return (
    <div className="flex items-center gap-4">
      <Avatar className="h-24 w-24">
        <AvatarImage src={preview} />
        <AvatarFallback>Upload</AvatarFallback>
      </Avatar>
      <div>
        <input
          ref={fileInputRef}
          type="file"
          accept="image/*"
          onChange={handleFileChange}
          className="hidden"
        />
        <Button
          type="button"
          variant="outline"
          onClick={() => fileInputRef.current?.click()}
        >
          Upload Avatar
        </Button>
        <p className="text-xs text-muted-foreground mt-2">
          JPG, PNG or GIF (max 5MB)
        </p>
      </div>
    </div>
  );
}
```

---

## Data Display Components

### Data Table Pattern

```tsx
interface Column<T> {
  header: string;
  accessor: (row: T) => React.ReactNode;
  sortable?: boolean;
}

interface DataTableProps<T> {
  data: T[];
  columns: Column<T>[];
  isLoading?: boolean;
}

export function DataTable<T>({ data, columns, isLoading }: DataTableProps<T>) {
  if (isLoading) {
    return <Skeleton className="h-96 w-full" />;
  }

  return (
    <div className="rounded-md border">
      <table className="w-full">
        <thead>
          <tr className="border-b bg-muted/50">
            {columns.map((column, i) => (
              <th key={i} className="px-4 py-3 text-left font-medium">
                {column.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.map((row, i) => (
            <tr key={i} className="border-b hover:bg-muted/50">
              {columns.map((column, j) => (
                <td key={j} className="px-4 py-3">
                  {column.accessor(row)}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

// Usage
const userColumns: Column<User>[] = [
  {
    header: 'Name',
    accessor: (user) => user.username,
  },
  {
    header: 'Email',
    accessor: (user) => user.email,
  },
  {
    header: 'Actions',
    accessor: (user) => (
      <Button variant="ghost" size="sm">
        Edit
      </Button>
    ),
  },
];

<DataTable data={users} columns={userColumns} />
```

### Infinite Scroll List

```tsx
export function InfiniteUserList() {
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteQuery({
    queryKey: ['users'],
    queryFn: ({ pageParam = 1 }) => fetchUsers(pageParam),
    getNextPageParam: (lastPage) => lastPage.nextPage,
  });

  const observerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasNextPage && !isFetchingNextPage) {
          fetchNextPage();
        }
      },
      { threshold: 1.0 }
    );

    if (observerRef.current) {
      observer.observe(observerRef.current);
    }

    return () => observer.disconnect();
  }, [hasNextPage, isFetchingNextPage, fetchNextPage]);

  const users = data?.pages.flatMap((page) => page.users) ?? [];

  return (
    <div className="space-y-4">
      {users.map((user) => (
        <UserCard key={user.id} user={user} />
      ))}
      
      <div ref={observerRef} className="h-10 flex items-center justify-center">
        {isFetchingNextPage && <Spinner />}
      </div>
    </div>
  );
}
```

---

## Layout Components

### Responsive Grid Layout

```tsx
export function DashboardGrid({ children }: { children: React.ReactNode }) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {children}
    </div>
  );
}

// Usage
<DashboardGrid>
  <StatCard title="Users" value={1234} />
  <StatCard title="Posts" value={5678} />
  <StatCard title="Comments" value={9012} />
</DashboardGrid>
```

### Sidebar Layout

```tsx
export function AppLayout({ children }: { children: React.ReactNode }) {
  const { isSidebarOpen } = useUIStore();

  return (
    <div className="flex h-screen">
      {/* Sidebar */}
      <aside
        className={cn(
          'w-64 border-r bg-muted/30 transition-transform',
          !isSidebarOpen && '-translate-x-full lg:translate-x-0'
        )}
      >
        <Sidebar />
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto">
        <div className="container py-6">
          {children}
        </div>
      </main>
    </div>
  );
}
```

---

## Reusable Patterns

### Loading States

```tsx
export function LoadingState({ size = 'default' }: { size?: 'sm' | 'default' | 'lg' }) {
  const sizeClasses = {
    sm: 'h-4 w-4',
    default: 'h-8 w-8',
    lg: 'h-12 w-12',
  };

  return (
    <div className="flex items-center justify-center p-8">
      <div className={cn('animate-spin rounded-full border-2 border-primary border-t-transparent', sizeClasses[size])} />
    </div>
  );
}
```

### Error Boundary

```tsx
interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends React.Component<
  { children: React.ReactNode; fallback?: React.ReactNode },
  ErrorBoundaryState
> {
  state: ErrorBoundaryState = { hasError: false };

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('Error caught by boundary:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="p-8 text-center">
          <h2 className="text-2xl font-bold mb-4">Something went wrong</h2>
          <p className="text-muted-foreground">{this.state.error?.message}</p>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### Empty States

```tsx
interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}

export function EmptyState({ icon, title, description, action }: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center p-12 text-center">
      {icon && <div className="mb-4 text-muted-foreground">{icon}</div>}
      <h3 className="text-lg font-semibold mb-2">{title}</h3>
      {description && (
        <p className="text-sm text-muted-foreground mb-4 max-w-sm">
          {description}
        </p>
      )}
      {action && (
        <Button onClick={action.onClick}>
          {action.label}
        </Button>
      )}
    </div>
  );
}
```

---

## Performance Patterns

### Memoization

```tsx
// ✅ Memoize expensive components
const UserCard = React.memo(({ user }: { user: User }) => {
  return (
    <Card>
      <CardHeader>{user.username}</CardHeader>
    </Card>
  );
});

// ✅ Memoize expensive calculations
function UserList({ users }: { users: User[] }) {
  const sortedUsers = useMemo(
    () => users.sort((a, b) => a.username.localeCompare(b.username)),
    [users]
  );

  return <>{sortedUsers.map(user => <UserCard key={user.id} user={user} />)}</>;
}

// ✅ Memoize callbacks
function ParentComponent() {
  const handleClick = useCallback(() => {
    console.log('Clicked!');
  }, []);

  return <ChildComponent onClick={handleClick} />;
}
```

### Code Splitting

```tsx
import { lazy, Suspense } from 'react';

// Lazy load heavy components
const HeavyChart = lazy(() => import('./HeavyChart'));

export function Dashboard() {
  return (
    <div>
      <h1>Dashboard</h1>
      <Suspense fallback={<Skeleton className="h-96" />}>
        <HeavyChart />
      </Suspense>
    </div>
  );
}
```

---

## Accessibility

### Semantic HTML

```tsx
// ✅ Good: Semantic elements
export function Article({ title, content }: { title: string; content: string }) {
  return (
    <article>
      <header>
        <h1>{title}</h1>
      </header>
      <section>
        <p>{content}</p>
      </section>
    </article>
  );
}

// ❌ Bad: Div soup
export function Article({ title, content }: { title: string; content: string }) {
  return (
    <div>
      <div>{title}</div>
      <div>{content}</div>
    </div>
  );
}
```

### ARIA Attributes

```tsx
// ✅ Proper ARIA labels
<button
  aria-label="Close modal"
  onClick={onClose}
>
  <X className="h-4 w-4" />
</button>

// ✅ ARIA live regions
<div role="alert" aria-live="polite">
  {errorMessage}
</div>

// ✅ ARIA expanded state
<button
  aria-expanded={isOpen}
  aria-controls="menu"
  onClick={toggle}
>
  Menu
</button>
```

### Keyboard Navigation

```tsx
export function Modal({ isOpen, onClose, children }: ModalProps) {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      return () => document.removeEventListener('keydown', handleEscape);
    }
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div role="dialog" aria-modal="true">
      {children}
    </div>
  );
}
```

---

## Related Guides

- [Development Guide](./development-guide.md) - Setup and workflow
- [Testing Guide](./testing-guide.md) - Testing patterns
- [State Management](./state-management.md) - Data flow

---

**Remember:** Keep components focused, compose them together, and always think about accessibility!
