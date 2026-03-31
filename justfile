# Lightweight, scaffold-safe tasks for the current repository state.

default:
  @just --list

test:
  @if [ -f Cargo.toml ]; then \
    if ! command -v cargo-nextest >/dev/null 2>&1; then \
      echo "cargo-nextest is required for mission-style test execution."; \
      echo "Install with: cargo install cargo-nextest --locked"; \
      exit 1; \
    fi; \
    cargo nextest run --all-targets --all-features; \
  else \
    echo "No Cargo manifest found; skipping tests."; \
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
      echo "Install with: cargo install cargo-nextest --locked"; \
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
