/* What user-mode emulation does and does not preserve, as one control.
 *
 * The same source is built for each target and run where it is measured: native
 * on aarch64 Linux, and as an x86-64 executable under QEMU's user-mode
 * emulator. Each line is one observation of an operation the native-entry
 * matrix depends on, printed as `name=value` so two runs can be compared
 * line by line. The program decides nothing; a difference between the native
 * and emulated runs is the emulator's, and is why an emulated pass cannot
 * qualify the kernel's signal, process-group or terminal semantics.
 *
 * Operations:
 *   host_threads            threads this process has, as the host kernel counts
 *   unshare_newuser         unshare(CLONE_NEWUSER)'s errno (0 on success)
 *   nocldwait_wait          wait result for a child exited under
 *                           SIGCHLD = SIG_DFL + SA_NOCLDWAIT (native: ECHILD)
 *   sigign_wait             the same under SIGCHLD = SIG_IGN (native: ECHILD)
 *   default_wait            the same under SIGCHLD = SIG_DFL (control: child status)
 *   zombie_group_probe      kill(-pgid, 0) for a group of one unreaped zombie
 */
#define _GNU_SOURCE
#include <errno.h>
#include <dirent.h>
#include <sched.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

static void nap(void) {
  struct timespec ts = {0, 100 * 1000 * 1000};
  nanosleep(&ts, NULL);
}

static int thread_count(void) {
  int n = 0;
  DIR *d = opendir("/proc/self/task");
  struct dirent *e;
  if (!d) return -1;
  while ((e = readdir(d)))
    if (e->d_name[0] != '.') n++;
  closedir(d);
  return n;
}

/* Fork a child that exits at once, wait for it under SIGCHLD set as given, and
 * return what the wait said: its exit status, or -errno. */
static int wait_under(int ignore, int flags) {
  struct sigaction act, old;
  memset(&act, 0, sizeof act);
  act.sa_handler = ignore ? SIG_IGN : SIG_DFL;
  act.sa_flags = flags;
  sigemptyset(&act.sa_mask);
  sigaction(SIGCHLD, &act, &old);
  pid_t child = fork();
  if (child == 0) _exit(7);
  nap();
  int status = 0;
  pid_t r = waitpid(child, &status, 0);
  int result = r == child ? (WIFEXITED(status) ? WEXITSTATUS(status) : 255) : -errno;
  sigaction(SIGCHLD, &old, NULL);
  return result;
}

static const char *name_of(int v) {
  static char buf[32];
  if (v == -ECHILD) return "ECHILD";
  if (v == -EINTR) return "EINTR";
  snprintf(buf, sizeof buf, "%d", v);
  return buf;
}

int main(void) {
  printf("host_threads=%d\n", thread_count());

  /* unshare(CLONE_NEWUSER) needs a single-threaded caller. */
  {
    pid_t child = fork();
    if (child == 0) {
      int r = unshare(CLONE_NEWUSER);
      _exit(r == 0 ? 0 : (errno & 0x7f));
    }
    int status = 0;
    waitpid(child, &status, 0);
    printf("unshare_newuser=%d\n", WIFEXITED(status) ? WEXITSTATUS(status) : -1);
  }

  printf("nocldwait_wait=%s\n", name_of(wait_under(0, SA_NOCLDWAIT)));
  printf("sigign_wait=%s\n", name_of(wait_under(1, 0)));
  printf("default_wait=%s\n", name_of(wait_under(0, 0)));

  /* A process group whose only member is an unreaped zombie. */
  {
    pid_t child = fork();
    if (child == 0) {
      setpgid(0, 0);
      _exit(0);
    }
    nap();
    int r = kill(-child, 0);
    printf("zombie_group_probe=%s\n", r == 0 ? "0" : strerror(errno));
    waitpid(child, NULL, 0);
    r = kill(-child, 0);
    printf("zombie_group_probe_after_reap=%s\n", r == 0 ? "0" : strerror(errno));
  }
  return 0;
}
