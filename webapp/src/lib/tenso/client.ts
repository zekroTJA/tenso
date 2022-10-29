import {
  Count,
  Credentials,
  InitCredentials,
  Initialized,
  Link,
  Stats,
} from "./models";

import { HttpClient } from "./http";

export class Client {
  private httpClient: HttpClient;

  constructor(endpoint: string) {
    this.httpClient = new HttpClient(endpoint);
  }

  authCheckInit(): Promise<Initialized> {
    return this.httpClient.req("GET", "auth/init");
  }

  authInit(creds: InitCredentials): Promise<void> {
    return this.httpClient.req("POST", "auth/init", creds);
  }

  authLogin(creds: Credentials): Promise<void> {
    return this.httpClient.req("POST", "auth/login", creds);
  }

  authCheck(): Promise<void> {
    return this.httpClient.req("GET", "auth/check");
  }

  links(search?: string, limit?: number, offset?: number): Promise<Link[]> {
    const params = new URLSearchParams();
    limit && params.set("limit", limit!.toString());
    offset && params.set("offset", offset!.toString());
    search && params.set("search", search!);
    return this.httpClient.req("GET", `links?${params.toString()}`);
  }

  link(id: string): Promise<Link> {
    return this.httpClient.req("GET", `links/${id}`);
  }

  linkCreate(link: Link): Promise<Link> {
    return this.httpClient.req("POST", "links", link);
  }

  linkUpdate(id: string, link: Partial<Link>): Promise<Link> {
    return this.httpClient.req("POST", `links/${id}`, link);
  }

  linkDelete(id: string): Promise<void> {
    return this.httpClient.req("DELETE", `links/${id}`);
  }

  stats(id: string, from?: string, to?: string): Promise<Stats> {
    const params = new URLSearchParams();
    from && params.set("from", from!);
    to && params.set("to", to!);
    return this.httpClient.req("GET", `stats/${id}?${params.toString()}`);
  }

  count(id: string): Promise<Count> {
    return this.httpClient.req("GET", `stats/${id}/count`);
  }
}
