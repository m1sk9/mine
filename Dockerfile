FROM rust:1.81.0-bookworm as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin ssv2

FROM debian:bookworm-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/ssv2 /usr/local/bin/ssv2

RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group ssv2
USER ssv2
WORKDIR /home/ssv2

LABEL org.opencontainers.image.source=https://github.com/SpaceServerUniverse/ServerStatusDiscordBotV2

ENTRYPOINT [ "sh", "-c", "ssv2" ]
