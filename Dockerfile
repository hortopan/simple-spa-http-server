FROM rust

WORKDIR /spa

COPY Cargo.lock /spa/
COPY Cargo.toml /spa/

RUN \
    mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src /spa/src

RUN \
    touch src/main.rs && \
    cargo test --release && \
    cargo build --release && \
    mv target/release/simple-spa-http-server bin && \
    rm -rf target

FROM ubuntu
RUN apt-get update && apt-get install openssl -y

WORKDIR /spa

COPY --from=0 /spa/bin /spa/bin
RUN mkdir www

ENTRYPOINT ["/spa/bin"]
