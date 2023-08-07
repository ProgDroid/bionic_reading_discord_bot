FROM lukemathwalker/cargo-chef:latest-rust-1.71-slim-buster as chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
RUN apt update && apt install --yes ca-certificates openssl libssl-dev pkg-config
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

ARG environment=local

COPY . .
RUN if [ "$environment" = "cloud" ]; then cargo build --release --bin bionic_reading_discord_bot --features cloud; else cargo build --release --bin bionic_reading_discord_bot; fi

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt update && apt install --yes ca-certificates

COPY --from=builder /app/target/release/bionic_reading_discord_bot bionic_reading_discord_bot
ENTRYPOINT ["./bionic_reading_discord_bot"]
