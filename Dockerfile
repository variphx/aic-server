FROM rust:alpine AS builder-cache
WORKDIR /app/

RUN apk add build-base openssl-dev curl
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/ && echo "fn main() {}" > src/main.rs
RUN cargo build --release

FROM rust:alpine AS builder
WORKDIR /app/

COPY Cargo.toml Cargo.lock /app/
COPY src/ /app/src/
COPY --from=builder-cache /app/target/ /app/target/
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

COPY --from=builder /app/target/release/aic-server /app/server

EXPOSE 8080

CMD [ "/app/server" ]