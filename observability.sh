kubectl create ns observability
helm repo add grafana https://grafana.github.io/helm-charts
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update
helm install -n observability loki grafana/loki
helm install -n observability --set fsInotifyMaxUserInstances=8096 --set config.lokiAddress="http://loki.observability.svc.cluster.local:3100/loki/api/v1/push" promtail grafana/promtail
helm install -n observability tempo grafana/tempo
helm install mimir-distributed grafana/mimir-distributed -n observability -f conf/mimir-distributed.yaml
helm install -n observability -f conf/kube-prometheus-stack.yaml kube-prometheus-stack prometheus-community/kube-prometheus-stack