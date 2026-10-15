/* Make a directory the root of this mount namespace and drop the old root.
 *
 * This image's util-linux ships no pivot_root and glibc has no wrapper for the
 * system call, and the guest's own umount cannot be relied on after the pivot
 * (it is an emulated executable reading the namespace it is detaching from), so
 * the whole step is here, with no dynamic loading: static, and only system
 * calls. Used once, by scripts/native-linux-qemu.sh, to make the guest's
 * filesystem the root of its private mount namespace.
 *
 *   pivot-root NEW_ROOT OLD_NAME
 *
 * NEW_ROOT must be a mount point and hold an empty directory OLD_NAME.
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <sys/mount.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>

int main(int argc, char **argv) {
  char old[4096];
  if (argc != 3) {
    fputs("usage: pivot-root NEW_ROOT OLD_NAME\n", stderr);
    return 2;
  }
  if (chdir(argv[1]) != 0) {
    perror("chdir NEW_ROOT");
    return 1;
  }
  if (syscall(SYS_pivot_root, ".", argv[2]) != 0) {
    perror("pivot_root");
    return 1;
  }
  if (chdir("/") != 0) {
    perror("chdir /");
    return 1;
  }
  snprintf(old, sizeof old, "/%s", argv[2]);
  if (umount2(old, MNT_DETACH) != 0) {
    perror("umount2 old root");
    return 1;
  }
  if (rmdir(old) != 0) {
    perror("rmdir old root");
    return 1;
  }
  return 0;
}
