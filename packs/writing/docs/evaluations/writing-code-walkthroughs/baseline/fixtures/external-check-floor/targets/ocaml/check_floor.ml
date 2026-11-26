(* The deployment floor's gate — `declare-the-deployment-floor-k153`, ADR-0069.
 *
 * It asks the ARTIFACT, never the plan. The floor is a build configuration, and
 * a build configuration's failure mode is that the flag looks present and
 * changes nothing: `-Xlinker -platform_version` moves the load command and
 * leaves every weak-import bit exactly where it was, because the load command is
 * written by the linker and the weak-import decision by the compiler. A fix
 * verified through `otool -l` alone would have looked complete and shipped the
 * hazard, so the dylib's real claim is read with `nm -m`.
 *
 * Two subcommands because the two artifact kinds carry different claims:
 *
 *   dylib <floor-file> <mach-o>    minos = floor, autolink > 0, other = 0
 *   exe   <floor-file> <mach-o>…   minos = floor
 *
 * THE AUTOLINK COUNT IS THE CONTROL ARM. `nm -m … | grep 'undefined.*weak
 * external' | grep -v _swift_FORCE_LOAD_` answering zero is satisfied just as
 * well by a filter that matched everything, a Mach-O `nm` could not read, or an
 * output format that moved. Swift's per-framework `_swift_FORCE_LOAD_*` autolink
 * stubs are ALWAYS undefined weak externals, so a positive count on that side is
 * what makes the zero on the other side a measurement. It is asserted as
 * `> 0` rather than as its whole-family value (29 at 252 frameworks) on purpose:
 * the count tracks how many frameworks the residual imports, so pinning it would
 * make the gate red for a reason that is not the hazard — and would make it red
 * on a checkout that has not run the emitter, where the dylib is the four
 * hand-written bridges alone. The observed pair is always printed, so the number
 * is available without being load-bearing. *)

let failures = ref 0

let fail fmt =
  Printf.ksprintf
    (fun message ->
      incr failures;
      print_endline ("FAIL  " ^ message))
    fmt

let pass fmt = Printf.ksprintf (fun message -> print_endline ("ok    " ^ message)) fmt

(* Run a tool and return its stdout lines. A tool that cannot be run, or that
   exits non-zero, is a failure of the gate rather than of the artifact — the
   distinction matters because "no output" is what every one of these checks
   reads as success if it is not caught here. *)
let run_lines program args =
  let command = Filename.quote_command program args in
  let channel = Unix.open_process_in command in
  let lines = ref [] in
  (try
     while true do
       lines := input_line channel :: !lines
     done
   with End_of_file -> ());
  (match Unix.close_process_in channel with
  | Unix.WEXITED 0 -> ()
  | _ -> failwith (command ^ ": did not exit 0"));
  List.rev !lines

let contains haystack needle =
  let n = String.length needle and h = String.length haystack in
  let rec at i = i + n <= h && (String.sub haystack i n = needle || at (i + 1)) in
  n = 0 || at 0

(* The `minos` of the Mach-O's LC_BUILD_VERSION, as `otool -l` spells it.
   Compared to the floor file's text literally: `26.5` in the floor file and
   `26.5` here, so a floor written as `26` would read back as `26.0` and fail
   loudly rather than being normalised into agreement. *)
let minos path =
  let lines = run_lines "otool" [ "-l"; path ] in
  let rec scan in_build_version = function
    | [] -> None
    | line :: rest ->
        let line = String.trim line in
        if contains line "LC_BUILD_VERSION" then scan true rest
        else if in_build_version && String.length line > 6 && String.sub line 0 6 = "minos "
        then Some (String.trim (String.sub line 6 (String.length line - 6)))
        else scan in_build_version rest
  in
  scan false lines

(* (autolink, other) over the undefined weak externals. `nm -m` is the only
   instrument that reports the weak-import bit; `otool -l` reports a different
   fact and the two disagree under a linker-side floor bump. *)
let weak_externals path =
  let lines = run_lines "nm" [ "-m"; path ] in
  List.fold_left
    (fun (autolink, other) line ->
      if contains line "(undefined)" && contains line "weak external" then
        if contains line "_swift_FORCE_LOAD_" then (autolink + 1, other)
        else (autolink, other + 1)
      else (autolink, other))
    (0, 0) lines

let weak_external_names path =
  List.filter
    (fun line ->
      contains line "(undefined)"
      && contains line "weak external"
      && not (contains line "_swift_FORCE_LOAD_"))
    (run_lines "nm" [ "-m"; path ])

let check_floor floor path =
  match minos path with
  | None -> fail "%s carries no LC_BUILD_VERSION at all" (Filename.basename path)
  | Some observed when observed = floor ->
      pass "%s minos %s = the declared floor" (Filename.basename path) observed
  | Some observed ->
      fail "%s minos %s, declared floor %s — the flag did not reach this stanza"
        (Filename.basename path) observed floor

let check_weak_externals path =
  let autolink, other = weak_externals path in
  Printf.printf
    "      %s: %d _swift_FORCE_LOAD_ autolink stubs, %d other undefined weak \
     externals\n"
    (Filename.basename path) autolink other;
  if autolink = 0 then
    fail
      "%s reports 0 _swift_FORCE_LOAD_ autolink stubs — the control arm is gone, so \
       the zero beside it is a statement about the filter, not about the artifact"
      (Filename.basename path);
  if other > 0 then begin
    fail
      "%s reports %d undefined weak external(s) that are not autolink stubs — each \
       one is an SDK symbol above the declared floor, NULL on a host at that floor"
      (Filename.basename path) other;
    List.iter (fun line -> print_endline ("        " ^ String.trim line)) (weak_external_names path)
  end
  else pass "%s has no undefined weak external outside the autolink stubs" (Filename.basename path)

let read_floor file =
  let channel = open_in file in
  let line = String.trim (input_line channel) in
  close_in channel;
  line

let usage () =
  prerr_endline
    "usage: check_floor dylib <floor-file> <mach-o>\n\
    \       check_floor exe   <floor-file> <mach-o> [<mach-o>…]";
  exit 2

let () =
  let argv = Array.to_list Sys.argv in
  match argv with
  | _ :: mode :: floor_file :: (_ :: _ as paths) ->
      let floor = read_floor floor_file in
      Printf.printf "deployment floor: %s (%s)\n" floor floor_file;
      (match mode with
      | "dylib" ->
          List.iter
            (fun path ->
              check_floor floor path;
              check_weak_externals path)
            paths
      | "exe" -> List.iter (check_floor floor) paths
      | _ -> usage ());
      if !failures > 0 then begin
        Printf.printf "\n%d check(s) failed\n" !failures;
        exit 1
      end
  | _ -> usage ()
