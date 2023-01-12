# WASI Crun Deploy

## OpenShift

In OpenShift 4.11 the Pod Security Standards are enforced and require specific configuration as this chart requires elevated privileges to deploy.

In order to meet those privaleges create a project called wasi-deploy and run the following commands to enable pods to run in a privaleged mode:

```bash
kubectl label --overwrite ns wasi-deploy   pod-security.kubernetes.io/enforce=privileged   pod-security.kubernetes.io/enforce-version=v1.24
kubectl label --overwrite ns wasi-deploy   pod-security.kubernetes.io/audit=privileged   pod-security.kubernetes.io/enforce-version=v1.24
kubectl label --overwrite ns wasi-deploy   pod-security.kubernetes.io/warn=privileged   pod-security.kubernetes.io/enforce-version=v1.24
```

Validate the configuration is complete with

```
kubectl get ns wasi-deploy --show-labels
NAME          STATUS   AGE   LABELS
wasi-deploy   Active   25m   kubernetes.io/metadata.name=wasi-deploy,pod-security.kubernetes.io/audit-version=v1.24,pod-security.kubernetes.io/audit=privileged,pod-security.kubernetes.io/enforce-version=v1.24,pod-security.kubernetes.io/enforce=privileged,pod-security.kubernetes.io/warn-version=v1.24,pod-security.kubernetes.io/warn=privileged
```

