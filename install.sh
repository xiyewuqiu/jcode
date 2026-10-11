#!/usr/bin/env bash
# Quick installer entrypoint for xiyewuqiu/jcode
set -eo pipefail
cd "$HOME" 2>/dev/null || true

SCRIPT_SOURCE="${BASH_SOURCE[0]:-}"
SCRIPT_DIR=""
if [ -n "$SCRIPT_SOURCE" ]; then
  SCRIPT_DIR="$(cd "$(dirname "$SCRIPT_SOURCE")" 2>/dev/null && pwd || true)"
fi

if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/scripts/install.sh" ]; then
  exec bash "$SCRIPT_DIR/scripts/install.sh" "$@"
else
  exec bash -c "$(curl -fsSL https://raw.githubusercontent.com/xiyewuqiu/jcode/master/scripts/install.sh)" "$@"
fi
