export type ServiceCategory = 
  | 'Application'
  | 'Backend Services'
  | 'Infrastructure'
  | 'Development Tools'
  | 'Observability';

export type ServiceStatus = 
  | 'operational'
  | 'checking'
  | 'down'
  | 'not-deployed';

export interface Service {
  name: string;
  icon: string;
  port: number | string;
  url: string;
  category: ServiceCategory;
  description: string;
  critical?: boolean;
  deployed: boolean;
  status?: ServiceStatus;
}

export const services: Service[] = [
  // Application
  {
    name: 'Frontend',
    icon: '⚛️',
    port: 5173,
    url: 'http://localhost:5173',
    category: 'Application',
    description: 'React + Vite development server',
    deployed: true,
  },
  {
    name: 'API Gateway',
    icon: '🔌',
    port: 8000,
    url: 'http://localhost:8000',
    category: 'Application',
    description: 'Main API endpoint for all services',
    deployed: false,
  },

  // Backend Services
  {
    name: 'Auth Service',
    icon: '🔐',
    port: 8001,
    url: 'http://localhost:8001',
    category: 'Backend Services',
    description: 'Authentication & JWT management',
    deployed: true,
  },
  {
    name: 'User Service',
    icon: '👤',
    port: 8002,
    url: 'http://localhost:8002',
    category: 'Backend Services',
    description: 'Profiles, connections, GDPR, settings',
    deployed: true,
  },
  {
    name: 'Event Service',
    icon: '📅',
    port: 8003,
    url: 'http://localhost:8003',
    category: 'Backend Services',
    description: 'Events & calendar',
    deployed: false,
  },
  {
    name: 'Invitation Service',
    icon: '✉️',
    port: 8004,
    url: 'http://localhost:8004',
    category: 'Backend Services',
    description: 'Invitation management & trust graph',
    deployed: false,
  },
  {
    name: 'Notification Service',
    icon: '🔔',
    port: 8005,
    url: 'http://localhost:8005',
    category: 'Backend Services',
    description: 'Notifications & email',
    deployed: false,
  },
  {
    name: 'Community Service',
    icon: '👥',
    port: 8006,
    url: 'http://localhost:8006',
    category: 'Backend Services',
    description: 'Communities & membership',
    deployed: false,
  },
  {
    name: 'Badge Service',
    icon: '🏆',
    port: 8007,
    url: 'http://localhost:8007',
    category: 'Backend Services',
    description: 'Gamification & achievements',
    deployed: false,
  },
  {
    name: 'Territory Service',
    icon: '🌍',
    port: 8008,
    url: 'http://localhost:8008',
    category: 'Backend Services',
    description: 'Pod management & federation',
    deployed: false,
  },

  // Infrastructure
  {
    name: 'PostgreSQL',
    icon: '🐘',
    port: 5432,
    url: 'http://localhost:8080',
    category: 'Infrastructure',
    description: 'Database server',
    critical: true,
    deployed: true,
  },
  {
    name: 'Redis',
    icon: '💾',
    port: 6379,
    url: 'http://localhost:8082',
    category: 'Infrastructure',
    description: 'Cache and session store',
    critical: true,
    deployed: true,
  },
  {
    name: 'NATS',
    icon: '📨',
    port: 4222,
    url: 'http://localhost:8222/varz',
    category: 'Infrastructure',
    description: 'Message bus (client connections)',
    critical: true,
    deployed: true,
  },
  {
    name: 'IPFS',
    icon: '🌐',
    port: 5001,
    url: 'http://localhost:8081/webui',
    category: 'Infrastructure',
    description: 'Decentralized storage',
    deployed: true,
  },

  // Development Tools
  {
    name: 'Adminer',
    icon: '🗄️',
    port: 8080,
    url: 'http://localhost:8080',
    category: 'Development Tools',
    description: 'PostgreSQL database management',
    deployed: true,
  },
  {
    name: 'Redis Commander',
    icon: '⚡',
    port: 8082,
    url: 'http://localhost:8082',
    category: 'Development Tools',
    description: 'Redis cache management UI',
    deployed: true,
  },
  {
    name: 'Forgejo',
    icon: '🦊',
    port: 3000,
    url: 'http://localhost:3000',
    category: 'Development Tools',
    description: 'Self-hosted Git with MCP integration',
    deployed: true,
  },
  {
    name: 'Docker Registry',
    icon: '🐳',
    port: 5000,
    url: 'http://localhost:5000/v2/_catalog',
    category: 'Development Tools',
    description: 'Local container image storage',
    deployed: true,
  },
  {
    name: 'MailHog',
    icon: '📧',
    port: '1025 / 8025',
    url: 'http://localhost:8025',
    category: 'Development Tools',
    description: 'Email testing & debugging',
    deployed: true,
  },
  {
    name: 'Traefik',
    icon: '🔀',
    port: 8083,
    url: 'http://localhost:8083/dashboard/',
    category: 'Development Tools',
    description: 'Reverse proxy & routing dashboard',
    deployed: true,
  },

  // Observability
  {
    name: 'Grafana',
    icon: '📊',
    port: 3001,
    url: 'http://localhost:3001',
    category: 'Observability',
    description: 'Metrics dashboards & visualization',
    deployed: true,
  },
  {
    name: 'Prometheus',
    icon: '📈',
    port: 9090,
    url: 'http://localhost:9090',
    category: 'Observability',
    description: 'Metrics collection & querying',
    deployed: true,
  },
  {
    name: 'Jaeger',
    icon: '🔍',
    port: 16686,
    url: 'http://localhost:16686',
    category: 'Observability',
    description: 'Distributed tracing & APM',
    deployed: true,
  },
  {
    name: 'PostgreSQL Exporter',
    icon: '🐘',
    port: 9187,
    url: 'http://localhost:9187/metrics',
    category: 'Observability',
    description: 'Database metrics for Prometheus',
    deployed: true,
  },
  {
    name: 'Redis Exporter',
    icon: '💾',
    port: 9121,
    url: 'http://localhost:9121/metrics',
    category: 'Observability',
    description: 'Cache metrics for Prometheus',
    deployed: true,
  },
  {
    name: 'NATS Exporter',
    icon: '📨',
    port: 7777,
    url: 'http://localhost:7777/metrics',
    category: 'Observability',
    description: 'Message bus metrics for Prometheus',
    deployed: true,
  },
  {
    name: 'Node Exporter',
    icon: '🖥️',
    port: 9100,
    url: 'http://localhost:9100/metrics',
    category: 'Observability',
    description: 'Host system metrics (CPU, RAM, Disk)',
    deployed: true,
  },
  {
    name: 'cAdvisor',
    icon: '🐳',
    port: 8089,
    url: 'http://localhost:8089',
    category: 'Observability',
    description: 'Container resource monitoring',
    deployed: true,
  },
];
