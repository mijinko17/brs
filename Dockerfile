FROM lukemathwalker/cargo-chef:0.1.67-rust-1.78-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
COPY proxy_cert/certificate.cer /usr/share/ca-certificates/proxy_cert/certificate.cer
RUN echo proxy_cert/certificate.cer >> /etc/ca-certificates.conf && \
  update-ca-certificates
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY proxy_cert/certificate.cer /usr/share/ca-certificates/proxy_cert/certificate.cer
RUN echo proxy_cert/certificate.cer >> /etc/ca-certificates.conf && \
  update-ca-certificates
RUN cargo build --release

FROM debian:12.6-slim
WORKDIR /app
COPY --from=builder /app/target/release/brocade-rest-simulator .
COPY ./self_signed_certs/* self_signed_certs/
ENTRYPOINT ["./brocade-rest-simulator"]