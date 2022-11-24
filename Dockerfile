FROM rust:slim-bullseye

RUN apt-get update && \
    apt-get install build-essential curl -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV SSL_VERSION=1.1.1s

RUN cd /tmp && \
    curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION* && \
    rm -rf /tmp/*

ENV OPENSSL_LIB_DIR=/usr/local/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/include/openssl \
    OPENSSL_STATIC=1

RUN ln -s /usr/local/lib/ /usr/local/ssl/lib && \
    ln -s /usr/local/include/openssl/ /usr/local/ssl/include

RUN cargo install wasm-pack --version "0.10.3" && \
    cargo install cargo-generate --version "0.17.2"

WORKDIR /work
