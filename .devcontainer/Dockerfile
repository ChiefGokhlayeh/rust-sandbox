# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.145.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    python3 \
    python3-pip \
    && python3 -m pip install pipenv \
    && apt-get -y autoremove \
    && apt-get clean \
    && rm -rf /var/cache/apt/archives /var/lib/apt/lists/*
