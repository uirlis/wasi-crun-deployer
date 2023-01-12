# wasi-crun-deployer

A helm chart to deploy an OCI compatible runtime with Web Assembly capabilities into a kubernetes cluster to enable the developers to run `.wasm` files.

[![build status](https://github.com/uirlis/wasi-crun-deployer/workflows/CI/badge.svg)](https://github.com/uirlis/wasi-crun-deployer/actions)
[![Docker Repository on Quay](https://quay.io/repository/uirlis/wasi-crun-deployer/status "Docker Repository on Quay")](https://quay.io/repository/uirlis/wasi-crun-deployer)

## Overview

This project creates a build of the latest [WASMEdge](https://github.com/WasmEdge/WasmEdge) enabled [crun release](https://github.com/containers/crun).

This build is then bundled into a vendored container along with an executable to help with the deployment.

Finally there is a helm chart to deploy it into managed kubernetes services.

The deployment is performed by mounting 3 locations on each node:

1. The `/lib` folder to copy the shared objects  `libyajl.so.2` `libwasmedge.so.0` `libLLVM-12.so.1`

2. The `/usr/local/sbin` folder to deploy the `crun` executable

3. The additional runtime is added to either the `crio.conf` or the containerd `config.toml`

## Supported Versions

* WASMEdge = 0.11.2
* crun = `main` branch release only (See https://github.com/containers/crun/commit/26fe1383a05279935e67ee31e7ff10c43e7d87ea)
* IBM Cloud Kubernetes Service >= v1.24.9, Ubuntu 20.04
* IBM Cloud RedHat OpenShift Kubernetes Service >= v1.24.9, Ubuntu 20.04
* PRs welcome to support other platforms!

## Usage

1. Make sure the cluster is running [Ubuntu 20.04](https://cloud.ibm.com/docs/containers?topic=containers-ubuntu-migrate)
   This will be the default on IBM Cloud in March 2023

1. Install the chart
    ```
    helm install wasi-crun-deployer . --create-namespace --namespace wasiservice
    ```
1. Restart the containerd service by running a debug session on the node

    For xKS flavours
    ```
    kubectl debug node/NODE_NAME
    systemctl restart containerd
    ```

    For OpenShift
    ```
    oc debug node/NODE_NAME
    systemctl restart crio
    ```

## Validate

Run the following demo image using kubectl

```
kubectl run -it --restart=Never wasi-demo --image=docker.io/wasmedge/example-wasi:latest  \
--annotations="module.wasm.image/variant=compat-smart" \
--overrides='{"kind":"Pod", "apiVersion":"v1", "spec": \
{"hostNetwork": true, "runtimeClassName": "crun"}}' \
/wasi_example_main.wasm 50000000
```

This will output:

```
Random number: -1184619679
Random bytes: [188, 162, 226, 6, 56, 76, 130, 89, 149, 165, 30, 171, 6, 234, 228, 118, 217,
167, 176, 170, 199, 202, 10, 30, 76, 41, 106, 204, 253, 25, 122, 86, 218, 192, 37, 33, 80,
144, 161, 134, 21, 104, 1, 205, 78, 56, 125, 249, 123, 20, 74, 81, 100, 76, 234, 234, 239,
247, 251, 47, 96, 245, 139, 169, 129, 247, 205, 249, 188, 111, 77, 134, 254, 107, 200, 77,
4, 205, 241, 37, 131, 9, 29, 137, 106, 22, 222, 89, 18, 74, 101, 227, 14, 39, 176, 195, 51,
156, 101, 225, 87, 254, 97, 115, 161, 34, 180, 243, 238, 145, 67, 36, 218, 175, 202, 93, 185,
89, 188, 129, 197, 167, 255, 5, 97, 144, 171, 132]
Printed from wasi: This is from a main function
This is from a main function
The env vars are as follows.
PATH: /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
TERM: xterm
HOSTNAME: kube-xxxxxxxxxxxxxxxxx-xxxxxxxxxxxxxxx-xxxxxxxxxxxx-00000d61
KUBERNETES_PORT_443_TCP_PROTO: tcp
KUBERNETES_PORT_443_TCP_PORT: 443
KUBERNETES_PORT_443_TCP_ADDR: 172.x.x.x
KUBERNETES_SERVICE_HOST: 172.x.x.x
KUBERNETES_SERVICE_PORT: 443
KUBERNETES_SERVICE_PORT_HTTPS: 443
KUBERNETES_PORT: tcp://172.x.x.x:443
KUBERNETES_PORT_443_TCP: tcp://172.x.x.x:443
HOME: /
The args are as follows.
/wasi_example_main.wasm
50000000
File content is This is in a file
```

## Features

Below are the intended features in the rough order of execution.

* [x] Deploy onto IBM Cloud IKS
* [x] Deploy onto IBM Cloud OpenShift
* [x] Support for smart annotations for sidecars
* [ ] Operator Support
* [ ] Deploy onto Amazon EKS/ROSA
* [ ] Deploy onto Azure AKS/ARO
* [ ] [Cloud Events](https://cloudevents.io/) Example

GCP unlikely to be supported as it locks down the host file system as read only.
