FROM rust:1.63.0-slim

RUN apt-get update \
    && apt-get install -y -q \
        libssl-dev \
        pkg-config