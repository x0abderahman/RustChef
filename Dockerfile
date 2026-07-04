# Stage 1: Build
FROM rust:1.96-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update -qq && apt-get install -y -qq \
    libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/chef /usr/local/bin/chef

ENTRYPOINT ["chef"]
CMD ["--help"]
