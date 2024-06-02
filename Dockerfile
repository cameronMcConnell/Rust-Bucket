FROM rust:1.60 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rust-bucket /usr/local/bin/rust-bucket

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

ENV APP_ADDRESS=${APP_ADDRESS:-0.0.0.0}
ENV APP_PORT=${APP_PORT:-8000}

EXPOSE 8000

CMD ROCKET_ADDRESS=$APP_ADDRESS ROCKET_PORT=$APP_PORT /usr/local/bin/rust-bucket