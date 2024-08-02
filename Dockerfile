# Start with the official Rust image as the builder stage
FROM rust:1.80.0 AS builder

# Install dependencies required for building librocksdb-sys
RUN apt-get update && apt-get install -y clang

# Set the working directory inside the container
WORKDIR /usr/src/clutch-node

# Copy Cargo.toml and Cargo.lock files separately to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# This step will create a dummy main.rs, and build the dependencies.
# This will leverage the caching of Docker layers, so subsequent builds will be faster.
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove the dummy main.rs file
RUN rm -f src/main.rs

# Copy the actual source code
COPY src ./src

# Build the project in release mode
RUN cargo build --release

# Create a new stage with a smaller base image
FROM ubuntu:24.04

# Install required libraries to run the binary
RUN apt-get update && apt-get install -y libclang-dev libc6 libstdc++6 && apt-get clean

# Set the working directory inside the container
WORKDIR /usr/src/clutch-node

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/clutch-node/target/release/clutch-node .

# Copy the configuration file
COPY config ./config

# Set the command to run the release binary
ENTRYPOINT ["./clutch-node", "--env"]
CMD ["node1"]
