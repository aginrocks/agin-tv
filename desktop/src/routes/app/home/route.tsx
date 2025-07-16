import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/app/home")({
  component: RouteComponent,
});

function RouteComponent() {
  return <Outlet />;
}
