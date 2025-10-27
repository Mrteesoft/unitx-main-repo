### Build stage
FROM rust:1.81 AS builder
WORKDIR /usr/src/unitx

# Pre-copy manifests for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY crates/unitx-core/Cargo.toml crates/unitx-core/Cargo.toml
COPY crates/unitx-api/Cargo.toml crates/unitx-api/Cargo.toml
COPY crates/unitx-core/README.md crates/unitx-core/README.md
COPY crates/unitx-api/README.md crates/unitx-api/README.md

RUN cargo fetch --locked

# Copy the full workspace
COPY . .

RUN cargo build --release -p unitx-api

### Runtime stage
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /usr/src/unitx/target/release/unitx-api /usr/local/bin/unitx-api

ENV RUST_LOG=unitx_api=info
EXPOSE 8080

CMD ["/usr/local/bin/unitx-api"]
