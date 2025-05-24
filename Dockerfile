# ---------------------------
# Stage 1: Build (MUSL target)
# ---------------------------
FROM rust:1.82 as builder

# Install musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/cart-service

# Copy manifest files and fetch dependencies first
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

# Copy source code and build real binary
COPY src/ ./src/
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---------------------------
# Stage 2: Runtime (Alpine)
# ---------------------------
FROM alpine:latest

# Create a non-root user
RUN adduser -D appuser

# Create working directory
WORKDIR /app

# Copy the statically linked binary from builder
COPY --from=builder /usr/src/cart-service/target/x86_64-unknown-linux-musl/release/cart-service .

# Set permissions and expose port
USER appuser
EXPOSE 8080

# Run the service
CMD ["./cart-service"]
