# Multi-stage build for optimized image size
# Build arguments for flexibility
ARG RUST_VERSION=1.76
ARG ALPINE_VERSION=3.19

#==============================================================================
# Builder Stage - Use Alpine for smaller base image
#==============================================================================
FROM rust:${RUST_VERSION}-alpine AS builder

# Install build dependencies in a single layer
RUN apk add --no-cache \
    musl-dev \
    clang17-dev \
    clang17 \
    llvm17-dev \
    llvm17-static \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    linux-headers \
    build-base \
    && rm -rf /var/cache/apk/*

# Set build environment for static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV CC=clang-17
ENV CXX=clang++-17
ENV LIBCLANG_PATH="/usr/lib/llvm17/lib"
ENV BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include/linux"

# Create app user for security
RUN addgroup -g 1000 clutch && \
    adduser -D -s /bin/sh -u 1000 -G clutch clutch

WORKDIR /usr/src/clutch-node

# Copy dependency files for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source and build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the final binary
RUN cargo build --release --target x86_64-unknown-linux-musl --bin clutch-node

# Strip the binary to reduce size further
RUN strip target/x86_64-unknown-linux-musl/release/clutch-node

#==============================================================================
# Runtime Stage - Minimal Alpine image
#==============================================================================
FROM alpine:${ALPINE_VERSION}

# Install only essential runtime dependencies
RUN apk add --no-cache ca-certificates tzdata && \
    rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -g 1000 clutch && \
    adduser -D -s /bin/sh -u 1000 -G clutch clutch

# Create directories with proper permissions
RUN mkdir -p /usr/local/bin /app/config && \
    chown -R clutch:clutch /app

# Copy the optimized binary
COPY --from=builder /usr/src/clutch-node/target/x86_64-unknown-linux-musl/release/clutch-node /usr/local/bin/clutch-node

# Set permissions and switch to non-root user
RUN chmod +x /usr/local/bin/clutch-node
USER clutch

# Set working directory
WORKDIR /app

# Health check for container monitoring
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD clutch-node --version || exit 1

# Expose default port (configurable via environment)
EXPOSE 8081

# Set the entrypoint and default command
ENTRYPOINT ["clutch-node"]
CMD ["--env", "default"]
