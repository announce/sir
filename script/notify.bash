#!/usr/bin/env bash

CURRENT_DATE=$(date +'%Y-%m-%d %I:%M:%S')
WORKING_DIR=$(basename "$(pwd)")

if [[ "${OSTYPE}" = "darwin"* ]]; then
  osascript -e "display notification \"${CURRENT_DATE}\"" with title "${1-${WORKING_DIR}}"
fi
