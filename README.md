# Projektkurs-Mono

## Run locally 

```
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm install minio --set auth.rootUser=minio-admin --set auth.rootPassword=strongPassword bitnami/minio
helm install postgres -f conf/postgres.yaml bitnami/postgresql
helm install redis --set auth.password=strongPassword bitnami/redis
kubectl apply -f manifests/
```

## Observability

```
kubectl create ns observability
helm repo add grafana https://grafana.github.io/helm-charts
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update
helm install -n observability --set fluent-bit.enabled=true,promtail.enabled=false loki-stack grafana/loki-stack
helm install -n observability tempo grafana/tempo
helm install mimir-distributed grafana/mimir-distributed -n observability
helm install -n observability -f kube-prometheus-stack.yaml kube-prometheus-stack prometheus-community/kube-prometheus-stack
```