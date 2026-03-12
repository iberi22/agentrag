# Multi-stage build for Cortex
# Stage 1: Builder
FROM rust:1.75-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy source
COPY . .

# Build release
RUN cargo build --release --bin cortex

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 cortex && \
    mkdir -p /data && \
    chown -R cortex:cortex /data

# Copy binary from builder
COPY --from=builder /app/target/release/cortex /usr/local/bin/

# Copy data directory
RUN mkdir -p /app/data

# Switch to non-root user
USER cortex

# Environment variables
ENV CORTEX_PORT=8003
ENV CORTEX_HOST=0.0.0.0
ENV RUST_LOG=info

# Expose port
EXPOSE 8003

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8003/health || exit 1

# Volume for data persistence
VOLUME ["/data"]

# Run cortex
ENTRYPOINT ["cortex"]
CMD ["serve", "--host", "0.0.0.0", "--port", "8003"]
