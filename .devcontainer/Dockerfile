FROM rust:1.80.1-bookworm
USER root

ENV RUNNING_IN_DOCKER=true

################################################
# basics
################################################

RUN apt update && apt upgrade -y
RUN apt install -y \
    git vim curl gnupg2

################################################
# shell
################################################

# install fish
RUN echo 'deb http://download.opensuse.org/repositories/shells:/fish:/release:/3/Debian_12/ /' | tee /etc/apt/sources.list.d/shells:fish:release:3.list
RUN curl -fsSL https://download.opensuse.org/repositories/shells:fish:release:3/Debian_12/Release.key | gpg --dearmor | tee /etc/apt/trusted.gpg.d/shells_fish_release_3.gpg > /dev/null
RUN apt update
RUN apt install -y fish

# install starship
RUN curl -sS https://starship.rs/install.sh | sh -s -- -y

# install fisher
RUN fish -c 'curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source && fisher install jorgebucaran/fisher'
RUN fish -c 'fisher install jorgebucaran/autopair.fish'
RUN fish -c 'fisher install edc/bass'

# rust toolchain
RUN rustup component add rustfmt

################################################
# user
################################################

ARG USER_ID=1000
ARG GROUP_ID=1000
RUN groupadd -g $GROUP_ID -o rust
RUN useradd -m -u $USER_ID -g $GROUP_ID -o -s /usr/bin/fish rust

# cache mount points
RUN mkdir -p /home/rust/.cache && chown rust:rust /home/rust/.cache
RUN mkdir -p /home/rust/.local && chown rust:rust /home/rust/.local

USER rust