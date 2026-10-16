# Extract the book's complete examples from Markdown pages into a staging
# directory, and report every page-structure error. Used only by
# scripts/examples.sh; the convention it reads is written in
# docs/book/examples.md.
#
#   awk -v out=STAGE -f examples-extract.awk PAGE...
#
# Output under STAGE:
#   errors                 one "PAGE:LINE: message" per line (absent when clean)
#   counts                 "fragments N"
#   ex-NNNN/where          PAGE:LINE of the example's label paragraph
#   ex-NNNN/requires       one required platform feature per line (may be absent)
#   ex-NNNN/files/PATH     a file block's bytes; ex-NNNN/executable lists bin/ paths
#   ex-NNNN/run-K/command  the shell text of the K-th command
#   ex-NNNN/run-K/stdout{,.mode}, stderr{,.mode}, status
#
# POSIX awk only (macOS ships a BSD awk with no gensub or asorti).

BEGIN {
  nexamples = 0
  nfragments = 0
  # Languages whose blocks are never examples. A block outside an example in any
  # other language, or with no language, is an error: it fails closed, so a
  # transcript tagged shell-session or left bare cannot go unchecked.
  split("text rust toml yaml yml markdown md diff ini xml html ruby python plantuml d2", names, " ")
  for (i in names) nonrunnable[names[i]] = 1
  known_requirement["confinement"] = 1
  file_open = 0
}

function fail(line, message) {
  print page ":" line ": " message >> (out "/errors")
  close(out "/errors")
}

function write_file(path, text) {
  printf "%s", text > path
  close(path)
}

# Remove blockquote markers so an example set in a note is read like any other.
# qdepth is how many were removed; a fence's content lines lose exactly as many,
# so a claimed "> hi" inside a quoted block keeps its marker.
function unquote(s) {
  qdepth = 0
  while (match(s, /^ ? ? ?> ?/)) {
    s = substr(s, RLENGTH + 1)
    qdepth++
  }
  return s
}

function unquote_n(s, n,   i) {
  for (i = 0; i < n; i++)
    if (match(s, /^ ? ? ?> ?/)) s = substr(s, RLENGTH + 1)
  return s
}

function begin_page(name) {
  if (infence) fail(fence_line, "a fenced block is never closed")
  finish_paragraph()
  end_example()
  ctx = ""
  infence = 0
  para = ""
  para_line = 0
  pending = ""
  fragment_pending = 0
  crlf_reported = 0
  page = name
}

# --- paragraph and example state -------------------------------------------

function start_example(line) {
  end_example()
  nexamples++
  exdir = sprintf("%s/ex-%04d", out, nexamples)
  system("mkdir -p '" exdir "/files'")
  write_file(exdir "/where", page ":" line "\n")
  ctx = "complete"
  ctx_line = line
  runs = 0
  files_closed = 0
  delete seen
  delete ex_requires
  nrequires = 0
}

function end_example(   k, ok) {
  if (ctx == "complete") {
    if (runs == 0) fail(ctx_line, "the complete example has no Command block")
    for (k = 1; k <= runs; k++) {
      if (!(has_stdout[k])) fail(ctx_line, "command " k " of the complete example has no expected standard output block (an empty block claims empty output)")
      if (!(has_status[k])) fail(ctx_line, "command " k " of the complete example has no Exit status block")
    }
  }
  ctx = ""
  pending = ""
  delete has_stdout
  delete has_status
  delete has_stderr
  runs = 0
}

function finish_paragraph(   first, rest, label, lines) {
  if (para == "") return
  fragment_pending = 0
  first = para
  sub(/\n.*/, "", first)
  rest = first
  sub(/^[*_]+/, "", rest)
  if (rest ~ /^Complete example/) {
    start_example(para_line)
  } else if (rest ~ /^Fragment/) {
    # A fragment label covers the one block directly after it.
    end_example()
    fragment_pending = 1
    nfragments++
    para = ""
    para_line = 0
    return
  } else if (ctx == "complete" && para !~ /\n/ && para ~ /^\*\*.*\*\*$/) {
    label = para
    sub(/^\*\*/, "", label)
    sub(/\*\*$/, "", label)
    pending = label
    pending_line = para_line
  } else {
    pending = ""
  }
  para = ""
  para_line = 0
}

# --- fenced blocks -----------------------------------------------------------

function handle_block(lang, start, text, nlines,   label, path, k, body) {
  lang = tolower(lang)
  if (ctx == "") {
    if (fragment_pending) { fragment_pending = 0; return }
    if (lang == "")
      fail(start, "a fenced block has no language tag; tag it, and introduce it with a paragraph beginning \"Complete example\" or \"Fragment\" if it is runnable")
    else if (!(lang in nonrunnable))
      fail(start, "a " lang " block sits outside a labelled example: introduce it with a paragraph beginning \"Complete example\" or \"Fragment\" (a language that is never an example is listed in scripts/examples-extract.awk)")
    return
  }
  label = pending
  pending = ""
  if (label == "") {
    fail(start, "a block in a complete example has no label paragraph directly before it")
    return
  }
  body = text
  if (label ~ /^`.*`$/) {
    path = label
    gsub(/^`|`$/, "", path)
    if (path == "" || path ~ /^\// || path ~ /(^|\/)\.\.(\/|$)/ || path ~ /\/\// || path ~ /\/$/ || path !~ /^[A-Za-z0-9._\/-]+$/) {
      fail(start, "file label `" path "` is not a plain relative path (letters, digits, . _ - and / only, no .. part) inside the example's directory")
      return
    }
    if (runs > 0) {
      fail(start, "file `" path "` comes after a Command block: files are written before any command runs")
      return
    }
    if (path in seen) {
      fail(start, "file `" path "` is given twice")
      return
    }
    seen[path] = 1
    k = path
    sub(/\/[^\/]*$/, "", k)
    if (k != path) system("mkdir -p '" exdir "/files/" k "'")
    write_file(exdir "/files/" path, (nlines > 0 ? body "\n" : ""))
    if (path ~ /^bin\//) {
      printf "%s\n", path >> (exdir "/executable")
      close(exdir "/executable")
    }
    return
  }
  if (label == "Command") {
    runs++
    rundir = sprintf("%s/run-%d", exdir, runs)
    system("mkdir -p '" rundir "'")
    if (body !~ /^\$ /) {
      fail(start, "a Command block's first line must begin with \"$ \"")
      return
    }
    sub(/^\$ /, "", body)
    gsub(/\n\$ /, "\n", body)
    write_file(rundir "/command", body "\n")
    write_file(rundir "/stderr.mode", "none\n")
    return
  }
  if (runs == 0) {
    fail(start, "the \"" label "\" block comes before any Command block")
    return
  }
  if (label ~ /^Standard output/ && label != "Standard output" && label != "Standard output, without a final newline") {
    fail(start, "unknown label \"" label "\" in a complete example")
  } else if (label == "Standard output" || label == "Standard output, without a final newline") {
    if (has_stdout[runs]) { fail(start, "command " runs " has two standard output blocks"); return }
    write_file(rundir "/stdout", (label == "Standard output" && nlines > 0 ? body "\n" : (label == "Standard output" ? "" : body)))
    write_file(rundir "/stdout.mode", "exact\n")
    has_stdout[runs] = 1
  } else if (label == "Standard error" || label == "Standard error, without a final newline" || label == "Standard error includes") {
    if (has_stderr[runs]) { fail(start, "command " runs " has two standard error blocks (one claim per command: exact, exact without a final newline, or includes)"); return }
    has_stderr[runs] = 1
    if (label == "Standard error includes") {
      if (body == "") { fail(start, "an empty Standard error includes block claims nothing"); return }
      write_file(rundir "/stderr", body)
      write_file(rundir "/stderr.mode", "includes\n")
    } else {
      write_file(rundir "/stderr", (label == "Standard error" && nlines > 0 ? body "\n" : (label == "Standard error" ? "" : body)))
      write_file(rundir "/stderr.mode", "exact\n")
    }
  } else if (label == "Exit status") {
    if (body !~ /^[0-9]+$/) {
      fail(start, "an Exit status block holds one non-negative integer")
      return
    }
    if (has_status[runs]) { fail(start, "command " runs " has two Exit status blocks"); return }
    write_file(rundir "/status", body "\n")
    has_status[runs] = 1
  } else {
    fail(start, "unknown label \"" label "\" in a complete example")
  }
}

# --- directives ----------------------------------------------------------------

function directive(line, text,   name) {
  # <!-- example: requires NAME -->
  if (text !~ /^<!--[ ]*example:/) return
  if (ctx != "complete") {
    fail(line, "an example directive sits outside a complete example")
    return
  }
  if (match(text, /^<!--[ ]*example:[ ]*requires[ ]+[a-z][a-z-]*[ ]*-->$/)) {
    name = text
    sub(/^<!--[ ]*example:[ ]*requires[ ]+/, "", name)
    sub(/[ ]*-->$/, "", name)
    if (!(name in known_requirement)) {
      fail(line, "unknown requirement \"" name "\" (the known requirements are listed in docs/book/examples.md)")
      return
    }
    printf "%s\n", name >> (exdir "/requires")
    close(exdir "/requires")
  } else {
    fail(line, "malformed example directive; the only one is <!-- example: requires NAME -->")
  }
}

# --- line loop ---------------------------------------------------------------------

FNR == 1 { begin_page(FILENAME) }

{
  raw = $0
  if (raw ~ /\r$/) {
    if (!crlf_reported) {
      fail(FNR, "the page has CRLF line endings; convert it to LF, because a claimed stream is compared byte for byte")
      crlf_reported = 1
    }
    sub(/\r$/, "", raw)
  }
  line = unquote(raw)
}

infence {
  line = unquote_n(raw, fence_qdepth)
  if (match(line, /^ ? ? ?(```+|~~~+)[ \t]*$/)) {
    closer = line
    gsub(/[ \t]/, "", closer)
    if (substr(closer, 1, 1) == fence_char && length(closer) >= fence_len) {
      handle_block(fence_lang, fence_line, block, nblock)
      infence = 0
      next
    }
  }
  block = (nblock == 0 ? line : block "\n" line)
  nblock++
  next
}

{
  if (match(line, /^ ? ? ?(```+|~~~+)/)) {
    finish_paragraph()
    opener = line
    sub(/^ +/, "", opener)
    fence_char = substr(opener, 1, 1)
    fence_len = 0
    while (substr(opener, fence_len + 1, 1) == fence_char) fence_len++
    info = substr(opener, fence_len + 1)
    sub(/^[ \t]+/, "", info)
    fence_lang = info
    sub(/[ \t,{].*$/, "", fence_lang)
    infence = 1
    fence_line = FNR
    fence_qdepth = qdepth
    block = ""
    nblock = 0
    next
  }
  if (line ~ /^ ? ? ?([-*+]|[0-9]+[.)])[ \t]+(```|~~~)/) {
    fail(FNR, "a fence opened on a list-item line is not read by this checker; start the fence on its own line")
    next
  }
  if (line ~ /^(\t|    +)(```|~~~)/) {
    fail(FNR, "a fence indented four or more spaces is not read by this checker; unindent it or move it out of the list")
    next
  }
  if (line ~ /^(\t|    +)\$ /) {
    fail(FNR, "an indented code block starting with \"$ \" is a command transcript the checker cannot see; fence and label it")
    next
  }
  if (line ~ /^<!--.*-->[ \t]*$/) {
    directive(FNR, line)
    next
  }
  if (line ~ /^[ \t]*$/) {
    finish_paragraph()
    next
  }
  if (line ~ /^#+[ \t]/) {
    finish_paragraph()
    end_example()
    next
  }
  if (para == "") para_line = FNR
  para = (para == "" ? line : para "\n" line)
}

END {
  if (infence) fail(fence_line, "a fenced block is never closed")
  finish_paragraph()
  end_example()
  write_file(out "/counts", "fragments " nfragments "\n")
}
