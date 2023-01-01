#! /bin/bash

podman build -t quay.io/uirlis/wasi-crun-deployer:latest .
podman push quay.io/uirlis/wasi-crun-deployer:latest