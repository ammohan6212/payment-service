# Stage 1: build with nightly Rust
FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/payment_service

COPY . .

# Build with nightly
RUN cargo build --release

# Stage 2: minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/payment_service/target/release/payment_service /usr/local/bin/payment_service

EXPOSE 8080

CMD ["payment_service"]
