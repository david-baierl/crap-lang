FROM rust:1.80.0
USER root

ENV RUNNING_IN_DOCKER=true

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y \
    git vim curl zsh

# gpg and yubikey
RUN apt-get install -y \
  gpg wget gnupg2 gnupg-agent dirmngr \
  cryptsetup scdaemon pcscd \
  yubikey-personalization yubikey-manager

# rust toolchain
RUN rustup component add rustfmt

# user
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN groupadd -g $GROUP_ID -o rust
RUN useradd -m -u $USER_ID -g $GROUP_ID -o -s /bin/zsh rust

# cache mount points
RUN mkdir -p /home/rust/.cache && chown rust:rust /home/rust/.cache
RUN mkdir -p /home/rust/.local && chown rust:rust /home/rust/.local

USER rust