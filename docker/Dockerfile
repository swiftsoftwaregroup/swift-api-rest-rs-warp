# Use the official Rust image as a parent image
FROM rust:1.80 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Install SQLite
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev

# Build the application
RUN cargo build --release

# Start a new stage with a minimal image
FROM debian:bookworm-slim

# Install SQLite runtime
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-0 && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/swift-api-rest-rs /usr/local/bin/

# Set the startup command to run your binary
CMD ["swift-api-rest-rs"]