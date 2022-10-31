import { APIError, Client } from "../lib/tenso";

import { APIClient } from "../services/api";
import { Embed } from "../components/Embed";
import { useNavigate } from "react-router-dom";
import { useNotification } from "./useNotification";

export const useApi = () => {
  const nav = useNavigate();
  const { showNotification } = useNotification();

  async function fetch<T>(
    req: (c: Client) => Promise<T>,
    silenceErrors?: boolean | number | number[]
  ): Promise<T> {
    if (typeof silenceErrors === "number") silenceErrors = [silenceErrors];
    try {
      return await req(APIClient);
    } catch (e) {
      if (typeof silenceErrors === "boolean" && silenceErrors) throw e;
      if (e instanceof APIError) {
        if (silenceErrors && silenceErrors.includes(e.status)) throw e;
        if (e.status === 401) {
          nav("/login");
        } else {
          showNotification(
            <span>
              <strong>API Error:</strong>&nbsp;{e.message}{" "}
              <Embed>({e.status})</Embed>
            </span>,
            "error",
            6000
          );
        }
      } else {
        showNotification(
          <span>
            <strong>Error:</strong>&nbsp;Unknown Request Error:{" "}
            <Embed>{`${e}`}</Embed>
          </span>,
          "error",
          6000
        );
      }
      throw e;
    }
  }

  return fetch;
};
