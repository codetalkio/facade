# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.208.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends cmake

# Switch to nightly and setup common cargo utilities.
RUN rustup default nightly \
  && cargo install cargo-binstall \
  && cargo binstall cargo-watch
