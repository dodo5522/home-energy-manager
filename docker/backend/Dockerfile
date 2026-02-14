FROM rust:1.90.0-trixie as builder
WORKDIR /tmp/backend
COPY ./crates ./crates
COPY ./src ./src
COPY ./Cargo.toml ./
COPY ./rust-toolchain.toml ./
RUN cargo build --release

FROM debian:trixie-20251020-slim
COPY --from=builder /tmp/backend/target/release/home-energy-manager /usr/local/bin/home-energy-manager
CMD ["home-energy-manager"]
