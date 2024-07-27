# Start with the official Rust image
FROM rust:latest

# Install libclang and other necessary dependencies
RUN apt-get update && \
    apt-get install -y clang llvm-dev libclang-dev

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY config ./config

# Build dependencies to create the Cargo.lock file
RUN cargo build --release

# Set the startup command to run the application with environment argument
ENTRYPOINT ["./target/release/clutch-node", "--env"]
CMD ["node1"]
