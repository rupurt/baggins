#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$ROOT_DIR/.keel/e2e-logs"
mkdir -p "$LOG_DIR"

ensure_npm() {
  if command -v npm >/dev/null 2>&1; then
    return
  fi

  if command -v nix >/dev/null 2>&1 && [ -f "$ROOT_DIR/flake.nix" ]; then
    echo "npm not found, re-running e2e via nix develop."
    exec nix develop -c "$ROOT_DIR/e2e/run-e2e.sh"
  fi

  echo "npm is required to run e2e tests; install nodejs_20 or run this inside the nix shell."
  exit 1
}

ensure_npm

wait_for_ready() {
  local url="$1"
  for _ in {1..120}; do
    if curl -fsS "$url" >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.5
  done
  echo "Timed out waiting for ${url}"
  return 1
}

stop_processes() {
  if [ -n "${BAGGINS_PID:-}" ] && kill -0 "$BAGGINS_PID" 2>/dev/null; then
    kill "$BAGGINS_PID"
  fi
  if [ -n "${BILLER_PID:-}" ] && kill -0 "$BILLER_PID" 2>/dev/null; then
    kill "$BILLER_PID"
  fi
  if [ -n "${PAYER_PID:-}" ] && kill -0 "$PAYER_PID" 2>/dev/null; then
    kill "$PAYER_PID"
  fi
  wait || true
}
trap stop_processes EXIT

if [ ! -d "$ROOT_DIR/interfaces/biller/node_modules" ]; then
  (cd "$ROOT_DIR/interfaces/biller" && npm ci)
fi

if [ ! -d "$ROOT_DIR/interfaces/payer/node_modules" ]; then
  (cd "$ROOT_DIR/interfaces/payer" && npm ci)
fi

if [ ! -d "$ROOT_DIR/e2e/node_modules" ]; then
  (cd "$ROOT_DIR/e2e" && npm install)
fi

(
  cd "$ROOT_DIR" &&
    cargo run --bin baggins > "$LOG_DIR/baggins.log" 2>&1
) &
BAGGINS_PID=$!

(
  cd "$ROOT_DIR/interfaces/biller" &&
    npm run dev -- --host 127.0.0.1 --port 5173 > "$LOG_DIR/biller.log" 2>&1
) &
BILLER_PID=$!

(
  cd "$ROOT_DIR/interfaces/payer" &&
    npm run dev -- --host 127.0.0.1 --port 5174 > "$LOG_DIR/payer.log" 2>&1
) &
PAYER_PID=$!

wait_for_ready 'http://127.0.0.1:8080/v1/health'
wait_for_ready 'http://127.0.0.1:5173'
wait_for_ready 'http://127.0.0.1:5174'

(cd "$ROOT_DIR/e2e" && npm run test)
