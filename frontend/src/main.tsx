import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { RouterProvider } from '@tanstack/react-router'
import { router } from './router'
import { ThemeProvider } from './components/theme-provider'
import './index.css'

// Initialize reduced motion preference on app load
if (typeof localStorage !== 'undefined') {
  const reducedMotion = localStorage.getItem('reducedMotion') === 'true'
  if (reducedMotion) {
    document.documentElement.classList.add('reduce-motion')
  }
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      <RouterProvider router={router} />
    </ThemeProvider>
  </StrictMode>,
)
