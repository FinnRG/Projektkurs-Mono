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