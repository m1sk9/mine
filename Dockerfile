FROM rust:1.81.0-bookworm as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin mine

FROM debian:bookworm-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/mine /usr/local/bin/mine

RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group mine
USER mine
WORKDIR /home/mine

LABEL org.opencontainers.image.source=https://github.com/m1sk9/mine

ENTRYPOINT [ "sh", "-c", "mine" ]
