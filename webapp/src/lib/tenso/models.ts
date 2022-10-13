export type Initialized = {
  initialized: boolean;
};

export type Credentials = {
  username: string;
  password: string;
};

export type Link = {
  ident: string;
  destination: string;
  enabled: boolean;
  permanent_redirect: boolean;
};
