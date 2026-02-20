# syntax=docker/dockerfile:1.7

# ── Stage 1: Build ──────────────────────────────────────────────────────
FROM rust:1.85-slim AS builder

WORKDIR /app

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 1. Copy manifests and toolchain pin to cache dependencies with the same compiler
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/robot-kit/Cargo.toml crates/robot-kit/Cargo.toml
# Create dummy targets declared in Cargo.toml so manifest parsing succeeds.
RUN mkdir -p src benches crates/robot-kit/src \
    && echo "fn main() {}" > src/main.rs \
    && echo "fn main() {}" > benches/agent_benchmarks.rs \
    && echo "pub fn placeholder() {}" > crates/robot-kit/src/lib.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --locked
RUN rm -rf src benches crates/robot-kit/src

# 2. Copy only build-relevant source paths
COPY src/ src/
COPY benches/ benches/
COPY crates/ crates/
COPY firmware/ firmware/
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release --locked && \
    cp target/release/zeroclaw /app/zeroclaw && \
    strip /app/zeroclaw

# Prepare runtime directory and default config
RUN mkdir -p /zeroclaw-data/.zeroclaw /zeroclaw-data/workspace && \
    printf '[gateway]\nport = 3000\nhost = "0.0.0.0"\nallow_public_bind = true\n\napi_key = ""\ndefault_provider = "openrouter"\ndefault_model = "anthropic/claude-sonnet-4-20250514"\ndefault_temperature = 0.7\nworkspace_dir = "/zeroclaw-data/workspace"\nconfig_path = "/zeroclaw-data/.zeroclaw/config.toml"\n' > /zeroclaw-data/.zeroclaw/config.toml && \
    chown -R 65534:65534 /zeroclaw-data

# ── Stage 2: Development Runtime (Debian) ──────────────────────
FROM debian:trixie-slim AS dev

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /zeroclaw-data /zeroclaw-data
COPY --from=builder /app/zeroclaw /usr/local/bin/zeroclaw

USER 65534:65534
WORKDIR /zeroclaw-data/workspace

EXPOSE 3000
CMD ["zeroclaw", "gateway"]

# ── Stage 3: Production Runtime (Distroless) ───────────────────────────────
FROM gcr.io/distroless/cc-debian12:nonroot AS prod

COPY --from=builder --chown=65534:65534 /zeroclaw-data /zeroclaw-data
COPY --from=builder /app/zeroclaw /usr/local/bin/zeroclaw

USER 65534:65534
WORKDIR /zeroclaw-data/workspace

EXPOSE 3000
CMD ["/usr/local/bin/zeroclaw", "gateway"]
