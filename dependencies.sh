helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm upgrade --install minio -f conf/minio.yaml bitnami/minio
helm upgrade --install postgres --set auth.postgresPassword=strongPassword,auth.database=projekt,primary.persistence.size=500Mi,readReplicas.persistence.size=200Mi bitnami/postgresql
helm upgrade --install redis --set auth.password=strongPassword,master.persistence.enabled=false,replicas.persistence.enabled=false bitnami/redis
helm upgrade --install kafka bitnami/kafka