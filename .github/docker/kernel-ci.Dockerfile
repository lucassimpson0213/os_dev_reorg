FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# ---- Base tooling + your build deps ----
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    jq \
    build-essential \
    nasm \
    xorriso \
    mtools \
    qemu-system-x86 \
    grub-pc-bin \
    grub-common \
    gcc-i686-linux-gnu \
    binutils-i686-linux-gnu \
  && rm -rf /var/lib/apt/lists/*

# ---- GitHub CLI (gh) ----
RUN type -p curl >/dev/null \
  && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
     | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
  && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
  && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
     > /etc/apt/sources.list.d/github-cli.list \
  && apt-get update \
  && apt-get install -y --no-install-recommends gh \
  && rm -rf /var/lib/apt/lists/*

# ---- Rust nightly + rust-src ----
# Pin nightly for reproducible builds; bump this when you want.
ARG RUST_NIGHTLY=nightly-2026-02-01
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain ${RUST_NIGHTLY} \
  && /root/.cargo/bin/rustup component add rust-src --toolchain ${RUST_NIGHTLY}

ENV PATH="/root/.cargo/bin:${PATH}"

# ---- grub2-mkrescue shim (Ubuntu often has grub-mkrescue) ----
RUN if ! command -v grub2-mkrescue >/dev/null 2>&1 && command -v grub-mkrescue >/dev/null 2>&1; then \
      ln -s "$(command -v grub-mkrescue)" /usr/local/bin/grub2-mkrescue; \
    fi

WORKDIR /work
