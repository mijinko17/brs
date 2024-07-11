FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update
RUN apt-get install -y --no-install-recommends curl sqlite3
USER vscode
RUN cargo install --force cargo-make