FROM rust:1.83.0-bookworm AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:12-slim

RUN apt-get update && apt-get install -y ca-certificates

COPY --from=build /app/target/release/pgsql-monitor-api /server

CMD ["/server"]
