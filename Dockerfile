# Stage 1: Build
FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/payment_service

RUN apt-get update && apt-get install -y libpq-dev pkg-config

COPY . .

RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Use upgrade to pull latest security fixes
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    ca-certificates \
    libpq5 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/payment_service/target/release/payment_service /usr/local/bin/payment_service

EXPOSE 8081

CMD ["payment_service"]
