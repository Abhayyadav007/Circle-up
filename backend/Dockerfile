# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# Stage 1: build a release binary
# ---------------------------------------------------------------------------
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies first.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && echo '' > src/lib.rs \
    && cargo build --release || true
RUN rm -rf src

COPY . .
# `touch` so cargo notices the real sources are newer than the dummy build.
RUN touch src/main.rs && cargo build --release --bin circleup-backend

# ---------------------------------------------------------------------------
# Stage 2: minimal runtime image
# ---------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 10001 circleup

COPY --from=builder /app/target/release/circleup-backend /usr/local/bin/circleup-backend
COPY --from=builder /app/migrations ./migrations

USER circleup
EXPOSE 8080
ENV PORT=8080
CMD ["circleup-backend"]

# ---------------------------------------------------------------------------
# Stage 3: dev image with hot-reload (used by docker-compose `backend` service)
# ---------------------------------------------------------------------------
FROM rust:1-slim-bookworm AS dev
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-watch --locked

ENV PORT=8080
EXPOSE 8080
# Source is bind-mounted by compose; recompile + restart on change.
CMD ["cargo", "watch", "-x", "run --bin circleup-backend"]
