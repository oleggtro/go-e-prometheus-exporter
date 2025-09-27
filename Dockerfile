FROM docker.io/library/alpine:latest AS builder
WORKDIR /app
RUN apk add build-base cargo
#RUN apk add --no-cache build-base cargo openssl-dev
COPY Cargo.toml Cargo.lock ./
COPY src src/

RUN cargo build --release

# --------- Runtime Stage ---------
FROM debian:buster-slim

# Set the binary name here if it's different
ENV BIN_NAME=goe-prometheus-exporter

# Create non-root user (optional, improves security)
RUN adduser --disabled-password --gecos '' appuser

# Copy the binary from the build stage
COPY --from=builder /app/target/release/$BIN_NAME /usr/local/bin/$BIN_NAME

# Change to non-root user (optional)
USER appuser

# Run your application
CMD ["myapp"]
