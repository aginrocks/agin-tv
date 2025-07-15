import { createFileRoute, redirect } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: RouteComponent,
  loader: () => {
    throw redirect({
      to: "/welcome",
    });
  },
});

function RouteComponent() {
  return <>a</>;
}
