/* The handled-signal domain, as the target's headers and the running kernel
 * state it, for the native-entry design's 1..=64 minus glibc-reserved 32/33.
 *
 * Prints one JSON object. Nothing here is the product: it reports
 *   - the header constants (_NSIG, SIGRTMIN, SIGRTMAX as glibc defines them),
 *   - for every number 1..=65, what glibc's sigaction(n, NULL, &old) answered,
 *   - for every number 1..=65, what the kernel's rt_sigaction answered through
 *     the raw system call (glibc refuses 32 and 33 itself; the kernel does not),
 *   - and whether sigismember accepts each number in a set.
 * Built with the target's own cc and run on the target's kernel.
 */
#define _GNU_SOURCE
#include <errno.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/syscall.h>
#include <unistd.h>

static void emit(const char *name, int (*probe)(int), int first) {
  printf("%s\"%s\":[", first ? "" : ",", name);
  for (int n = 1; n <= 65; n++) {
    printf("%s%d", n == 1 ? "" : ",", probe(n));
  }
  printf("]");
}

/* 0 on success, otherwise the errno. */
static int libc_query(int n) {
  struct sigaction old;
  memset(&old, 0, sizeof old);
  return sigaction(n, NULL, &old) == 0 ? 0 : errno;
}

static int raw_query(int n) {
  /* kernel struct sigaction: handler, flags, [restorer on some archs], mask.
   * A zeroed buffer larger than any layout, with the kernel's sigsetsize (8). */
  unsigned long buffer[8];
  memset(buffer, 0, sizeof buffer);
  long r = syscall(SYS_rt_sigaction, n, NULL, buffer, 8);
  return r == 0 ? 0 : errno;
}

static int set_member(int n) {
  sigset_t set;
  sigemptyset(&set);
  int r = sigismember(&set, n);
  return r == -1 ? errno : 0;
}

int main(void) {
  printf("{\"nsig\":%d,\"sigrtmin\":%d,\"sigrtmax\":%d,\"results\":{", _NSIG, SIGRTMIN, SIGRTMAX);
  emit("glibc_sigaction", libc_query, 1);
  emit("kernel_rt_sigaction", raw_query, 0);
  emit("glibc_sigismember", set_member, 0);
  printf("}}\n");
  return 0;
}
