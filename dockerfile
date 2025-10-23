# Use the official Rust image as the build environment
FROM rust:1.81 as builder

# Set the working directory
WORKDIR /app

# Copy Cargo manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base runtime image
FROM debian:bookworm-slim

# Install SSL certificates for HTTPS support
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory for runtime
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/app /app/app

# Expose the port the Actix server will run on
EXPOSE 8080

# Set environment variables to configure IP and port for the server
ENV HOST=0.0.0.0
ENV PORT=8080

# Run the application
CMD ["/app/app"]

