import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { router } from './router'
import { ThemeProvider } from './components/theme-provider'
import { Toaster } from 'sonner'
import './index.css'

// Initialize reduced motion preference on app load
if (typeof localStorage !== 'undefined') {
  const reducedMotion = localStorage.getItem('reducedMotion') === 'true'
  if (reducedMotion) {
    document.documentElement.classList.add('reduce-motion')
  }
}

// Create a client
const queryClient = new QueryClient()

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
        <RouterProvider router={router} />
        <Toaster position="top-right" richColors closeButton />
      </ThemeProvider>
    </QueryClientProvider>
  </StrictMode>,
)
