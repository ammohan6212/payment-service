# Use the official Rust image as builder
FROM rust:1.77 as builder

# Create a new directory inside container
WORKDIR /usr/src/payment_service

# Copy the Cargo.toml and Cargo.lock files first (for caching dependencies)
COPY Cargo.toml .
COPY Cargo.lock .

# Create an empty src directory to trick cargo into fetching dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Pre-fetch dependencies (this layer will be cached)
RUN cargo build --release
RUN rm -f src/main.rs

# Now copy the actual source code
COPY src ./src

# Build the actual app
RUN cargo build --release

# Use a minimal base image for the final container
FROM debian:buster-slim

# Install required system dependencies (if needed by Actix)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /usr/src/payment_service/target/release/payment_service /usr/local/bin/payment_service

# Expose port 8080
EXPOSE 8080

# Run the binary
CMD ["payment_service"]
