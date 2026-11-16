# Wadhu Connect → Cloud Run (us-central1).
# From workspace root:
#   docker build -f apps/wadhu-connect/Dockerfile -t wadhu-connect .
FROM rust:bookworm AS build
WORKDIR /src

COPY apps/wadhu-connect/docker/Cargo.workspace.toml ./Cargo.toml
COPY apps/wadhu-connect ./apps/wadhu-connect
# include_str! legal Markdown (same files as wadhu.com)
COPY docs/legal ./docs/legal

RUN sed -i \
  -e 's/edition.workspace = true/edition = "2024"/' \
  -e 's/license.workspace = true/license = "MIT OR Apache-2.0"/' \
  -e 's/authors.workspace = true/authors = ["Gotigin"]/' \
  apps/wadhu-connect/Cargo.toml \
  && cargo build --release -p wadhu-connect

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /src/target/release/wadhu-connect /app/wadhu-connect
COPY apps/wadhu-connect/public /app/public
ENV WADHU_CONNECT_ROOT=/app \
    PUBLIC_DIR=/app/public \
    DATA_DIR=/data \
    HOST=0.0.0.0 \
    PORT=8080 \
    PUBLIC_ORIGIN=https://connect.wadhu.com \
    META_GRAPH_VERSION=v27.0
EXPOSE 8080
USER nobody
CMD ["/app/wadhu-connect"]
