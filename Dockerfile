# ---------------------------
# Stage 1: Build (MUSL target)
# ---------------------------
FROM rust:1.82 AS builder

# Install musl-tools and other dependencies
RUN apt-get update && \
    apt-get install -y musl-tools pkg-config && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/cart-service

# Copy manifest files first
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Copy actual source
COPY src/ ./src/
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---------------------------
# Stage 2: Runtime (Alpine)
# ---------------------------
FROM alpine:latest

RUN adduser -D appuser

WORKDIR /app
COPY --from=builder /usr/src/cart-service/target/x86_64-unknown-linux-musl/release/cart-service .

USER appuser
EXPOSE 8080

CMD ["./cart-service"]
