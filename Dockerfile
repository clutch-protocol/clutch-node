# Use a Rust base image
FROM rust:latest

# Install Clang
RUN apt-get update && \
    apt-get install -y clang && \
    rm -rf /var/lib/apt/lists/*  # Clean up to reduce image size

# SET ENV
ENV DB_PATH="/clutch-node-db"

# Set the working directory inside the container
WORKDIR /clutch-node

# First, copy only the files needed for compiling dependencies
COPY Cargo.toml ./

# Create a dummy main file to build and cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

# Copy the dependency manifest files
COPY . .

# Build dependencies to cache them
RUN cargo build

# Command to run when starting the container
CMD ["cargo", "run"]
