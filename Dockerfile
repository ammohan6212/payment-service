# ---------------------------
# Stage 1: Build
# ---------------------------
FROM rust:1.72 as builder

# Create app directory
WORKDIR /usr/src/cart-service

# Copy Cargo.toml and Cargo.lock separately to leverage caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to build dependencies first
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# ---------------------------
# Stage 2: Runtime
# ---------------------------
FROM debian:bullseye-slim

# Create non-root user for security
RUN useradd -m appuser

# Copy the binary from the builder stage
COPY --from=builder /usr/src/cart-service/target/release/cart-service /usr/local/bin/cart-service

# Expose port
EXPOSE 8080

# Switch to non-root user
USER appuser

# Run the service
CMD ["cart-service"]
