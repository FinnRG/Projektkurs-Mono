FROM bufbuild/buf:1.4.0 as buf
FROM envoyproxy/envoy-dev:latest

# Generate Buf image 
COPY buf.work.yaml buf.work.yaml
COPY proto/ proto/
COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
RUN buf build -o image.bin

COPY envoy.yaml /etc/envoy/envoy.yaml
RUN chmod go+r /etc/envoy/envoy.yaml
