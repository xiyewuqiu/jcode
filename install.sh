#!/usr/bin/env bash
# Quick installer entrypoint for xiyewuqiu/jcode
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || true)"
if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/scripts/install.sh" ]; then
  exec bash "$SCRIPT_DIR/scripts/install.sh" "$@"
else
  exec bash -c "$(curl -fsSL https://raw.githubusercontent.com/xiyewuqiu/jcode/master/scripts/install.sh)" "$@"
fi
