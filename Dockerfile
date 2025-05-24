# ---------------------------
# Stage 1: Build
# ---------------------------
FROM rust:1.82 AS builder


WORKDIR /usr/src/cart-service

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY src/ ./src/

# Build the real binary
RUN cargo build --release

# ---------------------------
# Stage 2: Runtime
# ---------------------------
FROM debian:bullseye-slim

RUN useradd -m appuser

WORKDIR /app
COPY --from=builder /usr/src/cart-service/target/release/cart-service .

EXPOSE 8080

USER appuser
CMD ["./cart-service"]
