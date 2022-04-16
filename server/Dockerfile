FROM rust:1-alpine3.15 AS dependencies

RUN rustup default stable
RUN apk add --no-cache musl-dev

FROM dependencies AS builder

WORKDIR /var/www

COPY . .
RUN cargo build

FROM builder AS dev

CMD cargo run
