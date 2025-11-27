import path from 'path'
import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'
import { TanStackRouterVite } from '@tanstack/router-plugin/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    TanStackRouterVite(),
    react(),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 5173, // Vite default - avoids conflicts with Forgejo (3000) and Grafana (3001)
    proxy: {
      // Microservices Proxy (Local API Gateway)
      '/api/v1/auth': { target: 'http://localhost:8001', changeOrigin: true },
      '/api/v1/user/': { target: 'http://localhost:8002', changeOrigin: true }, // Authenticated user routes (singular)
      '/api/v1/users': { target: 'http://localhost:8002', changeOrigin: true }, // Public user routes (plural)
      '/api/v1/invitations': { target: 'http://localhost:8004', changeOrigin: true },
      '/api/v1/communities': { target: 'http://localhost:8006', changeOrigin: true },
      '/api/v1/badges': { target: 'http://localhost:8007', changeOrigin: true },
      '/api/v1/territories': { target: 'http://localhost:8008', changeOrigin: true },
      '/api/v1/utility': { target: 'http://localhost:8014', changeOrigin: true },
    },


  },
})
