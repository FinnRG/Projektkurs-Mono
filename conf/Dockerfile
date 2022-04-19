FROM envoyproxy/envoy:v1.22.0
COPY image.bin  /image.bin
COPY envoy.yaml /etc/envoy/envoy.yaml
RUN chmod go+r /etc/envoy/envoy.yaml
