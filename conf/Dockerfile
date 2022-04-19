FROM envoyproxy/envoy-dev:d4e28efb56a79703faffc18d6af1a578770af2f1
COPY image.bin  /image.bin
COPY envoy.yaml /etc/envoy/envoy.yaml
RUN chmod go+r /etc/envoy/envoy.yaml
