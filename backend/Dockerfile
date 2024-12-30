# Use the official Rust image as the base image
FROM rust:1.64 as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first, leveraging Docker's caching to avoid re-building dependencies if only source files have changed
COPY Cargo.toml Cargo.lock ./

# Build only dependencies initially to cache them
RUN cargo fetch --release

# Copy the application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a new, smaller image from a minimal base image (alpine or debian)
FROM debian:bullseye-slim

# Install necessary dependencies to run the application (if needed, like OpenSSL or others)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory for the runtime image
WORKDIR /usr/src/app

# Copy the built binary from the builder image to the runtime image
COPY --from=builder /usr/src/app/target/release/ai-zero-trust-threat-system .

# Set the entrypoint to run the built binary
CMD ["./ai-zero-trust-threat-system"]

# Ensure the binary has execute permissions in case Docker requires it (optional, but good practice)
RUN chmod +x ./ai-zero-trust-threat-system
