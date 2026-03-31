# Lightweight, scaffold-safe tasks for the current repository state.

default:
  @just --list

test:
  @if [ -f Cargo.toml ]; then \
    if command -v cargo-nextest >/dev/null 2>&1; then \
      cargo nextest run --all-targets --all-features; \
    elif command -v cargo >/dev/null 2>&1; then \
      echo "cargo-nextest not found; falling back to cargo test --all."; \
      cargo test --all; \
    else \
      echo "Cargo not found; skipping tests."; \
    fi; \
  else \
    echo "No Cargo manifest found; skipping tests."; \
  fi

quality:
  @if [ -f Cargo.toml ] && command -v cargo >/dev/null 2>&1; then \
    cargo fmt --all -- --check; \
    if command -v cargo-clippy >/dev/null 2>&1; then \
      cargo clippy --all-targets --all-features; \
    else \
      echo "cargo-clippy not found; skipping clippy checks."; \
    fi; \
  else \
    echo "No cargo toolchain detected; skipping quality checks."; \
  fi
