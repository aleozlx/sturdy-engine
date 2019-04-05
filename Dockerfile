FROM ubuntu:18.04
MAINTAINER Alex Yang <aleozlx@gmail.com>

# System dependencies
RUN apt-get -y update && apt-get install -y build-essential python3-dev python3-pip
RUN apt-get install -y vim wget file patch sudo

# Rustup & Cargo
RUN wget -P /tmp --quiet https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init && chmod +x /tmp/rustup-init && /tmp/rustup-init --default-toolchain nightly -y

RUN ln -sT /usr/bin/python3 /usr/bin/python
RUN ln -sT /root/.cargo/bin/cargo /usr/bin/cargo
COPY . /root/sturdy-engine
WORKDIR /root/sturdy-engine

RUN cargo build

# CMD [ "target/debug/sturdy-engine" ]
