FROM mcr.microsoft.com/vscode/devcontainers/rust:1-ubuntu-22.04

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
  && apt-get install -y nodejs

RUN npm install -g npm@latest svelte-language-server

USER vscode

WORKDIR /workspace