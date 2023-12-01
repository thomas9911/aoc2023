#!/usr/bin/env just --justfile

test:
  @just maturin-dev
  @just test-only

test-only:
  poetry run pytest

maturin-dev:
  poetry run maturin develop
