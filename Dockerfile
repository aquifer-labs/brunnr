# SPDX-License-Identifier: Apache-2.0

FROM rust:trixie AS builder

WORKDIR /src
COPY . .
RUN cargo build --release -p brunnr-cli --features qdrant --bins

FROM debian:trixie-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libstdc++6 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --create-home --home-dir /var/lib/brunnr --shell /usr/sbin/nologin brunnr \
    && mkdir -p /data \
    && chown brunnr:brunnr /data
WORKDIR /data

COPY --from=builder /src/target/release/brunnr /usr/local/bin/brunnr
COPY --from=builder /src/target/release/brunnrd /usr/local/bin/brunnrd

USER brunnr
VOLUME ["/data"]
ENTRYPOINT ["brunnrd"]
CMD ["--config", "/data/brunnr.toml", "--root", "/data"]
