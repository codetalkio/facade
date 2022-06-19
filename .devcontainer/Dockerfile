# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.208.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends cmake

# Switch to nightly and setup common cargo utilities.
RUN rustup default nightly \
  # linux x86_64: wget https://github.com/ryankurte/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
  # macos M1: wget https://github.com/ryankurte/cargo-binstall/releases/latest/download/cargo-binstall-aarch64-apple-darwin.zip
  && wget https://github.com/ryankurte/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && tar -xf cargo-binstall-x86_64-unknown-linux-musl.tgz \
  && mv cargo-binstall /usr/local/bin/cargo-binstall \
  && cargo binstall --no-confirm cargo-watch
