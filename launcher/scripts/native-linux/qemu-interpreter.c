/* The binfmt_misc interpreter for the emulated x86-64 campaign: it runs the
 * pinned QEMU with the options this target needs. It is the same wrapper
 * scripts/release-smoke.sh builds (build_qemu_interpreters), compiled from a
 * file rather than a here-document, because the campaign's environment is kept
 * running and entered repeatedly rather than run once.
 *
 * Why options live here and not in the environment: the worker's front scrubs
 * QEMU_* from its environment, so options set there would not reach the
 * emulator for every process; a registered interpreter sees them all.
 *
 * QEMU_OPTIONS is a list of string-literal arguments ending in a comma, such as
 *   "-B","0x800000000000","-cpu","Nehalem",
 * ARGV0_OPTION is 1 for a QEMU that takes argv[0] from -0, and 0 for the
 * tonistiigi build, which takes the argument after the executable's path.
 */
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
    fputs("native-linux interpreter: expected an executable's path and argv[0]\n", stderr);
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
  perror("native-linux interpreter: /qemu/qemu");
  return 127;
}
