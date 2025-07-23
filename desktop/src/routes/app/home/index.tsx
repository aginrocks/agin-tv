import Carousel from "@components/Carousel";
import useApi from "@lib/providers/useApi";
import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/app/home/")({
  component: RouteComponent,
});

function RouteComponent() {
  const api = useApi();

  const xd = api.useQuery("get", "/api/home");

  useEffect(() => {
    console.log(xd);
  }, [xd]);

  return (
    <div>
      <Carousel slides={xd.data?.carousel} />
    </div>
  );
}
