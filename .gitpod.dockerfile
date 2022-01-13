FROM gitpod/workspace-postgres

RUN sudo apt-get update -y && \
    sudo apt install -y --no-install-recommends ffmpeg