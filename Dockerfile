# syntax=docker/dockerfile:1.6

###############################################
# Build stage
###############################################
FROM rust:1.76-bullseye AS builder

ENV CARGO_TERM_COLOR=always \
    RUSTFLAGS="-C target-cpu=native"

RUN apt-get update \
 && apt-get install --yes --no-install-recommends \
        pkg-config \
        libssl-dev \
        protobuf-compiler \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --workspace

# Collect release binaries (if any) into a predictable directory
RUN mkdir -p /artifacts \
 && find target/release -maxdepth 1 -type f -executable -exec cp {} /artifacts \;

###############################################
# Runtime stage
###############################################
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install --yes --no-install-recommends \
        ca-certificates \
        libssl3 \
        tzdata \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /srv/app

# Copy binaries and migrations generated during the build stage
COPY --from=builder /artifacts/ ./bin/
COPY --from=builder /app/crates/persistence/migrations ./migrations

# Lightweight entrypoint wrapper that allows the service binary to be overridden.
COPY <<'EOS' /usr/local/bin/entrypoint.sh
#!/bin/sh
set -euo pipefail

BIN_NAME="${SERVICE_BIN:-hh-tonic-service}"
BIN_PATH="/srv/app/bin/${BIN_NAME}"

if [ ! -x "${BIN_PATH}" ]; then
    echo "error: expected service binary '${BIN_NAME}' to be present at ${BIN_PATH}" >&2
    echo "available binaries:" >&2
    ls -1 /srv/app/bin >&2 || true
    exit 1
fi

exec "${BIN_PATH}"
EOS

RUN chmod +x /usr/local/bin/entrypoint.sh

ENV SERVICE_BIN=hh-tonic-service \
    PERSISTENCE__MIGRATIONS_DIR=/srv/app/migrations

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Pass through any extra arguments to the service binary.
CMD []
