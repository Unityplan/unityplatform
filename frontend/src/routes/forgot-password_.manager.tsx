import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/forgot-password_/manager')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/forgot-password_/manager"!</div>
}
