# Build stage
FROM rust:1.83-slim-bookworm as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/relay.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/securebeam-relay /app/securebeam-relay

# Create non-root user
RUN useradd -r -s /bin/false securebeam && \
    chown -R securebeam:securebeam /app

USER securebeam

EXPOSE 4001

ENV RUST_LOG=info
ENV RELAY_HOST=0.0.0.0
ENV RELAY_PORT=4001

CMD ["./securebeam-relay"]
