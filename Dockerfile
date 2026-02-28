# Stage 1
FROM rust:1.93-alpine AS build
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Stage 2
FROM alpine:3.23 AS final

COPY --from=build /app/target/release/glitchtip-telegram /glitchtip-telegram

EXPOSE 8000

CMD ["/glitchtip-telegram"]
