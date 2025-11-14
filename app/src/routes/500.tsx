import { createFileRoute } from '@tanstack/react-router'
import { ServerErrorPage } from '@/pages/errors/ServerErrorPage'

export const Route = createFileRoute('/500')({
  component: ServerErrorPage,
})
