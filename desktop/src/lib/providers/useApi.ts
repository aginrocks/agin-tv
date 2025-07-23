import { useMemo } from "react";
import type { paths } from "@/types/api";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";
import { useAtomValue } from "jotai";
import { tokenAtom } from "@lib/atoms/token";
import { useNavigate } from "@tanstack/react-router";

export default function useApi() {
  const token = useAtomValue(tokenAtom);
  const navigate = useNavigate();

  const api = useMemo(() => {
    if (!token) {
      navigate({ to: "/" });
    }
    const fetchClient = createFetchClient<paths>({
      baseUrl: "http://localhost:42069",
      headers: {
        Authorization: token ? `Bearer ${token}` : "",
      },
    });

    const $api = createClient(fetchClient);

    return $api;
  }, [token]);

  return api;
}
