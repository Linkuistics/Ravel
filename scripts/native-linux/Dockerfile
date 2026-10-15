# The GNU/Linux subject environment for the native-entry support campaign (k49).
#
# One image per platform: linux/arm64 runs natively on the Docker VM's kernel;
# linux/amd64 runs under the VM's user-mode emulator and is recorded as emulated.
# The base is pinned by digest and the toolchain by exact version; every installed
# tool's SHA-256 is written to /opt/native-linux/tools.txt so the campaign's
# domain record can name what it measured with.
#
# The declared minimum Rust is 1.85 (rust-version in Cargo.toml), so the default
# toolchain is exactly 1.85.0, the first release that satisfies it.
ARG BASE=ubuntu@sha256:f144425ff09be612d6d9ad965196e9cdc23dae1f42110a8a11a3e9a8198759f7
FROM ${BASE}

ARG RUST_VERSION=1.85.0
ARG NEXTEST_VERSION=0.9.148
ARG TASK_VERSION=3.54.0
ARG TARGETARCH

# CARGO_HOME is the unprivileged user's (the registry cache is written there);
# the installed toolchain and nextest live under /opt and are read-only to it.
ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/home/ubuntu/.cargo \
    PATH=/opt/cargo/bin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# The packages the build, the instruments and the Taskfile tasks themselves use:
# a C linker, git/patch/perl/tar for the incumbent assembly, jq and shasum for the
# driver, shellcheck for `task check`, procps for process observation, curl for
# the toolchain fetches below, and unzip/xz for the pinned Bun runtime fetch.
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      bubblewrap build-essential ca-certificates curl git jq patch perl procps \
      python3 shellcheck unzip util-linux xz-utils bash coreutils findutils \
 && rm -rf /var/lib/apt/lists/*

# No installed tool is executed here: the amd64 image is built under Docker's
# default user-mode emulator, which cannot run rustc (see native-linux.sh), and
# the campaign records each tool's own version where it measures.
RUN set -eux; \
    case "$TARGETARCH" in \
      arm64) rust_host=aarch64-unknown-linux-gnu; task=arm64 ;; \
      amd64) rust_host=x86_64-unknown-linux-gnu;  task=amd64 ;; \
      *) echo "unsupported TARGETARCH: $TARGETARCH" >&2; exit 1 ;; \
    esac; \
    curl --proto '=https' --tlsv1.2 -fsSL "https://static.rust-lang.org/rustup/dist/${rust_host}/rustup-init" -o /tmp/rustup-init; \
    chmod +x /tmp/rustup-init; \
    CARGO_HOME=/opt/cargo /tmp/rustup-init -y --no-modify-path --profile minimal \
      --default-toolchain "${RUST_VERSION}" --default-host "${rust_host}" \
      -c clippy -c rustfmt; \
    rm /tmp/rustup-init; \
    curl --proto '=https' --tlsv1.2 -fsSL \
      "https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${NEXTEST_VERSION}/cargo-nextest-${NEXTEST_VERSION}-${rust_host}.tar.gz" \
      | tar -xzf - -C /opt/cargo/bin; \
    curl --proto '=https' --tlsv1.2 -fsSL \
      "https://github.com/go-task/task/releases/download/v${TASK_VERSION}/task_linux_${task}.tar.gz" \
      | tar -xzf - -C /usr/local/bin task; \
    mkdir -p /opt/native-linux; \
    toolchain="/opt/rustup/toolchains/${RUST_VERSION}-${rust_host}"; \
    { for tool in "$toolchain/bin/rustc" "$toolchain/bin/cargo" "$toolchain/bin/cargo-clippy" \
                  "$toolchain/bin/rustfmt" /opt/cargo/bin/cargo-nextest \
                  /usr/local/bin/task /usr/bin/jq /usr/bin/shellcheck /usr/bin/bwrap; do \
        printf '%s  %s\n' "$(sha256sum "$tool" | cut -d' ' -f1)" "$tool"; \
      done; } >/opt/native-linux/tools.txt; \
    cat /opt/native-linux/tools.txt

# The product's filesystem confinement and several protected checks behave
# differently for root (permission bits are not enforced), so the campaign runs
# as an ordinary user, as release-smoke's floor container does (--user 1000:1000).
RUN id -u ubuntu | grep -qx 1000 \
 && install -d -o ubuntu -g ubuntu /work /home/ubuntu/.cargo
USER ubuntu
WORKDIR /work
