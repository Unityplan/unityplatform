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

export interface ResourceStats {
  cpu_usage_percent?: number;
  memory_usage_mb?: number;
  memory_limit_mb?: number;
  memory_percent?: number;
  network_rx_mb?: number;
  network_tx_mb?: number;
  block_read_mb?: number;
  block_write_mb?: number;
}

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
  resources?: ResourceStats;
}

// Fetch services from monitoring backend
export async function fetchServicesStatus(): Promise<Service[]> {
  try {
    const response = await fetch('http://localhost:8090/api/v1/services');
    if (!response.ok) {
      throw new Error('Failed to fetch services');
    }
    
    const data = await response.json();
    
    // Map backend response to our Service interface with icons and categories
    return mapServicesToUI(data.services);
  } catch (error) {
    console.error('Error fetching services:', error);
    return [];
  }
}

// Map backend services to UI format with icons and categories
interface BackendService {
  name: string;
  status: string;
  container_state?: string;
  health?: string;
  uptime?: string;
  resources?: ResourceStats;
}

function mapServicesToUI(backendServices: BackendService[]): Service[] {
  const serviceConfig: Record<string, Partial<Service>> = {
    'Frontend': { icon: '⚛️', port: 5173, url: 'http://localhost:5173', category: 'Application', description: 'React + Vite development server' },
    'Auth Service': { icon: '🔐', port: 8001, url: 'http://localhost:8001/api/v1/health', category: 'Backend Services', description: 'Authentication & JWT management' },
    'User Service': { icon: '👤', port: 8002, url: 'http://localhost:8002/api/v1/health', category: 'Backend Services', description: 'Profiles, connections, GDPR, settings' },
    
    'PostgreSQL': { icon: '🐘', port: 5432, url: 'http://localhost:8080', category: 'Infrastructure', description: 'Database server', critical: true },
    'Redis': { icon: '💾', port: 6379, url: 'http://localhost:8082', category: 'Infrastructure', description: 'Cache and session store', critical: true },
    'NATS': { icon: '📨', port: 4222, url: 'http://localhost:8222/varz', category: 'Infrastructure', description: 'Message bus', critical: true },
    'IPFS': { icon: '🌐', port: 5001, url: 'http://localhost:8081/webui', category: 'Infrastructure', description: 'Decentralized storage' },
    
    'Adminer': { icon: '🗄️', port: 8080, url: 'http://localhost:8080', category: 'Development Tools', description: 'PostgreSQL database management' },
    'Redis Commander': { icon: '⚡', port: 8082, url: 'http://localhost:8082', category: 'Development Tools', description: 'Redis cache management UI' },
    'Forgejo': { icon: '🦊', port: 3000, url: 'http://localhost:3000', category: 'Development Tools', description: 'Self-hosted Git with MCP integration' },
    'Docker Registry': { icon: '🐳', port: 5000, url: 'http://localhost:5000/v2/_catalog', category: 'Development Tools', description: 'Local container image storage' },
    'MailHog': { icon: '📧', port: '1025 / 8025', url: 'http://localhost:8025', category: 'Development Tools', description: 'Email testing & debugging' },
    'Traefik': { icon: '🔀', port: 8083, url: 'http://localhost:8083/dashboard/', category: 'Development Tools', description: 'Reverse proxy & routing dashboard' },
    
    'Grafana': { icon: '📊', port: 3001, url: 'http://localhost:3001', category: 'Observability', description: 'Metrics dashboards & visualization' },
    'Prometheus': { icon: '📈', port: 9090, url: 'http://localhost:9090', category: 'Observability', description: 'Metrics collection & querying' },
    'Jaeger': { icon: '🔍', port: 16686, url: 'http://localhost:16686', category: 'Observability', description: 'Distributed tracing & APM' },
    'PostgreSQL Exporter': { icon: '🐘', port: 9187, url: 'http://localhost:9187/metrics', category: 'Observability', description: 'Database metrics for Prometheus' },
    'Redis Exporter': { icon: '💾', port: 9121, url: 'http://localhost:9121/metrics', category: 'Observability', description: 'Cache metrics for Prometheus' },
    'NATS Exporter': { icon: '📨', port: 7777, url: 'http://localhost:7777/metrics', category: 'Observability', description: 'Message bus metrics for Prometheus' },
    'Node Exporter': { icon: '🖥️', port: 9100, url: 'http://localhost:9100/metrics', category: 'Observability', description: 'Host system metrics (CPU, RAM, Disk)' },
    'cAdvisor': { icon: '🐳', port: 8089, url: 'http://localhost:8089', category: 'Observability', description: 'Container resource monitoring' },
  };

  return backendServices.map(bs => {
    const config = serviceConfig[bs.name] || {
      icon: '📦',
      port: 'N/A',
      url: '#',
      category: 'Infrastructure' as ServiceCategory,
      description: bs.name
    };

    return {
      name: bs.name,
      status: bs.status,
      deployed: bs.status !== 'not-deployed',
      resources: bs.resources,
      ...config
    } as Service;
  });
}
