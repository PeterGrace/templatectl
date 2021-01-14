#!/bin/bash
docker run --rm -v /home/runner/work/templatectl/templatectl:/src ghcr.io/toltec-dev/rust:latest cargo build --manifest-path /src/Cargo.toml --release
