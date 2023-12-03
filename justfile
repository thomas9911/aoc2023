#!/usr/bin/env just --justfile

test *ARGS:
  @just maturin-dev
  @just test-only {{ARGS}}

test-only *ARGS:
  poetry run pytest {{ARGS}}

format:
  poetry run black -q .
  cargo +nightly fmt

maturin-dev:
  poetry run maturin develop

install:
  poetry install
