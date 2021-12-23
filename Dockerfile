FROM rust:latest as builder
WORKDIR /build
COPY . .
# If you do not have docker buildkit installed, uncomment line 6 and coment line 7
#RUN cargo build --release; mv /build/target/release/xd_bot /xd_bot
RUN --mount=type=cache,target=./target cargo build --release; mv /build/target/release/xd_bot /xd_bot

FROM debian:buster-slim
WORKDIR /app
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /xd_bot /app/xd_bot
VOLUME ./config.toml /app/config.toml
CMD ["/app/xd_bot"]

