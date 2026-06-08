FROM rust:1.82-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/api-gateway /usr/local/bin/api-gateway
COPY config.example.toml /app/config.toml
ENV CONFIG_PATH=/app/config.toml
ENV SERVER_ADDR=0.0.0.0:8000
EXPOSE 8000
CMD ["api-gateway"]
