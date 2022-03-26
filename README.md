# Projektkurs-Mono

## Run locally 

```
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm install minio --set auth.rootUser=minio-admin --set auth.rootPassword=strongPassword bitnami/minio
helm install postgres -f conf/postgres.yml bitnami/postgresql
helm install redis --set auth.password=strongPassword bitnami/redis
kubectl create deployment --image=finnrg/backend:latest projekt-backend
kubectl create service nodeport projekt-backend --tcp=8000:8000
kubectl port-forward svc/projekt-backend 8000:8000
```