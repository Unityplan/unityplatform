# Unity Platform - Development Dashboard

Modern React-based development dashboard for Unity Platform microservices.

## Tech Stack

- **React 19** - Latest React version
- **Vite** - Fast build tool and dev server
- **TypeScript** - Type-safe development
- **Tailwind CSS 4** - Utility-first styling
- **shadcn/ui** - High-quality UI components
- **Radix UI** - Accessible component primitives
- **Lucide React** - Beautiful icons
- **next-themes** - Dark/light mode support

## Features

- **Dashboard View**: Visual service status with cards grouped by category
- **Services Table**: Comprehensive table view with all service information
- **Real-time Monitoring**: Auto-checks service health every 30 seconds
- **Dark Mode**: Full dark/light theme support
- **Responsive**: Works on all screen sizes
- **Type-Safe**: Full TypeScript coverage

## Development

### Install Dependencies

```bash
pnpm install
```

### Run Dev Server

```bash
pnpm dev
```

Dashboard will be available at http://localhost:8888

### Build for Production

```bash
pnpm build
```

### Preview Production Build

```bash
pnpm preview
```

## Service Categories

The dashboard organizes services into five categories:

1. **Application** - Frontend and API Gateway
2. **Backend Services** - Rust microservices (auth, user, etc.)
3. **Infrastructure** - PostgreSQL, Redis, NATS, IPFS
4. **Development Tools** - Adminer, Forgejo, MailHog, etc.
5. **Observability** - Grafana, Prometheus, Jaeger, exporters

## Docker Integration

The dashboard is served via Docker Compose at http://localhost:8888.

Update `docker-compose.dev.yml` to point to the new build output:

```yaml
volumes:
  - ./monitoring/dev-dashboard/dist:/usr/share/nginx/html:ro
```

## Project Structure

```
dev-dashboard/
├── src/
│   ├── components/
│   │   ├── ui/          # shadcn components
│   │   ├── dashboard-view.tsx
│   │   ├── services-table.tsx
│   │   ├── dashboard-tabs.tsx
│   │   ├── theme-provider.tsx
│   │   └── theme-toggle.tsx
│   ├── lib/
│   │   ├── services.ts  # Service definitions
│   │   └── utils.ts     # Utility functions
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── package.json
├── vite.config.ts
├── tsconfig.json
└── components.json     # shadcn configuration
```

## Adding New Services

Edit `src/lib/services.ts` and add your service to the `services` array:

```typescript
{
  name: 'My Service',
  icon: '🚀',
  port: 8080,
  url: 'http://localhost:8080',
  category: 'Backend Services',
  description: 'My awesome service',
  deployed: true,
}
```

## Status Indicators

- 🟢 **Operational** - Service is running and responding
- 🔴 **Down** - Service is not responding
- ⚪ **Not Deployed** - Service not yet implemented
- 🔵 **Checking...** - Status being verified
