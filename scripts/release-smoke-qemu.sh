#!/usr/bin/env bash
#
# One Linux target's installed smoke test in the glibc-2.17 userland under a
# pinned user-mode QEMU emulating the target's CPU floor, CPU.
# scripts/release-smoke.sh runs this as root in a helper container of Docker's
# own architecture; it is not an entry point of its own.
#
#   release-smoke-qemu.sh INPUTS TARGET ARCHIVE CPU
#
# INPUTS, read-only, holds everything the run reads: rootfs.tar, the floor
# image's filesystem for TARGET's architecture; qemu, the pinned emulator;
# interpreter, the binfmt_misc interpreter release-smoke.sh builds, which runs
# qemu with the options this target needs and -cpu CPU, and interpreter-max,
# its twin with -cpu max; release-smoke-target.sh, installed-smoke.sh, the
# glibc and CPU probes in probes/, ARCHIVE, and elf-headers, the first 20
# bytes of each ELF file in ARCHIVE.
#
# The userland is the same image the container route runs, unpacked into
# /floor and entered with chroot as uid 1000. Its executables reach qemu
# through binfmt_misc, even where the helper's architecture could run them
# itself, and the registration must not touch the Docker host's own, which
# every other container uses. So it
# goes into a private binfmt_misc instance, which the kernel (6.7 and later)
# gives each user namespace that mounts one. The namespace maps uids and gids
# 0-65535 to themselves, so root inside it is the container's root, and uid
# 1000 in the chroot owns nothing there, as in the container route.
#
# Before the smoke test, the guest's own /proc/self/maps must show a vDSO and
# no mapping at or above the target's user address limit but the vsyscall
# page. Docker Desktop's QEMU 8.1.5 gives x86-64 guests no vDSO, and QEMU
# 10.2.3 without a guest base maps them above 2^47 (docs/runtime-evidence.md,
# *Source findings retained with the implementation*); either
# would otherwise run the smoke test in an environment no x86-64 kernel makes.
# Then the CPU model must be seen to refuse an instruction beyond it, which
# the twin interpreter's -cpu max executes (release-smoke.sh, WHY A CPU MODEL).

set -euo pipefail
IFS=$'\n\t'

readonly FLOOR=/floor

die() {
  echo "release-smoke: ${TARGET:-qemu}: FAIL: $*" >&2
  exit 1
}

note() {
  echo "release-smoke: $TARGET: $*"
}

# binfmt_misc's magic and mask for ARCH's ELF executables, as QEMU's own
# scripts/qemu-binfmt-conf.sh registers them, then the lowest address ARCH's
# user space never reaches: 2^47 on x86-64, 2^48 on arm64 with 48-bit
# addresses, as Docker's kernel has.
# https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/scripts/qemu-binfmt-conf.sh
guest() {
  case "$1" in
    x86_64)
      printf '%s\n' \
        '\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3e\x00' \
        '\xff\xff\xff\xff\xff\xfe\xfe\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff' \
        800000000000
      ;;
    aarch64)
      printf '%s\n' \
        '\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\xb7\x00' \
        '\xff\xff\xff\xff\xff\xff\xff\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff' \
        1000000000000
      ;;
    *) die "no user-mode guest is known for $1" ;;
  esac
}

# Keep FILE, an executable of the helper's own architecture, from matching
# guest()'s registration. Both masks require ELF ident bytes 8 to 15 to be
# zero, and Linux's ELF loader never reads them, so FILE gets EI_ABIVERSION
# (byte 8) 1. On an arm64 helper an aarch64 registration would otherwise also
# match the interpreters and QEMU themselves: the interpreter would be its own
# interpreter until exec failed with ELOOP. The helper's own executables still
# match, so after registration they run emulated too.
exempt() {
  printf '\001' | dd of="$1" bs=1 seek=8 count=1 conv=notrunc status=none
  [[ "$(od -An -tu1 -j8 -N1 "$1" | tr -d ' ')" == 1 ]] || die "cannot exempt $1 from the registration"
}

# Run as the container's root: unpack the userland, then enter a new user and
# mount namespace. A process inside that namespace may map only its own uid,
# and `unshare --map-users` needs newuidmap, which this image lacks, so the
# container's root, outside the namespace, writes the identity maps. The child
# waits on a FIFO until then, and re-executes this script so that it holds its
# capabilities in the namespace: execve grants them to uid 0, and the child
# was unmapped when it last executed.
outside() {
  local inputs="$1" archive="$3" cpu="$4" arch file
  arch="${TARGET%%-*}"
  guest "$arch" >/dev/null
  mkdir "$FLOOR" "$FLOOR/qemu"
  tar -xf "$inputs/rootfs.tar" -C "$FLOOR" || die "cannot unpack the floor image's filesystem"
  for file in qemu interpreter interpreter-max; do
    cp "$inputs/$file" "$FLOOR/qemu/"
    exempt "$FLOOR/qemu/$file"
  done
  # The interpreter runs /qemu/qemu in its caller's root, which is the
  # helper's own for a helper executable that matches the registration.
  ln -s "$FLOOR/qemu" /qemu
  note "userland unpacked; $("$FLOOR/qemu/qemu" --version | head -n 1), -cpu $cpu"

  local sync own pid status=0
  sync="$(mktemp -d)"
  mkfifo "$sync/mapped"
  own="$(readlink /proc/self/ns/user)"
  # shellcheck disable=SC2016 # the child shell expands its own arguments
  unshare --user --mount /bin/bash -c 'read -r _ <"$1" && shift && exec /bin/bash "$@"' \
    _ "$sync/mapped" "$0" --in-namespace "$inputs" "$TARGET" "$archive" "$cpu" &
  pid=$!
  until [[ "$(readlink "/proc/$pid/ns/user" 2>/dev/null)" != "$own" ]]; do :; done
  kill -0 "$pid" 2>/dev/null || die "unshare could not create a user namespace"
  echo '0 0 65536' >"/proc/$pid/uid_map"
  echo '0 0 65536' >"/proc/$pid/gid_map"
  echo >"$sync/mapped"
  wait "$pid" || status=$?
  return "$status"
}

# Every ELF file in the archive must match MAGIC under MASK, or it would run
# on the helper's own CPU, out of the model's reach, and pass untested. HEADERS
# lists each file's first 20 bytes; it must include the front and the worker.
assert_archive_matches() {
  local headers="$1" magic mask header path i count=0 front=0 worker=0
  # A registration's escapes as hex; a pipe carries their NUL bytes.
  magic="$(printf '%b' "$2" | od -An -tx1 | tr -d ' \n')"
  mask="$(printf '%b' "$3" | od -An -tx1 | tr -d ' \n')"
  while IFS=$'\t' read -r header path; do
    ((${#header} == 40)) || die "$path is too short to be an ELF executable: $header"
    for ((i = 0; i < 40; i += 2)); do
      if (((16#${header:i:2} ^ 16#${magic:i:2}) & 16#${mask:i:2})); then
        die "$path does not match the registration, so it would escape the CPU model: $header"
      fi
    done
    count=$((count + 1))
    case "$path" in
      */bin/ravel) front=1 ;;
      */libexec/ravel/ravel-policy) worker=1 ;;
    esac
  done <"$headers"
  ((front && worker)) || die "$headers lists no ravel front or worker: $(cat "$headers")"
  note "all $count ELF files in the archive match the registration, so they run under the model"
}

# Run as root in the namespace: bind the inputs, /proc and /dev into the
# userland, register the interpreter, check the guest's address space and CPU
# model, and run the smoke test there as uid 1000.
in_namespace() {
  local inputs="$1" archive="$3" cpu="$4" arch magic mask limit binfmt
  arch="${TARGET%%-*}"
  { read -r magic && read -r mask && read -r limit; } < <(guest "$arch")
  assert_archive_matches "$inputs/elf-headers" "$magic" "$mask"
  binfmt="$(mktemp -d)"
  mount -t binfmt_misc binfmt_misc "$binfmt" || die "cannot mount a private binfmt_misc instance"
  mkdir -p "$FLOOR/smoke"
  # A bind keeps the read-only flag of release-smoke.sh's volume, which the
  # namespace cannot clear.
  mount --bind "$inputs" "$FLOOR/smoke"
  mount --rbind /proc "$FLOOR/proc"
  mount --rbind /dev "$FLOOR/dev"
  # Last: on an arm64 helper its own executables match too (exempt), so of
  # them only the chroot that starts each guest command runs emulated. F opens
  # the interpreter now, outside the chroot; the interpreter finds qemu inside
  # it. P passes the executable's own argv[0], which the interpreter hands on
  # as its qemu takes it.
  printf '%s' ":release-smoke-$arch:M::$magic:$mask:$FLOOR/qemu/interpreter:PF" >"$binfmt/register" ||
    die "cannot register the interpreter"

  local maps line end vdso=0
  maps="$(chroot --userspec=1000:1000 "$FLOOR" /usr/bin/cat /proc/self/maps)" ||
    die "the userland's cat did not run under $FLOOR/qemu/interpreter"
  while IFS= read -r line; do
    case "$line" in
      *'[vsyscall]') continue ;;
      *'[vdso]') vdso=1 ;;
    esac
    end="${line%% *}"
    end="${end#*-}"
    # Fifteen hex digits at most, so bash's signed arithmetic cannot wrap.
    ((${#end} < 16 && 16#$end <= 16#$limit)) || die "the guest has a mapping above 0x$limit: $line"
  done <<<"$maps"
  ((vdso)) || die "the guest has no vDSO: $maps"
  note "guest address space below 0x$limit, with a vDSO"

  # The CPU probe prints, executes one instruction beyond the floor, and prints
  # again. Under the registered interpreter it must die of SIGILL between the
  # two lines. Under interpreter-max, identical but for -cpu max, it must run to
  # the end, which shows the instruction is valid and that this QEMU executes
  # it, so the refusal is the model's.
  local probe="/smoke/probes/cpu-$arch" refused output status=0
  refused="$(chroot --userspec=1000:1000 "$FLOOR" "$probe" 2>&1)" || status=$?
  if ((status != 132)) || [[ "$refused" != "cpu probe started"* || "$refused" == *"executed its instruction"* ]]; then
    die "positive control: under -cpu $cpu, the CPU probe was not killed by SIGILL at its instruction (exit $status): $refused"
  fi
  status=0
  output="$(chroot --userspec=1000:1000 "$FLOOR" /qemu/interpreter-max "$probe" "cpu-$arch" 2>&1)" || status=$?
  if ((status != 0)) || [[ "$output" != *"cpu probe executed its instruction" ]]; then
    die "the CPU probe did not run under the same QEMU with -cpu max, so its refusal is not evidence (exit $status): $output"
  fi
  note "CPU control: -cpu $cpu killed the probe at its instruction (exit 132): ${refused//$'\n'/ | }; -cpu max ran it"

  exec chroot --userspec=1000:1000 "$FLOOR" \
    /usr/bin/env -i PATH=/usr/bin:/bin /bin/bash /smoke/release-smoke-target.sh \
    "$TARGET" "/smoke/$archive" /smoke/installed-smoke.sh /smoke/probes
}

main() {
  local phase=outside
  if [[ "${1:-}" == --in-namespace ]]; then
    phase=in_namespace
    shift
  fi
  (($# == 4)) || {
    echo "usage: release-smoke-qemu.sh INPUTS TARGET ARCHIVE CPU" >&2
    exit 1
  }
  TARGET="$2"
  "$phase" "$@"
}

main "$@"
