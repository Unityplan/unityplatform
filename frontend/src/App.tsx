import { useState } from 'react'
import { LoginPage } from './pages/auth/LoginPage'
import { RegisterPage } from './pages/auth/RegisterPage'
import { PasswordResetPage } from './pages/auth/PasswordResetPage'
import { ProfileViewPage } from './pages/profile/ProfileViewPage'
import { ProfileEditPage } from './pages/profile/ProfileEditPage'
import './App.css'

function App() {
  const [page, setPage] = useState<'login' | 'register' | 'reset-password' | 'profile' | 'profile-edit'>('login')

  // Simple navigation for testing (will be replaced with proper routing later)
  if (typeof window !== 'undefined') {
    const path = window.location.pathname
    if (path === '/register' && page !== 'register') {
      setPage('register')
    } else if (path === '/reset-password' && page !== 'reset-password') {
      setPage('reset-password')
    } else if (path === '/profile' && page !== 'profile') {
      setPage('profile')
    } else if (path === '/profile/edit' && page !== 'profile-edit') {
      setPage('profile-edit')
    } else if (path === '/login' && page !== 'login') {
      setPage('login')
    }
  }

  if (page === 'register') return <RegisterPage />
  if (page === 'reset-password') return <PasswordResetPage />
  if (page === 'profile') return <ProfileViewPage />
  if (page === 'profile-edit') return <ProfileEditPage />
  return <LoginPage />
}

export default App
