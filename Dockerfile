FROM rust:1-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml ./
COPY Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM alpine:3.24

WORKDIR /app

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/docker_notif /usr/local/bin/docker_notif

ENTRYPOINT ["docker_notif"]
