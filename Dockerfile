FROM rust:1.89-bullseye

USER root

RUN apt-get update -y
RUN apt-get install -y git nodejs npm

RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install wasm-pack

WORKDIR /usr/workspace/wasm

VOLUME ./wasm:/usr/workspace/wasm