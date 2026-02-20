FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/robot-kit/Cargo.toml crates/robot-kit/Cargo.toml
RUN mkdir -p src benches crates/robot-kit/src \
    && echo "fn main() {}" > src/main.rs \
    && echo "fn main() {}" > benches/agent_benchmarks.rs \
    && echo "pub fn placeholder() {}" > crates/robot-kit/src/lib.rs
RUN cargo build --release --locked
RUN rm -rf src benches crates/robot-kit/src
COPY src/ src/
COPY benches/ benches/
COPY crates/ crates/
COPY firmware/ firmware/
RUN cargo build --release --locked \
    && cp target/release/zeroclaw /app/zeroclaw \
    && strip /app/zeroclaw
RUN mkdir -p /zeroclaw-data/.zeroclaw /zeroclaw-data/workspace \
    && printf '[gateway]\nport = 3000\nhost = "0.0.0.0"\nallow_public_bind = true\napi_key = ""\ndefault_provider = "openrouter"\n' \
       > /zeroclaw-data/.zeroclaw/config.toml \
    && chown -R 65534:65534 /zeroclaw-data

FROM gcr.io/distroless/cc-debian12:nonroot AS prod
COPY --from=builder --chown=65534:65534 /zeroclaw-data /zeroclaw-data
COPY --from=builder /app/zeroclaw /usr/local/bin/zeroclaw
USER nonroot:nonroot
WORKDIR /zeroclaw-data/workspace
EXPOSE 3000
CMD ["zeroclaw", "gateway"]