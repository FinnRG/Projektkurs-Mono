FROM bufbuild/buf:1.3.1 as buf
FROM envoyproxy/envoy-dev:d4e28efb56a79703faffc18d6af1a578770af2f1

# Generate Buf image 
COPY buf.work.yaml buf.work.yaml
COPY proto/ proto/
COPY --from=buf /usr/local/bin/buf /usr/local/bin/buf
RUN buf build -o image.bin

COPY envoy.yaml /etc/envoy/envoy.yaml
RUN chmod go+r /etc/envoy/envoy.yaml
