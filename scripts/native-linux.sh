#!/usr/bin/env bash
# Host-side launcher for the GNU/Linux native-entry support campaign (k49).
#
# The campaign itself is the same Taskfile/driver the Darwin host runs
# (task test:native-campaign); this script only provides the Linux environment
# it runs in: an image per platform, a container holding a copy of the working
# tree (never the host's target/), and the git objects the frozen incumbent
# revision is exported from. Nothing here measures anything.
#
# linux/arm64 runs natively on the Docker VM's kernel. linux/amd64 runs under
# user-mode emulation: that measurement is "emulated" and can show the code and
# instruments build and behave under that emulator; it cannot establish a native
# kernel's signal, process-group or PTY semantics.
#
# Why amd64 is not simply `docker run --platform linux/amd64`: Docker Desktop's
# registered QEMU (10.2.3) maps a 64-bit guest above 2^47 on this VM's 48-bit
# address space, and rustc's allocator then faults before doing any work (rustc
# 1.85.0 and 1.93.1 both, measured). scripts/release-smoke.sh records the same
# finding and its fix, a guest base of 2^47 through a private binfmt_misc
# instance, which scripts/native-linux-qemu.sh reuses. The pinned emulator is
# the one release-smoke.sh pins.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
context="$root/scripts/native-linux"

die() {
  printf 'native-linux: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat >&2 <<'USAGE'
usage: native-linux.sh image PLATFORM        build the subject image (arm64 | amd64)
       native-linux.sh up PLATFORM           start the container and stage the tree
       native-linux.sh sync PLATFORM         re-copy the working tree into it
       native-linux.sh exec PLATFORM CMD...  run CMD in /work as the unprivileged user
       native-linux.sh pull PLATFORM SRC DST copy SRC out of the container to DST
       native-linux.sh campaign PLATFORM OUT run the whole support measurement in a
                                             fresh environment, copying results to OUT
       native-linux.sh down PLATFORM         remove the container

PLATFORM is arm64 (native to the Docker VM) or amd64 (emulated).
USAGE
  exit 2
}

arch_of() {
  case "${1:-}" in
    arm64 | amd64) printf '%s' "$1" ;;
    *) usage ;;
  esac
}

# The git directory holding the objects of the incumbent revision. A jj
# secondary workspace's .git is a file naming the worktree's directory inside
# the common one; the common directory holds the objects.
git_common_dir() {
  git -C "$root" rev-parse --path-format=absolute --git-common-dir
}

name_of() { printf 'ravel-native-linux-%s' "$1"; }

cmd_image() {
  local arch
  arch="$(arch_of "${1:-}")"
  docker build --platform "linux/$arch" --build-arg "TARGETARCH=$arch" \
    -t "ravel-native-linux:$arch" "$context"
}

# Run a command in the measured environment as the unprivileged user, in /work.
# arm64: the container itself. amd64: the guest userland, entered through the
# helper's private namespace (native-linux-qemu.sh holds it). Extra docker-exec
# flags (-i) come first, separated from the command by --.
env_exec() {
  local arch="$1" name flags=()
  shift
  name="$(name_of "$arch")"
  while [ "${1:-}" != "--" ]; do
    flags+=("$1")
    shift
  done
  shift
  if [ "$arch" = arm64 ]; then
    docker exec "${flags[@]}" -w /work "$name" "$@"
  else
    docker exec "${flags[@]}" "$name" bash -c '
      exec nsenter --target "$(cat /run/native-qemu.pid)" --user --mount \
        --setuid 1000 --setgid 1000 /usr/bin/env -i \
        HOME=/home/ubuntu USER=ubuntu LOGNAME=ubuntu TERM=dumb \
        NATIVE_CAMPAIGN_EMULATION=qemu-user,-B=0x800000000000,-cpu='"$CPU"' \
        RUSTUP_HOME=/opt/rustup CARGO_HOME=/home/ubuntu/.cargo \
        PATH=/opt/cargo/bin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
        /bin/bash -c '\''cd /work && exec "$@"'\'' _ "$@"' _ "$@"
  fi
}

# The working tree as the campaign measures it: everything the driver and the
# tests read, none of the host's build output, dependency installs or tooling
# state, and none of the immutable Darwin receipts under docs/evidence (nothing
# the campaign reads; their read-only directories also cannot be re-extracted
# over). Ownership is reset on extraction so the container's git accepts it.
copy_tree() {
  local arch="$1"
  # COPYFILE_DISABLE stops macOS tar writing AppleDouble members;
  # --no-xattrs keeps its provenance xattrs out of the stream.
  (cd "$root" && COPYFILE_DISABLE=1 tar -c --no-xattrs \
    --exclude=./target --exclude=node_modules --exclude=./.git --exclude=./.jj \
    --exclude=./.grove --exclude=./docs/evidence --exclude=.DS_Store .) |
    env_exec "$arch" -i -- tar -x --no-same-owner -C /work
}

stage_git() {
  env_exec "$1" -- bash -c '
    set -e
    git config --global --add safe.directory "*"
    git config --global user.name native-linux
    git config --global user.email native-linux@localhost
    cd /work
    git init -q .
    echo /srv/git/objects > .git/objects/info/alternates'
}

# The container's security options, and why each is needed. Used by both routes
# (for amd64, by the helper that holds the emulated namespace).
#
# seccomp=unconfined: Docker's built-in profile refuses unshare(2) to a
# container without CAP_SYS_ADMIN, which the product's bubblewrap confinement
# needs. scripts/release-smoke.sh records the same finding and prefers lifting
# the filter to granting SYS_ADMIN. systempaths=unconfined is the second
# finding, measured here (k49): Docker masks parts of /proc, and the kernel
# refuses a new procfs mount inside a user namespace when the container's /proc
# is partly hidden ("Can't mount proc on /newroot/proc"), which bubblewrap's
# --proc needs. Neither option adds a capability: the container keeps Docker's
# default capability set.
#
# --init is the third: orphaned descendants are reparented to the container's
# PID 1, and `sleep` as PID 1 never reaps them, so a launcher that correctly
# killed and abandoned a group left unreaped zombies that every group-absence
# check then reads as "present". A host's init reaps; so must this one.
container_options() {
  printf '%s\n' --init --security-opt seccomp=unconfined --security-opt systempaths=unconfined
}

# The pinned x86-64 emulator and the options that make it a faithful guest, from
# release-smoke.sh (one source of truth for the digest).
qemu_image() {
  grep -oE 'docker\.io/tonistiigi/binfmt@sha256:[0-9a-f]{64}' "$root/scripts/release-smoke.sh" | head -n 1
}

# CPU model for the emulated guest: the x86-64 floor release-smoke.sh runs at.
readonly CPU=Nehalem

stage_qemu_inputs() {
  local inputs="$1" image container
  rm -rf "$inputs"
  mkdir -p "$inputs"
  image="$(qemu_image)"
  [ -n "$image" ] || die "no pinned QEMU image found in scripts/release-smoke.sh"
  container="$(docker create --platform linux/arm64 "$image" /bin/true)"
  docker cp --quiet "$container:/usr/bin/qemu-x86_64" "$inputs/qemu"
  docker rm "$container" >/dev/null
  container="$(docker create --platform linux/amd64 "ravel-native-linux:amd64" /bin/true)"
  docker export --output "$inputs/rootfs.tar" "$container"
  docker rm "$container" >/dev/null
  # The interpreter is a native helper-architecture executable; the arm64 image
  # carries the C compiler. tonistiigi's build takes argv[0] after the path.
  docker run --rm --platform linux/arm64 --user "$(id -u):$(id -g)" \
    --volume "$inputs:/out" --volume "$context:/src:ro" \
    "ravel-native-linux:arm64" cc -static -O2 \
    "-DQEMU_OPTIONS=\"-B\",\"0x800000000000\",\"-cpu\",\"$CPU\"," -DARGV0_OPTION=0 \
    -o /out/interpreter /src/qemu-interpreter.c
  docker run --rm --platform linux/arm64 --user "$(id -u):$(id -g)" \
    --volume "$inputs:/out" --volume "$context:/src:ro" \
    "ravel-native-linux:arm64" cc -static -O2 -o /out/pivot-root /src/pivot-root.c
  cp "$root/scripts/native-linux-qemu.sh" "$inputs/"
}

cmd_up() {
  local arch name common options=()
  arch="$(arch_of "${1:-}")"
  name="$(name_of "$arch")"
  common="$(git_common_dir)"
  docker image inspect "ravel-native-linux:$arch" >/dev/null 2>&1 ||
    die "no image for $arch (run: native-linux.sh image $arch)"
  docker rm -f "$name" >/dev/null 2>&1 || true
  while IFS= read -r option; do options+=("$option"); done < <(container_options)

  if [ "$arch" = arm64 ]; then
    docker run -d --name "$name" --platform linux/arm64 "${options[@]}" \
      --volume "$common:/srv/git:ro" \
      "ravel-native-linux:arm64" sleep infinity >/dev/null
  else
    docker image inspect "ravel-native-linux:arm64" >/dev/null 2>&1 ||
      die "the emulated route needs the arm64 image too (its compiler builds the interpreter)"
    local inputs="$root/target/native-linux-amd64-inputs"
    mkdir -p "$root/target"
    stage_qemu_inputs "$inputs"
    # The helper is Docker's own architecture, as root, holding the guest's
    # namespace; it carries the checkout's git objects for the guest to read.
    docker run -d --name "$name" --platform linux/arm64 --user root "${options[@]}" \
      --volume "$inputs:/inputs:ro" --volume "$common:/srv/git:ro" \
      "ravel-native-linux:arm64" sleep infinity >/dev/null
    docker exec -d "$name" bash -c \
      'exec bash /inputs/native-linux-qemu.sh serve /inputs "$0" >/run/native-qemu.log 2>&1' "$CPU"
    local waited=0
    until docker exec "$name" test -s /run/native-qemu.ready 2>/dev/null; do
      waited=$((waited + 1))
      if [ "$waited" -ge 600 ] || ! docker exec "$name" pgrep -f 'native-linux-qemu.sh' >/dev/null 2>&1; then
        docker exec "$name" cat /run/native-qemu.log >&2 || true
        die "the emulated environment did not become ready"
      fi
      sleep 1
    done
    docker exec "$name" cat /run/native-qemu.log
  fi
  copy_tree "$arch"
  stage_git "$arch"
  printf 'native-linux: %s is up\n' "$name"
}

cmd_sync() {
  local arch
  arch="$(arch_of "${1:-}")"
  copy_tree "$arch"
}

cmd_exec() {
  local arch
  arch="$(arch_of "${1:-}")"
  shift
  [ $# -ge 1 ] || usage
  env_exec "$arch" -- "$@"
}

cmd_pull() {
  local arch name prefix=""
  arch="$(arch_of "${1:-}")"
  name="$(name_of "$arch")"
  [ $# -eq 3 ] || usage
  [ ! -e "$3" ] || die "destination exists (one writer): $3"
  [ "$arch" = arm64 ] || prefix=/floor
  docker cp "$name:$prefix$2" "$3"
}

# One whole support measurement of one target, in a fresh environment, with its
# results copied out to OUT (which must not exist). The environment is left up so
# a failing run can be inspected; `down` removes it. A file over 1 MiB is
# compressed on the way out (the version control refuses to snapshot a larger
# new file), with its original SHA-256 recorded beside it.
cmd_campaign() {
  local arch name out prefix="" code=0
  arch="$(arch_of "${1:-}")"
  out="${2:-}"
  [ -n "$out" ] || usage
  [ ! -e "$out" ] || die "output exists (one writer, never reused): $out"
  name="$(name_of "$arch")"
  cmd_up "$arch"
  env_exec "$arch" -- bash scripts/native-linux/measure.sh /work/target/k49 || code=$?
  [ "$arch" = arm64 ] || prefix=/floor
  mkdir -p "$(dirname "$out")"
  docker cp --quiet "$name:$prefix/work/target/k49" "$out"
  {
    echo "== docker version"
    docker version --format 'client {{.Client.Version}}; server {{.Server.Version}} {{.Server.Os}}/{{.Server.Arch}}'
    echo "== docker vm kernel"
    docker info --format '{{.KernelVersion}} | {{.OperatingSystem}} | {{.Architecture}}'
    echo "== image"
    docker image inspect "ravel-native-linux:$arch" --format '{{.Id}} {{.Architecture}} {{.Created}}'
    echo "== base image"
    grep -E '^ARG BASE=' "$context/Dockerfile"
  } >"$out/docker.txt" 2>&1 || true
  # shellcheck disable=SC2312
  find "$out" -type f -size +1000000c | LC_ALL=C sort | while IFS= read -r file; do
    printf '%s  %s\n' "$(shasum -a 256 "$file" | cut -d' ' -f1)" "${file#"$out"/}" >>"$out/large-files.sha256"
    gzip -n -9 "$file"
  done
  return "$code"
}

cmd_down() {
  local arch
  arch="$(arch_of "${1:-}")"
  docker rm -f "$(name_of "$arch")" >/dev/null
  rm -rf "$root/target/native-linux-amd64-inputs"
}

[ $# -ge 1 ] || usage
mode="$1"
shift
case "$mode" in
  image) cmd_image "$@" ;;
  up) cmd_up "$@" ;;
  sync) cmd_sync "$@" ;;
  exec) cmd_exec "$@" ;;
  pull) cmd_pull "$@" ;;
  campaign) cmd_campaign "$@" ;;
  down) cmd_down "$@" ;;
  *) usage ;;
esac
