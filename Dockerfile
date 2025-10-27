FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release -p unitx-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/unitx-api /usr/local/bin/unitx-api

EXPOSE 8080
ENV RUST_LOG=unitx_api=info

CMD ["unitx-api"]