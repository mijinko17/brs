FROM rust:1.78 AS builder

WORKDIR /app

COPY src src
COPY Cargo.toml .
COPY Cargo.lock .

COPY proxy_cert/certificate.cer /usr/share/ca-certificates/proxy-cert/certificate.cer
RUN echo proxy-cert/certificate.cer >> /etc/ca-certificates.conf && \
  update-ca-certificates

RUN cargo build --release

FROM debian:12.6-slim
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates curl

COPY --from=builder /app/target/release/brocade-rest-simulator .
COPY ./self_signed_certs/* self_signed_certs/

ENTRYPOINT [ "./brocade-rest-simulator"]