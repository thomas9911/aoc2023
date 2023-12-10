#!/usr/bin/env just --justfile

test *ARGS:
  @just maturin-dev
  @just test-only {{ARGS}}

test-release *ARGS:
  @just maturin-release
  @just test-only {{ARGS}}

test-only *ARGS:
  poetry run pytest {{ARGS}}

format:
  poetry run black -q .
  cargo +nightly fmt

maturin-dev:
  poetry run maturin develop

maturin-release:
  poetry run maturin develop --release

install:
  poetry install

new-day day:
  bash ./adder.sh {{day}}
