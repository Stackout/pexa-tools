FROM rust:1.35.0-slim AS build

# Add musl deps for targeting alpine build:
RUN apt-get update \
  && apt-get install musl-tools -y \
  && rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new --bin build --name pexa-tools
WORKDIR /build

# Faster images builds by first building deps.
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build \
    --release \
    --target=x86_64-unknown-linux-musl \
  && rm ./src/*.rs \
  && rm ./target/x86_64-unknown-linux-musl/release/deps/auth_proxy*

# Build source.
COPY ./src ./src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build \
    --release \
    --target=x86_64-unknown-linux-musl

FROM alpine:3.10.0

RUN apk --no-cache add tini

# Run as non-root user.
RUN addgroup -g 1000 pexa-tools \
    && adduser -D -s /bin/sh -u 1000 -G pexa-tools pexa-tools
USER pexa-tools

# Copy binary from build stage.
COPY --chown=pexa-tools --from=build /build/target/x86_64-unknown-linux-musl/release/auth_proxy /usr/local/bin/pexa-tools

ENTRYPOINT [ "tini", "--" ]
CMD ["pexa-tools"]
