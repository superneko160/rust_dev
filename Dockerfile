FROM rust:1.86-slim

RUN apt-get update \
    && apt-get install -y -q \
        libssl-dev \
        pkg-config
