#!/bin/bash
echo $(pwd)
ls -l 
docker run --rm -v `pwd`:/src ghcr.io/toltec-dev/rust:latest cargo build --manifest-path /src --release
