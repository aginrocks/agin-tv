import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/app")({
  component: RouteComponent,
  // loader: () => {
  //   throw redirect({
  //     to: "/app/home",
  //   });
  // },
});

function RouteComponent() {
  return <Outlet />;
}
