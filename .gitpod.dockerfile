FROM gitpod/workspace-postgres

RUN sudo apt-get update && \
    sudo apt install ffmpeg