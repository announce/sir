#!/usr/bin/env bash

sir () {
  set -u

  init () {
    :
  }

  lint () {
    lint-shell \
    && lint-rust
  }

  lint-shell () {
    docker run --rm -v "$PWD:/mnt" koalaman/shellcheck:v0.5.0 \
      --exclude=SC1090 \
      script/*.bash
  }

  lint-rust () {
    docker run --rm -v "$(pwd)":/app -w /app instrumentisto/clippy:0.0.212
  }

  rust () {
    docker run --rm --interactive \
      --user "$(id -u)":"$(id -g)" \
      --env USER="sir" \
      --volume "$PWD":/usr/src/sir \
      --workdir /usr/src/sir \
      rust:1.28.0-stretch "$@"
  }

  clean () {
    docker system prune
  }

  die () {
    MESSAGE="${1:-Something went wrong.}"
    echo "[$(basename "$0")] ERROR: ${MESSAGE}" >&2
    exit 1
  }

  message () {
    MESSAGE="${1:-}"
    echo "[$(basename "$0")] INFO: ${MESSAGE}"
  }

  usage () {
    SELF="$(basename "$0")"
    echo -e "${SELF} -- sir
    \\nUsage: ${SELF} [arguments]
    \\nArguments:"
    declare -F | awk '{print "\t" $3}' | grep -v "${SELF}"
  }

  if [ $# = 0 ]; then
    usage
  elif [ "$(type -t "$1")" = "function" ]; then
    $1 "$(shift && echo "$@")"
  else
    rust "$@"
  fi
}

sir "$@"
