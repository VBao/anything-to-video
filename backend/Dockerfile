# Use a Rust base image for building
FROM rust:1.80-slim-bullseye as builder

WORKDIR /app

# Create a blank project to cache dependencies
RUN cargo init
COPY Cargo.toml ./
RUN cargo build --release

# Copy the source code
COPY src ./src
# Touch main.rs to ensure rebuild
RUN touch src/main.rs
RUN cargo build --release

# Final image
FROM debian:bullseye-slim

# Install ffmpeg and ca-certificates and curl for healthcheck
RUN apt-get update && apt-get install -y \
    ffmpeg \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend

# Create temp uploads directory
RUN mkdir -p temp_uploads

EXPOSE 8080

ENV FFMPEG_PATH=ffmpeg

CMD ["./backend"]
