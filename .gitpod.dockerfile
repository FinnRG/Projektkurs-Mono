# Note: You can use any Debian/Ubuntu based image you want. 
FROM dtzar/helm-kubectl
ENV USER_UID=1001
RUN set -x && \
    apk add --no-cache ca-certificates && \
    apk add ffmpeg openssl libpq-dev && \
    adduser kubectl -Du ${USER_UID}

VOLUME [ "/var/lib/docker" ]

# Setting the ENTRYPOINT to docker-init.sh will start up the Docker Engine 
# inside the container "overrideCommand": false is set in devcontainer.json. 
# The script will also execute CMD if you need to alter startup behaviors.
ENTRYPOINT [ "/usr/local/bin/entrypoint" ]
CMD [ "sleep", "infinity" ]
