import { createFileRoute } from '@tanstack/react-router'
import { DataManagementPage } from '@/pages/settings/DataManagementPage'

export const Route = createFileRoute('/settings/data')({
  component: DataManagementPage,
})
