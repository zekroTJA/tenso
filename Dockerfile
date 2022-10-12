FROM rust:slim AS build
WORKDIR /build
COPY migrations migrations
COPY tenso tenso
COPY Cargo.lock .
COPY Cargo.toml .
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo build --bin tenso --release

FROM debian:11-slim AS release
COPY --from=build /build/target/release/tenso /bin/tenso
RUN apt-get update && apt-get install -y libpq5
ENV WS_BINDADDRESS="0.0.0.0:80"
EXPOSE 80
ENTRYPOINT [ "/bin/tenso" ]