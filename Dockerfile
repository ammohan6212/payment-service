# Stage 1: build the Rust app
FROM rust:1.77 as builder

# Create app directory inside the container
WORKDIR /usr/src/payment_service

# Copy the full project (Cargo.toml + Cargo.lock + src folder)
COPY . .

# Build the app in release mode
RUN cargo build --release

# Stage 2: create minimal runtime image
FROM debian:buster-slim

# Install certificates (needed for HTTPS etc.)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /usr/src/payment_service/target/release/payment_service /usr/local/bin/payment_service

# Expose port 8080
EXPOSE 8080

# Command to run the app
CMD ["payment_service"]
