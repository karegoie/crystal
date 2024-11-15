FROM rust:1

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
  && apt-get install -y nodejs

RUN npm install -g npm@latest svelte-language-server

USER vscode