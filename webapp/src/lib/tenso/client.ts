import { Credentials, InitCredentials, Initialized, Link } from "./models";

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

  links(): Promise<Link[]> {
    return this.httpClient.req("GET", "links");
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

  stats(id: string, from?: string, to?: string): Promise<void> {
    const params = new URLSearchParams();
    from ?? params.set("from", from!);
    to ?? params.set("to", to!);
    return this.httpClient.req("GET", `stats/${id}?${params.toString()}`);
  }
}
