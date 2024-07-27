# Start with the official Rust image
FROM rust:latest AS builder

# Install libclang and other necessary dependencies
RUN apt-get update && \
    apt-get install -y clang llvm-dev libclang-dev

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code and config directory
COPY src ./src
COPY config ./config

# Build the project in release mode
RUN cargo build --release

# Second stage: create a smaller image
FROM alpine:latest

# Install necessary libraries
RUN apk add --no-cache libgcc libstdc++

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/clutch-node ./

# Copy the config directory
COPY --from=builder /usr/src/app/config ./config

# Set the startup command to run the application with environment argument
ENTRYPOINT ["./clutch-node", "--env"]
CMD ["node1"]
