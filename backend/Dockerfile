# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy all crate sources
COPY domain ./domain
COPY presenter ./presenter
COPY db_domain ./db_domain
COPY repository ./repository
COPY repository_impl ./repository_impl
COPY feature ./feature
COPY usecase ./usecase
COPY controller ./controller
COPY di_service ./di_service
COPY app ./app

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/app /app/app

# Expose port
EXPOSE 8080

# Run the application
CMD ["/app/app"]
