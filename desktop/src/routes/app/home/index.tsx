import Carousel from "@components/Carousel";
import { $api } from "@lib/providers/api";
import useApi from "@lib/providers/useApi";
import { useQuery } from "@tanstack/react-query";
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
