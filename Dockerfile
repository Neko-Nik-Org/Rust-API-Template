# Stage 1: Build
FROM rust:1.83.0-bookworm AS build

WORKDIR /app

# Cache dependencies by copying only Cargo files first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// placeholder" > src/lib.rs
RUN cargo build --release

# Copy the actual source code and rebuild
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:12-slim

# Install necessary runtime dependencies
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates
RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*

# Create a directory for the binary
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=build /app/target/release/rust-api .

# Set the default command to run the binary
CMD ["/app/rust-api"]
