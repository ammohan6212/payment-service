# Stage 1: Build the Rust application
FROM rust:1.83 AS builder

# Create app directory inside the container
WORKDIR /usr/src/payment_service

# Copy entire project (Cargo.toml, Cargo.lock, src/, etc.)
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install SSL certificates for HTTPS support
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /usr/src/payment_service/target/release/payment_service /usr/local/bin/payment_service

# Expose the application port
EXPOSE 8000

# Set environment variables if needed (optional)
# ENV DATABASE_URL=mysql://user:password@host:port/dbname

# Run the compiled binary
CMD ["payment_service"]
