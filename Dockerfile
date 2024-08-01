# Start with the official Rust image as the builder stage
FROM rust:1.80.0

# Install dependencies required for building librocksdb-sys
RUN apt-get update && apt-get install -y clang

# Set the working directory inside the container
WORKDIR /usr/src/clutch-node

# Copy Cargo.toml and Cargo.lock files separately to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo fetch

# Copy the source code and config directory
COPY src ./src
COPY config ./config

# Build the project in release mode
RUN cargo build --release

# Set the command to run the release binary
CMD ["./target/release/clutch-node", "--env", "node1"]