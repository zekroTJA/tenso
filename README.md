# tensō

> ### WIP
> This project is currently work in progress. Feel free to follow any development on this project.

tensō (転送, jap. for `forwarding`) is just another short link service, primarily built to gather more knowledge about the Rust language, actix-web and diesel.

## Usage

It is recommendet to host tenso using the provided Docker image. You can either clone this repository 
and build the Docker image yourself or you can use the [prebuilt docker images](https://github.com/zekroTJA/tenso/pkgs/container/tenso).

### Configuration

tensō is configured using environment variables.

 Name | Type | Required | Description 
------|------|----------|-------------
`DATABASE_URL` | `string` | Yes | The Postgres database connection stirng.
`WS_SIGNING_KEY` | `string` | Yes | The signing key used for the webapp authentication tokens. This should be a random string with at least 32 characters length.
`WS_ORIGINURL` | `string` | No | The URL on which your tensō instance will be accessible to. This is required to properly set and validate XSRF tokens and CORS headers.
`WS_BINDADDR` | `string` | No (default: `0.0.0.0:80`) | The address the backend service will bind to.
`WS_REDIRECT_DEFAULT` | `string` | No | The URL the server will redirect to when navigating to `<addr>/`.
`WS_REDIRECT_NOTFOUND` | `string` | No | The URL the server will redirect to when the specified short link does not exist.
`WS_ASSETDIR` | `string` | No (default: `./webapp/dist`) | Can be used to specify a custom location for the webapp asset directory.
`DEBUG_MODE` | `boolean` | No (default: `false`) | Disables XSRF tokens and CORS headers. **This should only be used for development and never be enabled in production!**

After first setup, you need to grab the initialization token which is printed to the logs of the service on startup. This token is then to be used in the web interface to create the initial user. After that, you can log in with the credentials specified to the web interface.