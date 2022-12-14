export type Initialized = {
  initialized: boolean;
};

export type Credentials = {
  username: string;
  password: string;
};

export type InitCredentials = Credentials & {
  token: string;
};

export type Link = {
  id: string;
  ident: string;
  destination: string;
  enabled: boolean;
  permanent_redirect: boolean;
};

export type Count = {
  count: number;
};

export type Stats = [string, number][];
