#!/bin/bash

cd ../
set -a
export $(grep -v '^#' .env | xargs)
set +a
cd ../charts/wasi-crun-deployer

helm install wasi-crun-deployer . --create-namespace --namespace wasiservice
