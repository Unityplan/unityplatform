import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/forgot-password_/email')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/forgot-password_/email"!</div>
}
