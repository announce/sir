#!/usr/bin/env bash

readonly CURRENT_DATE=$(date +'%Y-%m-%d %I:%M:%S')
readonly WORKING_DIR=$(basename "$(pwd)")

if [[ "${OSTYPE}" = "darwin"* ]]; then
  osascript -e "display notification \"${1-0}\" with title \"${*-${WORKING_DIR}}\"" && say "${1-0}"
elif [[ "${XDG_CURRENT_DESKTOP}" = "GNOME" ]]; then
  notify-send "${WORKING_DIR}" "${1-0}" && espeak "${1-0}"
fi

exit "${1-0}"
