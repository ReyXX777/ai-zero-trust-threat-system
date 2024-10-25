# Use the official Rust image as the base image
FROM rust:1.64

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first, leveraging Docker's caching to avoid re-building dependencies if only source files have changed
COPY Cargo.toml Cargo.lock ./

# Build only dependencies initially to cache them
RUN cargo build --release || true

# Copy the application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Set the entrypoint to run the built binary
CMD ["./target/release/ai-zero-trust-threat-system"]

# Ensure the binary has execute permissions in case Docker requires it
RUN chmod +x ./target/release/ai-zero-trust-threat-system
