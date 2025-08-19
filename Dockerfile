FROM rust:alpine AS builder
WORKDIR /app

RUN apk add build-base

COPY Cargo.toml Cargo.lock /app/
COPY src/ /app/src/
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

COPY --from=builder /app/target/release/aic-server /app/server

EXPOSE 8080

CMD [ "/app/server" ]