FROM rust:1.80-slim

RUN apt-get update \
    && apt-get install -y -q \
        libssl-dev \
        pkg-config