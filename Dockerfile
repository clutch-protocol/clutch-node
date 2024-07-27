# Start with the official Rust image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml file
COPY Cargo.toml ./

# This step will build the dependencies and create the Cargo.lock file
RUN cargo build --release

# Copy the source code
COPY src ./src

# Copy the config directory
COPY config ./config

# Build the application
RUN cargo build --release

# Set the startup command to run the application with environment argument
ENTRYPOINT ["./target/release/clutch-node", "--env"]
CMD ["node1"]
