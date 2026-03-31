# Lightweight, scaffold-safe tasks for the current repository state.

default:
  @just --list

build-frontend-ui *APP:
  @if [ -f Cargo.toml ] && [ -d "interfaces/$APP" ] && command -v npm >/dev/null 2>&1; then \
    cd "interfaces/$APP" && \
    if [ -f package-lock.json ]; then \
      npm ci --silent; \
    else \
      npm install --silent; \
    fi && \
    npm run build; \
  else \
    echo "No Node/npm tooling or target app directory found; cannot build frontend."; \
  fi

build-frontend:
  @just build-frontend-ui biller && \
  just build-frontend-ui payer

serve-biller-ui:
  @if [ -d interfaces/biller ] && command -v npm >/dev/null 2>&1; then \
    cd interfaces/biller && \
    if [ ! -f node_modules/.bin/vite ]; then \
      npm install --silent; \
    fi && \
    npm run dev -- --host 0.0.0.0 --port 5173; \
  else \
    echo "Cannot run biller UI. Install npm and ensure interfaces/biller exists."; \
  fi

serve-payer-ui:
  @if [ -d interfaces/payer ] && command -v npm >/dev/null 2>&1; then \
    cd interfaces/payer && \
    if [ ! -f node_modules/.bin/vite ]; then \
      npm install --silent; \
    fi && \
    npm run dev -- --host 0.0.0.0 --port 5174; \
  else \
    echo "Cannot run payer UI. Install npm and ensure interfaces/payer exists."; \
  fi

test:
  @if [ -f Cargo.toml ]; then \
    if ! command -v cargo-nextest >/dev/null 2>&1; then \
      echo "cargo-nextest is required for mission-style test execution."; \
      echo "Enter the nix shell (nix develop) to load cargo-nextest from the repository flake."; \
      exit 1; \
    fi; \
    cargo nextest run --all-targets --all-features; \
  else \
    echo "No Cargo manifest found; skipping tests."; \
  fi

baggins *ARGS:
  @if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then \
    cargo run --bin baggins --all-features -- "$@"; \
  else \
    echo "No Cargo manifest or cargo toolchain detected; cannot run baggins."; \
  fi

e2e:
  @./e2e/run-e2e.sh

build:
  @if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then \
    cargo build --all-targets --all-features; \
  else \
    echo "No Cargo manifest or cargo toolchain detected; skipping build."; \
  fi

test-all:
  @if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then \
    cargo test --all; \
  else \
    echo "No Cargo manifest found; skipping tests."; \
  fi

quality:
  @if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then \
    cargo fmt --all -- --check; \
    if ! command -v cargo-nextest >/dev/null 2>&1; then \
      echo "cargo-nextest is required for mission-quality test execution."; \
      echo "Enter the nix shell (nix develop) to load cargo-nextest from the repository flake."; \
      exit 1; \
    fi; \
    cargo nextest run --all-targets --all-features; \
    if command -v cargo-clippy >/dev/null 2>&1; then \
      cargo clippy --all-targets --all-features; \
    else \
      echo "cargo-clippy not found; skipping clippy checks."; \
    fi; \
  else \
    echo "No cargo toolchain detected; skipping quality checks."; \
  fi
