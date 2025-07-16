import Carousel from "@components/Carousel";
import { $api } from "@lib/providers/api";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/app/home/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { data: homeData } = $api.useQuery("get", "/api/home");

  return (
    <div>
      <Carousel />
    </div>
  );
}
