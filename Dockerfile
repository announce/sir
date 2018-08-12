# https://hub.docker.com/_/rust/
FROM rust:1.28.0-stretch

WORKDIR "/usr/src/sir"
COPY . .
RUN rustup component add rustfmt-preview \
  && cargo install
