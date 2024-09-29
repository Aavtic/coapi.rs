# Stage 1: Python base image
FROM python:3.10-slim as python-base

# Stage 2: Rust build stage
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /rust-app

# Copy the Cargo.toml and Cargo.lock files first to build dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src/

# Build the Rust project in release mode
RUN cargo build --release

# Stage 3: Final stage with Debian base and Python environment
FROM debian:bullseye-slim

# Update and install Python
RUN apt-get update && apt-get install -y python3 && apt-get clean

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /rust-app/target/release/coapi_rs /usr/local/bin/rust-app

EXPOSE 8081

# Set the entrypoint to run your Rust application
CMD ["/usr/local/bin/rust-app"]

