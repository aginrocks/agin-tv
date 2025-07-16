import type { paths } from "@/types/api";
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";

const fetchClient = createFetchClient<paths>({
  baseUrl: "http://localhost:42069",
});

export const $api = createClient(fetchClient);
