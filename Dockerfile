FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release
VOLUME ./target /build/target

FROM debian:buster-slim
WORKDIR /app
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/xd_bot /app/xd_bot
CMD ["/app/xd_bot"]
