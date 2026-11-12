# syntax=docker/dockerfile:1
# One image with the server, the CLI, the built portal, and Mesa's lavapipe so model thumbnails render
# without a GPU. Build: docker build -t hoard .
# Blender and ffmpeg aren't in the image: switch them on in the portal's Settings and Hoard downloads them into the
# data volume. The X and GL libraries below are what Blender's official Linux build needs, even run headless.

FROM node:24-slim AS web
WORKDIR /web
COPY web/package.json web/package-lock.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

FROM rust:1-slim-trixie AS build
WORKDIR /src
# rust-toolchain.toml pins the exact compiler; rustup in the base image installs it on first use.
COPY rust-toolchain.toml Cargo.toml Cargo.lock rustfmt.toml clippy.toml ./
COPY crates ./crates
# xtask is a workspace member; it isn't built, but cargo needs its manifest to load the workspace.
COPY xtask ./xtask
COPY tools/unreal_export.py ./tools/unreal_export.py
# The version this build reports (CI passes the release tag's); empty means Cargo.toml's.
ARG HOARD_VERSION=
RUN cargo build --release --locked -p hoardd -p hoard

FROM debian:trixie-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates mesa-vulkan-drivers libvulkan1 fonts-dejavu-core timgm6mb-soundfont \
    libx11-6 libxi6 libxxf86vm1 libxfixes3 libxrender1 libxext6 libxkbcommon0 libsm6 libice6 libgl1 libegl1 \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --system --uid 10001 --home /data hoard && mkdir -p /data /files /cache && chown hoard /data /files /cache
COPY --from=build /src/target/release/hoardd /src/target/release/hoard /usr/local/bin/
COPY --from=web /web/dist /srv/hoard/web
ENV HOARD_LISTEN=0.0.0.0:7878 \
    HOARD_DATA_DIR=/data \
    HOARD_WEB_DIR=/srv/hoard/web
USER hoard
VOLUME /data
EXPOSE 7878
HEALTHCHECK CMD ["sh", "-c", "exec 3<>/dev/tcp/127.0.0.1/7878"]
CMD ["hoardd"]
