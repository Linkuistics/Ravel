#!/usr/bin/env bash
#
# The installed-layout smoke test of the release archives: each target's
# archive, extracted where that target runs and exercised with no Bun or Node
# on PATH.
#
#   scripts/release-smoke.sh [--archives DIR] [TARGET...]
#
# DIR holds one ravel-v<version>-<target>.tar.xz per target; the default is
# target/archives, which `task release:archives` fills. The default targets are
# every release target. `task release:smoke` rebuilds the archives from the
# working copy first, so it never reads a stale one; the release task runs this
# over target/dist before it publishes anything. Each archive must first match
# release-common.sh's archive manifest, as the build checked it.
#
#   aarch64-apple-darwin      natively. It needs a macOS arm64 host.
#   *-unknown-linux-gnu       in a glibc-2.17 userland of the target's
#                             architecture, under a pinned user-mode QEMU
#                             emulating the target's CPU floor (WHY A CPU
#                             MODEL, below). A target native to Docker also
#                             runs in a container of that userland first. It
#                             needs an arm64 Docker, and Zig, which builds the
#                             controls' probes and QEMU's interpreter; Docker
#                             on any other architecture is refused.
#
# scripts/release-smoke-target.sh runs inside each environment: it asserts the
# environment, extracts the archive with that environment's tar into a fresh
# prefix, checks the versions, and runs ravel's installed-layout
# cases, crates/ravel/scripts/installed-smoke.sh.
#
# WHY A CONTROL. A userland that is not really at the floor passes everything a
# floor userland passes. So each Linux run also executes a pair of probes built
# from one C source: one against glibc 2.17, which must run, and one that calls
# getrandom against glibc 2.25, which the userland must refuse for its
# GLIBC_2.25 symbol version. The first shows the userland runs binaries of that
# architecture at all; the refusal of the second then shows it enforces 2.17.
# The observed refusal is printed on every run.
#
# WHY OUR OWN QEMU. Docker Desktop on Apple silicon emulates amd64 with QEMU
# 8.1.5, which gives x86-64 guests no vDSO, and CentOS 7's glibc then crashes
# (docs/runtime-evidence.md, *Installed
# smoke*). QEMU 10.2.3 supplies the vDSO, but with no reserved address space it
# bounds a 64-bit guest by nothing, so on an arm64 kernel with 48-bit addresses
# it maps the guest above 2^47, where no x86-64 kernel maps a process; the
# worker's JavaScriptCore heap then cannot allocate. Reserving the space (-R)
# fails, because QEMU maps the vsyscall page at 0xffffffffff600000. A guest base
# of 2^47 (-B 0x800000000000) maps the host's upper half onto the guest's
# [0, 2^47) instead. The worker's front scrubs QEMU_* from its environment, so
# the options reach QEMU through a native interpreter registered in
# binfmt_misc, and scripts/release-smoke-qemu.sh registers it in a private
# instance, leaving the Docker host's own registration alone.
#
# WHY A CPU MODEL. A binary built for a newer CPU than the floor passes every
# test on a newer CPU, and Bun's x64 runtime chooses AVX2 code paths when the
# CPU has them. So each Linux target's smoke test runs under QEMU emulating the
# floor's CPU: Nehalem for x86_64, the Cortex-A53 for aarch64, the models Bun's
# own baseline verification emulates. A model that accepted newer instructions
# would pass everything, so before the smoke test a probe that executes one
# instruction beyond the model (AVX2 on x86_64, an Armv8.1 LSE atomic on
# aarch64) must be killed by SIGILL under it, and must run under the same QEMU
# and options with -cpu max. x86_64 on an arm64 Docker already runs under QEMU,
# so its one emulated run covers both floors. aarch64 runs natively in its
# container, then again emulated. tonistiigi/binfmt's arm64 image omits the
# emulator for its own architecture, so aarch64's QEMU is Debian's.
#
# The Linux environments see only their inputs, read only, and run the smoke
# test as an unprivileged user with no network. Neither a container nor
# user-mode emulation observes the kernel floor, which is Bun's documented
# range; see docs/specs/session-execution.md, *Delivery and
# release*.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly TARGET_SCRIPT="$REPO_ROOT/scripts/release-smoke-target.sh"
readonly QEMU_SCRIPT="$REPO_ROOT/scripts/release-smoke-qemu.sh"
readonly CASES="$REPO_ROOT/crates/ravel/scripts/installed-smoke.sh"

# TARGETS, the release target list, and assert_archive, the manifest check.
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

# CentOS Linux 7.9.2009, whose glibc is 2.17, Ravel's Linux floor. It is
# end-of-life, which is the point: no maintained distribution ships 2.17. The
# manifest-list digest serves linux/amd64 and linux/arm64/v8, so every run on
# either architecture executes the same userland.
readonly FLOOR_IMAGE="docker.io/library/centos:7@sha256:be65f488b7764ad3638f236b7b515b3678369a5124c47b8d32916d6487418ea4"

# QEMU 10.2.3's user-mode emulators, built for Docker's architecture:
# tonistiigi/binfmt's qemu-v10.2.3-68, pinned by manifest-list digest. This
# build takes the argument after the executable's path as its argv[0], the
# layout binfmt_misc's P flag passes.
readonly QEMU_IMAGE="docker.io/tonistiigi/binfmt@sha256:400a4873b838d1b89194d982c45e5fb3cda4593fbfd7e08a02e76b03b21166f0"

# Debian's qemu-user 1:10.2.2+ds-1 for arm64, whose static qemu-aarch64
# emulates arm64 guests on an arm64 host. snapshot.debian.org serves it at a
# permanent address, its SHA-1; the package is refused unless its SHA-256 is
# the one pinned here. It is cached under target/qemu-user/. Upstream QEMU
# takes a guest's argv[0] through -0, not after the executable's path.
readonly QEMU_DEB="qemu-user_10.2.2+ds-1_arm64.deb"
readonly QEMU_DEB_URL="https://snapshot.debian.org/file/4b7f47627ad6e57745d33d1108b893970e19ebf2"
readonly QEMU_DEB_SHA256="f8bf89dacd04e66a1e34526bf6eb9b4eb1b4da6205d0d820381f0c87cb0db8bb"

# The helper container that holds the emulated userland: Ubuntu 24.04, for
# util-linux's unshare and mount and coreutils' chroot --userspec.
readonly HELPER_IMAGE="docker.io/library/ubuntu:24.04@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3"

die() {
  echo "release-smoke: $*" >&2
  exit 1
}

# The archive DIR holds for TARGET; exactly one.
find_archive() {
  local dir="$1" target="$2" matches=() match
  for match in "$dir"/ravel-v*-"$target".tar.xz; do
    [[ -f "$match" ]] && matches+=("$match")
  done
  ((${#matches[@]} == 1)) && {
    echo "${matches[0]}"
    return
  }
  ((${#matches[@]} == 0)) &&
    die "no archive for $target in $dir; build one with 'task release:archives TARGETS=$target'"
  die "more than one archive for $target in $dir: ${matches[*]}"
}

docker_platform() {
  case "$1" in
    aarch64-unknown-linux-gnu) echo linux/arm64 ;;
    x86_64-unknown-linux-gnu) echo linux/amd64 ;;
    *) die "no container platform for $1" ;;
  esac
}

# The glibc control's probe pair for ARCH (uname -m's name for it), in DIR.
build_glibc_probes() {
  local arch="$1" dir="$2"
  cat >"$dir/probe.c" <<'EOF'
#include <stdio.h>
#ifdef ABOVE_FLOOR
#include <sys/random.h>
#endif

int main(void) {
#ifdef ABOVE_FLOOR
  unsigned char byte;
  if (getrandom(&byte, 1, 0) != 1) return 2;
#endif
  puts("glibc probe ran");
  return 0;
}
EOF
  if ! zig cc -target "$arch-linux-gnu.2.17" -O2 -o "$dir/floor-$arch" "$dir/probe.c" ||
    ! zig cc -target "$arch-linux-gnu.2.25" -DABOVE_FLOOR -O2 -o "$dir/above-$arch" "$dir/probe.c"; then
    die "zig cannot build the glibc probes for $arch"
  fi
  rm "$dir/probe.c"
}

# ARCH's CPU floor, as QEMU names the model (WHY A CPU MODEL).
cpu_floor() {
  case "$1" in
    x86_64) echo Nehalem ;;
    aarch64) echo cortex-a53 ;;
    *) die "no CPU floor is known for $1" ;;
  esac
}

# The CPU control's probe for ARCH, in DIR. It prints, executes one instruction
# beyond ARCH's CPU floor, then prints again, so a SIGILL between the two lines
# is that instruction's.
build_cpu_probe() {
  local arch="$1" dir="$2"
  cat >"$dir/cpu.c" <<'EOF'
#include <stdio.h>

int main(void) {
  puts("cpu probe started");
  fflush(stdout);
#if defined(__x86_64__)
  /* AVX2, a 256-bit integer add: Haswell and later, beyond Nehalem. */
  __asm__ volatile("vpaddd %%ymm0, %%ymm0, %%ymm0" ::: "xmm0");
#elif defined(__aarch64__)
  /* LSE, an Armv8.1 atomic add: beyond the Armv8.0 Cortex-A53. */
  unsigned int counter = 0, old;
  __asm__ volatile(".arch_extension lse\n\tldadd %w2, %w0, [%1]"
                   : "=&r"(old)
                   : "r"(&counter), "r"(1u)
                   : "memory");
  if (counter != 1 || old != 0) return 3;
#else
#error "no instruction beyond the CPU floor is known for this architecture"
#endif
  puts("cpu probe executed its instruction");
  return 0;
}
EOF
  zig cc -target "$arch-linux-gnu.2.17" -O2 -o "$dir/cpu-$arch" "$dir/cpu.c" ||
    die "zig cannot build the CPU probe for $arch"
  rm "$dir/cpu.c"
}

smoke_native() {
  local target="$1" archive="$2"
  env -i PATH=/usr/bin:/bin /bin/bash "$TARGET_SCRIPT" "$target" "$archive" "$CASES"
}

# The binfmt_misc interpreter pair for ARCH's guests, in DIR: static
# executables for Docker's arm64 that run /qemu/qemu, where
# release-smoke-qemu.sh puts the emulator inside the userland, with ARCH's
# options before the arguments binfmt_misc passes them. With its P flag those
# are the executable's path, then its argv[0], then the rest, and each
# interpreter hands argv[0] on as its emulator takes it. `interpreter` passes
# -cpu CPU, the floor; `interpreter-max`, identical but for -cpu max, is the
# CPU control's twin (WHY A CPU MODEL).
build_qemu_interpreters() {
  local arch="$1" cpu="$2" dir="$3" options argv0_option
  case "$arch" in
    # QEMU 10.2.3 sets no guest address limit for a 64-bit guest without -R:
    # https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/linux-user/main.c
    # tonistiigi's build takes the argument after the path as argv[0].
    x86_64) options='"-B","0x800000000000",' argv0_option=0 ;;
    # Upstream QEMU takes argv[0] from after the path only when its own auxv
    # says binfmt_misc started it, and here the interpreter starts it:
    # https://gitlab.com/qemu-project/qemu/-/blob/v10.2.2/linux-user/main.c
    aarch64) options='' argv0_option=1 ;;
    *) die "no QEMU options are known for $arch guests" ;;
  esac
  cat >"$dir/interpreter.c" <<'EOF'
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
  static char *const fixed[] = {"/qemu/qemu", QEMU_OPTIONS};
  const int count = sizeof fixed / sizeof *fixed;
  char *args[count + argc + 1];
  int n = 0;
  for (int i = 0; i < count; i++) args[n++] = fixed[i];
#if ARGV0_OPTION
  if (argc < 3) {
    fputs("release-smoke interpreter: expected an executable's path and argv[0]\n", stderr);
    return 127;
  }
  args[n++] = "-0";
  args[n++] = argv[2];
  args[n++] = argv[1];
  for (int i = 3; i < argc; i++) args[n++] = argv[i];
#else
  for (int i = 1; i < argc; i++) args[n++] = argv[i];
#endif
  args[n] = NULL;
  execv(args[0], args);
  perror("release-smoke interpreter: /qemu/qemu");
  return 127;
}
EOF
  local name model
  for name in interpreter interpreter-max; do
    model="$cpu"
    [[ "$name" == interpreter-max ]] && model=max
    zig cc -target aarch64-linux-musl -static -O2 "-DARGV0_OPTION=$argv0_option" \
      "-DQEMU_OPTIONS=$options\"-cpu\",\"$model\"" \
      -o "$dir/$name" "$dir/interpreter.c" || die "zig cannot build the QEMU $name"
  done
  rm "$dir/interpreter.c"
}

sha256_of() {
  shasum -a 256 "$1" | cut -d' ' -f1
}

# Debian's qemu-aarch64 at DEST, from the pinned package. The cached package
# is checked as a private copy, so the bytes verified are the bytes extracted;
# a missing or differing cache is fetched again, and a download that differs
# is refused.
debian_qemu_aarch64() {
  local dest="$1" cache="$REPO_ROOT/target/qemu-user" copy="$SCRATCH/$QEMU_DEB" download found
  if [[ ! -f "$cache/$QEMU_DEB" ]] || ! cp "$cache/$QEMU_DEB" "$copy" ||
    [[ "$(sha256_of "$copy")" != "$QEMU_DEB_SHA256" ]]; then
    mkdir -p "$cache" || return 1
    download="$(mktemp "$cache/.download.XXXXXX")" || return 1
    if ! curl --fail --silent --show-error --location --output "$download" "$QEMU_DEB_URL"; then
      rm -f "$download"
      die "cannot download $QEMU_DEB from $QEMU_DEB_URL"
    fi
    cp "$download" "$copy" || return 1
    found="$(sha256_of "$copy")"
    if [[ "$found" != "$QEMU_DEB_SHA256" ]]; then
      rm -f "$download"
      die "$QEMU_DEB from $QEMU_DEB_URL has SHA-256 $found, but $QEMU_DEB_SHA256 is pinned; refusing it"
    fi
    mv "$download" "$cache/$QEMU_DEB" || return 1
  fi
  ar p "$copy" data.tar.xz | tar -xJOf - ./usr/bin/qemu-aarch64 >"$dest" || return 1
  chmod +x "$dest"
}

# The pinned user-mode emulator for ARCH's guests on an arm64 Docker, at DEST:
# tonistiigi/binfmt's QEMU 10.2.3 for x86_64 (WHY OUR OWN QEMU) and Debian's
# QEMU 10.2.2 for aarch64 (WHY A CPU MODEL). build_qemu_interpreters knows
# which argv[0] layout each takes.
pinned_qemu() {
  local arch="$1" dest="$2"
  case "$arch" in
    x86_64) copy_from_image "$DOCKER_PLATFORM" "$QEMU_IMAGE" /usr/bin/qemu-x86_64 "$dest" ;;
    aarch64) debian_qemu_aarch64 "$dest" ;;
    *) die "no pinned QEMU emulates $arch guests" ;;
  esac
}

# Copy PATH out of IMAGE for PLATFORM to DEST without running the image; a PATH
# of / exports its whole filesystem as a tar file.
copy_from_image() {
  local platform="$1" image="$2" path="$3" dest="$4" container status=0
  container="$(docker create --platform "$platform" "$image" /bin/true)" || return 1
  if [[ "$path" == / ]]; then
    docker export --output "$dest" "$container" || status=$?
  else
    docker cp --quiet "$container:$path" "$dest" || status=$?
  fi
  docker rm "$container" >/dev/null || status=$?
  return "$status"
}

# main calls the routes below on the left of `||`, where errexit does not
# reach, so every step before the run returns its own failure.

# The inputs every Linux route reads, in MOUNT: the archive, the two
# in-environment scripts, and the glibc and CPU probes.
stage_linux_inputs() {
  local target="$1" archive="$2" mount="$3" arch="${1%%-*}"
  mkdir -p "$mount/probes" || return 1
  build_glibc_probes "$arch" "$mount/probes"
  build_cpu_probe "$arch" "$mount/probes"
  cp "$archive" "$TARGET_SCRIPT" "$CASES" "$mount/"
}

smoke_in_floor_container() {
  local target="$1" archive="$2" platform="$3" mount="$4"
  echo "release-smoke: $target: $platform container, native to Docker's $DOCKER_PLATFORM"
  docker run --rm --network none --user 1000:1000 \
    --platform "$platform" \
    --volume "$mount:/smoke:ro" \
    "$FLOOR_IMAGE" \
    /usr/bin/env -i PATH=/usr/bin:/bin /bin/bash /smoke/release-smoke-target.sh \
    "$target" "/smoke/$(basename "$archive")" /smoke/installed-smoke.sh /smoke/probes
}

# Every ELF file in ARCHIVE, as the hex of its first 20 bytes, a tab and its
# path, one per line, in OUT. release-smoke-qemu.sh holds each against its
# registration, so that no executable escapes the CPU model; its helper
# container has no xz to read the archive itself.
archive_elf_headers() {
  local archive="$1" out="$2" dir file header
  dir="$(mktemp -d "$SCRATCH/elf.XXXXXX")" || return 1
  tar -xJf "$archive" -C "$dir" || return 1
  while IFS= read -r file; do
    header="$(od -An -tx1 -N20 "$file" | tr -d ' \n')"
    if [[ "$header" == 7f454c46* ]]; then
      printf '%s\t%s\n' "$header" "${file#"$dir"/}"
    fi
  done < <(find "$dir" -type f) >"$out"
  rm -rf "$dir"
}

# The floor image's userland for PLATFORM under the pinned QEMU emulating the
# target's CPU floor, in a helper container of Docker's own architecture (WHY
# OUR OWN QEMU, WHY A CPU MODEL).
#
# WHY THE HELPER HAS NO SECCOMP FILTER. release-smoke-qemu.sh registers the
# emulator in a binfmt_misc instance of its own, which takes a user namespace.
# Docker 29.8.1's built-in seccomp profile refuses unshare(2) to a container
# without CAP_SYS_ADMIN, so the helper's root got EPERM and no emulated route
# ran. Under Docker Desktop 28.1.1 the same script passed with the profile in
# place. Lifting the filter adds no capability: the helper keeps Docker's
# default set, no network and a read-only mount, and runs only this script's
# digest-pinned inputs. --cap-add SYS_ADMIN works too and keeps the filter,
# but it hands the container the capability most container escapes begin
# with. The floor container, which needs no namespace, keeps the profile.
smoke_under_qemu() {
  local target="$1" archive="$2" platform="$3" mount="$4" arch="${1%%-*}" cpu
  cpu="$(cpu_floor "$arch")" || return 1
  archive_elf_headers "$archive" "$mount/elf-headers" || return 1
  copy_from_image "$platform" "$FLOOR_IMAGE" / "$mount/rootfs.tar" || return 1
  pinned_qemu "$arch" "$mount/qemu" || return 1
  build_qemu_interpreters "$arch" "$cpu" "$mount"
  cp "$QEMU_SCRIPT" "$mount/" || return 1
  echo "release-smoke: $target: $platform userland under the pinned QEMU with -cpu $cpu, in a $DOCKER_PLATFORM $HELPER_IMAGE container"
  docker run --rm --network none \
    --security-opt seccomp=unconfined \
    --platform "$DOCKER_PLATFORM" \
    --volume "$mount:/smoke:ro" \
    "$HELPER_IMAGE" \
    /bin/bash /smoke/release-smoke-qemu.sh /smoke "$target" "$(basename "$archive")" "$cpu"
}

# A target native to Docker runs in its container, then emulated at its CPU
# floor; a foreign one runs emulated only, which covers both floors.
smoke_linux() {
  local target="$1" archive="$2" mount="$SCRATCH/$1" platform
  platform="$(docker_platform "$target")" || return 1
  stage_linux_inputs "$target" "$archive" "$mount" || return 1
  if [[ "$platform" == "$DOCKER_PLATFORM" ]]; then
    smoke_in_floor_container "$target" "$archive" "$platform" "$mount" || return 1
  fi
  smoke_under_qemu "$target" "$archive" "$platform" "$mount"
}

main() {
  local archives="$REPO_ROOT/target/archives" targets=() target known
  if [[ "${1:-}" == "--archives" ]]; then
    archives="${2:?--archives needs a directory}"
    shift 2
  fi
  targets=("$@")
  ((${#targets[@]} > 0)) || targets=("${TARGETS[@]}")
  for target in "${targets[@]}"; do
    for known in "${TARGETS[@]}"; do
      [[ "$target" == "$known" ]] && continue 2
    done
    die "unknown target '$target'; the release targets are $(IFS=' ' && echo "${TARGETS[*]}")"
  done
  for target in "${targets[@]}"; do
    [[ "$target" == *-linux-gnu ]] || continue
    command -v zig >/dev/null 2>&1 || die "zig is required for the glibc control's probes; see the chapter 12 section \"Build archives before a release\" (docs/book/src/12-deliver-ravel.md, #build-archives-before-a-release)"
    docker info >/dev/null 2>&1 || die "Docker is required for the Linux targets, and is not running"
    DOCKER_PLATFORM="$(docker version --format '{{.Server.Os}}/{{.Server.Arch}}')"
    [[ "$DOCKER_PLATFORM" == linux/arm64 ]] ||
      die "the Linux targets need an arm64 Docker, the only one whose QEMU and interpreters are pinned; this Docker is $DOCKER_PLATFORM"
    echo "release-smoke: floor image $FLOOR_IMAGE, docker $(docker version --format '{{.Server.Version}}') on $DOCKER_PLATFORM, zig $(zig version)"
    break
  done

  SCRATCH="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$SCRATCH'" EXIT
  local archive
  for target in "${targets[@]}"; do
    archive="$(find_archive "$archives" "$target")"
    echo "release-smoke: $target: $archive"
    assert_archive "$archive" "$(basename "$archive" .tar.xz)" ||
      die "$target: $archive does not match the archive manifest; see above"
    case "$target" in
      *-apple-darwin) smoke_native "$target" "$archive" ;;
      *-unknown-linux-gnu) smoke_linux "$target" "$archive" ;;
    esac || die "$target: the installed smoke test failed; see above"
  done
  echo "release-smoke: passed: $(IFS=' ' && echo "${targets[*]}")"
}

main "$@"
