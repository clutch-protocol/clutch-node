# Start with the official Rust image as the builder stage
FROM rust:latest AS builder

# Install libclang and other necessary dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends clang llvm-dev libclang-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock files separately to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dependencies
RUN cargo fetch

# Copy the source code and config directory
COPY src ./src
COPY config ./config

# Build the project in release mode
RUN cargo build --release && \
    strip target/release/clutch-node

# Second stage: create a smaller image
FROM alpine:latest

# Install necessary libraries
RUN apk add --no-cache libgcc libstdc++

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the binary and config directory from the builder stage
COPY --from=builder /usr/src/app/target/release/clutch-node ./
COPY --from=builder /usr/src/app/config ./config

# Set the startup command to run the application with environment argument
ENTRYPOINT ["./clutch-node", "--env"]
CMD ["node1"]
