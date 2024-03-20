FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /app
RUN rustup default nightly && rustup toolchain remove stable

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo +nightly chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo +nightly build --release --bin exchangeapp

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/exchangeapp .
CMD ["./exchangeapp --help"]