FROM rust:1.78 AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:12.6-slim
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates curl

COPY --from=builder /app/target/release/brocade-rest-simulator .
COPY ./self_signed_certs/* self_signed_certs/

ENTRYPOINT [ "./brocade-rest-simulator"]