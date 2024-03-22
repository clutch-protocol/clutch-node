# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/clutch-node

# Copy the dependency manifest files
COPY . .

# Build dependencies to cache them
RUN cargo build --release

# Command to run tests
CMD ["cargo", "test"]
