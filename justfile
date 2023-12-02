#!/usr/bin/env just --justfile

test:
  @just maturin-dev
  @just test-only

test-only:
  poetry run pytest

format:
  poetry run black -q .
  cargo fmt

maturin-dev:
  poetry run maturin develop

install:
  poetry install
