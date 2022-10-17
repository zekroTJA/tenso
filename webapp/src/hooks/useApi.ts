import { APIError, Client } from "../lib/tenso";

import { APIClient } from "../services/api";
import { useNavigate } from "react-router-dom";

export const useApi = () => {
  const nav = useNavigate();

  async function fetch<T>(
    req: (c: Client) => Promise<T>,
    silenceErrors: boolean = false
  ): Promise<T> {
    try {
      return await req(APIClient);
    } catch (e) {
      if (!silenceErrors && e instanceof APIError && e.status === 401) {
        nav("/login");
      }
      throw e;
    }
  }

  return fetch;
};
