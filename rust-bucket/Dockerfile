FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /Rust-Bucket/bucket

COPY /target/release/rust-bucket /Rust-Bucket/rust-bucket

EXPOSE 8000

CMD ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8000 /Rust-Bucket/rust-bucket