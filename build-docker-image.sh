#!/usr/bin/env bash

docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/oleggtro/go-e-prometheus-exporter:v0.1.0 --push .
