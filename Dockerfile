# syntax=docker/dockerfile:1
# One image with the server, the CLI, the built portal, and Mesa's lavapipe so model thumbnails render
# without a GPU. Build: docker build -t hoard .
# Blender and ffmpeg aren't in the image: switch them on in the portal's Settings and Hoard downloads them into the
# cache volume. The X and GL libraries below are what Blender's official Linux build needs, even run headless.
#
# Layers are ordered so CI's layer cache does the work: the npm install and the compiled Rust dependencies are
# their own layers, rebuilt only when package-lock.json or Cargo.lock/the manifests change, not on every code change.

FROM node:24-slim AS web
WORKDIR /web
COPY web/package.json web/package-lock.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

# The pinned compiler (rust-toolchain.toml) and cargo-chef, which change almost never.
FROM rust:1-slim-trixie AS chef
WORKDIR /src
COPY rust-toolchain.toml ./
RUN rustup show active-toolchain || rustup toolchain install
RUN cargo install cargo-chef --locked --version ^0.1

# The dependency list, from the manifests alone.
FROM chef AS plan
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
# xtask is a workspace member; it isn't built, but cargo needs its manifest to load the workspace.
COPY xtask ./xtask
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
# Every dependency, compiled: this layer stays cached until the recipe (Cargo.lock or a manifest) changes.
COPY --from=plan /src/recipe.json ./
RUN cargo chef cook --release --locked --recipe-path recipe.json -p hoardd -p hoard
COPY Cargo.toml Cargo.lock rustfmt.toml clippy.toml ./
COPY crates ./crates
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
 && mkdir -p /config /library /cache
COPY --from=build /src/target/release/hoardd /src/target/release/hoard /usr/local/bin/
COPY --from=web /web/dist /srv/hoard/web
# Fixes up the mounted folders' owners, then runs hoardd as an ordinary user (see the script).
COPY --chmod=755 deploy/entrypoint.sh /usr/local/bin/hoard-entrypoint
# For the NVIDIA Container Toolkit: which GPUs to pass in, and "graphics" so it mounts the Vulkan driver (the model
# thumbnail renderer's) and not only CUDA. CUDA base images set the first; this one isn't one, so without these
# `runtime: nvidia` passes nothing in. Ignored on hosts without the toolkit.
ENV NVIDIA_VISIBLE_DEVICES=all \
    NVIDIA_DRIVER_CAPABILITIES=graphics,compute,utility
ENV HOARD_LISTEN=0.0.0.0:4627 \
    HOARD_DATA_DIR=/config \
    HOARD_FILES_DIR=/library \
    HOARD_CACHE_DIR=/cache \
    HOARD_WEB_DIR=/srv/hoard/web
VOLUME /config
EXPOSE 4627
HEALTHCHECK CMD ["sh", "-c", "exec 3<>/dev/tcp/127.0.0.1/4627"]
ENTRYPOINT ["hoard-entrypoint"]
CMD ["hoardd"]
