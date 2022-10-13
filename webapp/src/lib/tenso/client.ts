import { APIError } from "./errors";

export type HttpMethod =
  | "GET"
  | "PUT"
  | "POST"
  | "DELETE"
  | "PATCH"
  | "OPTIONS";

export type HttpHeadersMap = { [key: string]: string };

export class HttpClient {
  private _xsrfToken: string | undefined = undefined;

  constructor(public endpoint: string) {}

  async req<T>(
    method: HttpMethod,
    path: string,
    body?: object,
    headers: HttpHeadersMap = {}
  ): Promise<T> {
    const _headers = new Headers();
    _headers.set("Accept", "application/json");
    Object.keys(headers).forEach((k) => _headers.set(k, headers[k]));

    let _body = null;
    if (!!body) {
      if (body instanceof File) {
        const formData = new FormData();
        formData.append("file", body);
        _body = formData;
      } else {
        _headers.set("Content-Type", "application/json");
        _body = JSON.stringify(body);
      }
    }

    let xsrfToken = this.xsrfToken;
    if (!!xsrfToken) {
      _headers.append("X-XSRF-Token", xsrfToken);
    }

    const fullPath = replaceDoublePath(`${this.endpoint}/${path}`);
    const res = await window.fetch(fullPath, {
      method,
      headers: _headers,
      body: _body,
      credentials: "include",
    });

    if (res.status >= 400) throw new APIError(res, await res.text());

    if (res.status === 204) {
      return {} as T;
    }

    let data = {};
    try {
      data = await res.json();
    } catch {}

    return data as T;
  }

  protected basePath(path?: string): string {
    return replaceDoublePath(`${this.endpoint}/${path}`);
  }

  private get xsrfToken() {
    if (this._xsrfToken) return this._xsrfToken;
    this._xsrfToken = getCookie("xsrf-token");
    return this._xsrfToken;
  }
}

function replaceDoublePath(url: string): string {
  const split = url.split("://");
  split[split.length - 1] = split[split.length - 1].replace(/\/\//g, "/");
  return split.join("://");
}

function getCookie(key: string) {
  return document.cookie
    .split(";")
    .map((kv) => kv.trim().split("=", 2))
    .filter((kv) => kv.length === 2 && !!kv[0] && !!kv[1])
    .find((kv) => kv[0] === key)
    ?.at(1);
}
