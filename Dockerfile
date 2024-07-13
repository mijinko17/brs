FROM lukemathwalker/cargo-chef:0.1.67-rust-1.78-slim-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY docker_files/proxy_cert/certificate.cer /usr/share/ca-certificates/proxy_cert/certificate.cer
RUN echo proxy_cert/certificate.cer >> /etc/ca-certificates.conf && \
  update-ca-certificates
RUN apt-get update
RUN apt-get install -y --no-install-recommends libssl-dev pkg-config
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:12.6-slim
RUN apt-get update
RUN apt-get install -y --no-install-recommends curl sqlite3
WORKDIR /app
COPY --from=builder /app/target/release/brocade-rest-simulator .
COPY docker_files/self_signed_certs/* self_signed_certs/
RUN mkdir data && touch data/database.db
ENTRYPOINT ["./brocade-rest-simulator"]