#!/usr/bin/env bash
#
# The emulated x86-64 environment for the native-entry support campaign (k49),
# run as root in a helper container of Docker's own architecture. It is not an
# entry point: scripts/native-linux.sh stages the inputs and starts it.
#
#   native-linux-qemu.sh serve INPUTS CPU
#
# INPUTS, read-only, holds rootfs.tar (the amd64 image's filesystem), qemu (the
# pinned emulator) and interpreter (the binfmt_misc interpreter, which runs qemu
# with the guest base and CPU model). The userland is unpacked into /floor and
# becomes the namespace's root (see in_namespace for why not a chroot, which is
# how scripts/release-smoke-qemu.sh enters its glibc-2.17 floor). As there, the
# registration goes into a
# private binfmt_misc instance, which the kernel (6.7 and later) gives each user
# namespace that mounts one, so the Docker host's own registration (which every
# other container uses, and which cannot run rustc: docs/runtime-evidence.md
# records the guest-address-space finding) is left alone.
#
# Unlike the release smoke, this environment is kept running: after the guest's
# address space and vDSO are checked it writes /run/native-qemu.ready and holds
# the namespace with `sleep`, so native-linux.sh can enter it repeatedly with
# nsenter (build, campaign, R01) and a failed run can be inspected.
set -euo pipefail
IFS=$'\n\t'

readonly FLOOR=/floor
readonly PIDFILE=/run/native-qemu.pid
readonly READY=/run/native-qemu.ready
# The x86-64 registration, as QEMU's own scripts/qemu-binfmt-conf.sh writes it
# and scripts/release-smoke-qemu.sh's guest() copies it, then the lowest address
# an x86-64 user space never reaches.
# https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/scripts/qemu-binfmt-conf.sh
readonly MAGIC='\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3e\x00'
readonly MASK='\xff\xff\xff\xff\xff\xfe\xfe\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff'
readonly LIMIT=800000000000

die() {
  echo "native-linux-qemu: FAIL: $*" >&2
  exit 1
}

note() {
  echo "native-linux-qemu: $*"
}

# As the container's root: unpack the userland, then enter a new user and mount
# namespace. A process inside that namespace may map only its own uid, and
# `unshare --map-users` needs newuidmap, which this image lacks, so the
# container's root, outside the namespace, writes the identity maps; the child
# waits on a FIFO until then and re-executes this script so that it holds its
# capabilities in the namespace.
outside() {
  local inputs="$1" cpu="$2"
  mkdir -p "$FLOOR/qemu"
  tar -xf "$inputs/rootfs.tar" -C "$FLOOR" || die "cannot unpack the userland"
  cp "$inputs/qemu" "$inputs/interpreter" "$FLOOR/qemu/"
  chmod 0755 "$FLOOR/qemu/qemu" "$FLOOR/qemu/interpreter"
  ln -sfn "$FLOOR/qemu" /qemu
  note "userland unpacked; $("$FLOOR/qemu/qemu" --version | head -n 1), -cpu $cpu"

  local sync own pid
  sync="$(mktemp -d)"
  mkfifo "$sync/mapped"
  own="$(readlink /proc/self/ns/user)"
  # shellcheck disable=SC2016 # the child shell expands its own arguments
  unshare --user --mount /bin/bash -c 'read -r _ <"$1" && shift && exec /bin/bash "$@"' \
    _ "$sync/mapped" "$0" --in-namespace "$inputs" "$cpu" &
  pid=$!
  until [[ "$(readlink "/proc/$pid/ns/user" 2>/dev/null)" != "$own" ]]; do :; done
  kill -0 "$pid" 2>/dev/null || die "unshare could not create a user namespace"
  echo '0 0 65536' >"/proc/$pid/uid_map"
  echo '0 0 65536' >"/proc/$pid/gid_map"
  printf '%s\n' "$pid" >"$PIDFILE"
  echo >"$sync/mapped"
  wait "$pid"
}

# As root in the namespace: bind what the guest needs, register the
# interpreter, check the guest's address space, make the guest's filesystem this
# namespace's root, then hold the namespace.
#
# Why a pivot_root and not a chroot (the release smoke's choice): unshare(2)
# refuses CLONE_NEWUSER to a chrooted process, and the product's confinement
# runs bubblewrap, so the guest is made the namespace's actual root and
# nsenter lands in it directly. That is not what stops confinement here. What
# does is measured and recorded in the campaign's evidence: QEMU's user-mode
# emulator runs a second thread (its RCU thread) in every emulated process, so
# unshare(CLONE_NEWUSER), which needs a single-threaded caller, fails EINVAL for
# any emulated process. No emulated process can create a user namespace, so the
# product's bubblewrap confinement cannot run in this environment at all.
in_namespace() {
  local inputs="$1" cpu="$2" binfmt="$FLOOR/run/binfmt" maps line end vdso=0
  mkdir -p "$binfmt" "$FLOOR/srv/git" "$FLOOR/oldroot"
  mount -t binfmt_misc binfmt_misc "$binfmt" || die "cannot mount a private binfmt_misc instance"
  mount --bind /srv/git "$FLOOR/srv/git"
  mount --rbind /proc "$FLOOR/proc"
  mount --rbind /dev "$FLOOR/dev"
  # The guest resolves names through the helper's resolver and sees its network.
  [[ -e "$FLOOR/etc/resolv.conf" ]] || : >"$FLOOR/etc/resolv.conf"
  mount --bind /etc/resolv.conf "$FLOOR/etc/resolv.conf"
  # Last: only guest executables match, so the chroot that starts each guest
  # command is the only thing that runs emulated. F opens the interpreter now,
  # outside the guest's root; P passes the executable's own argv[0].
  printf '%s' ":native-linux-x86_64:M::$MAGIC:$MASK:$FLOOR/qemu/interpreter:PF" >"$binfmt/register" ||
    die "cannot register the interpreter"

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
    ((${#end} < 16 && 16#$end <= 16#$LIMIT)) || die "the guest has a mapping above 0x$LIMIT: $line"
  done <<<"$maps"
  ((vdso)) || die "the guest has no vDSO: $maps"
  note "guest address space below 0x$LIMIT, with a vDSO, under -cpu $cpu"

  # The helper's /run is gone with the old root, so readiness is written through
  # a descriptor opened before the pivot, after the pivot has happened.
  exec 9>"$READY"
  # Recursive: the binfmt registration, /proc, /dev and the other mounts above
  # are submounts of the guest's tree and must come along into the new root.
  mount --make-rprivate /
  mount --rbind "$FLOOR" "$FLOOR"
  "$inputs/pivot-root" "$FLOOR" oldroot
  echo ready >&9
  exec 9>&-
  exec /usr/bin/sleep infinity
}

main() {
  case "${1:-}" in
    serve) shift; (($# == 2)) || die "usage: serve INPUTS CPU"; outside "$@" ;;
    --in-namespace) shift; in_namespace "$@" ;;
    *) die "usage: native-linux-qemu.sh serve INPUTS CPU" ;;
  esac
}

main "$@"
