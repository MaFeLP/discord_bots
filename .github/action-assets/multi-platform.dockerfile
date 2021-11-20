ARG docker_arch=linux/amd64
ARG binary_name=x86_64-unknown-linux-gnu-discord_bots
FROM --platform=${docker_arch} ubuntu:latest
WORKDIR /app
COPY ./${binay_name} /app/xd_bot
CMD ["/app/xd_bot"]
