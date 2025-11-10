import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/forgot-password_/friend')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/forgot-password_/friend"!</div>
}
