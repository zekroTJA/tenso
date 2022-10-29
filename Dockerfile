FROM node:slim AS build-fe
WORKDIR /build
COPY webapp .
RUN yarn
RUN yarn run build

FROM rust:slim AS build-be
WORKDIR /build
COPY migrations migrations
COPY tenso tenso
COPY Cargo.lock .
COPY Cargo.toml .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo build --bin tenso --release

FROM debian:11-slim AS release
COPY --from=build-be /build/target/release/tenso /bin/tenso
COPY --from=build-fe /build/dist /var/opt/tenso/webapp
RUN apt-get update && apt-get install -y libpq5
ENV WS_BINDADDRESS="0.0.0.0:80"
ENV WS_ASSETDIR="/var/opt/tenso/webapp"
EXPOSE 80
ENTRYPOINT [ "/var/tenso/tenso" ]