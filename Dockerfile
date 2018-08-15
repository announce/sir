# https://hub.docker.com/_/rust/
# https://hub.docker.com/r/rustlang/rust/
ARG IMAGE_TAG="rust:1.28.0-stretch"
FROM ${IMAGE_TAG}

WORKDIR "/usr/src/sir"
COPY . .
RUN rustup component add rustfmt-preview \
  && cargo install
