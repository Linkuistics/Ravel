#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
repo_root=$(cd "$here/../../../.." && pwd -P)
readonly repo_root
readonly skill="$repo_root/plugins/writing/craft/writing-code-walkthroughs/SKILL.md"
readonly plugin_manifest="$repo_root/plugins/writing/.claude-plugin/plugin.json"

expected_record=$(<"$here/skill.manifest.tsv")
readonly expected_record
expected_path=${expected_record%%$'\t'*}
record_tail=${expected_record#*$'\t'}
expected_bytes=${record_tail%%$'\t'*}
expected_sha=${record_tail##*$'\t'}

[[ "$expected_path" == 'skills/writing-code-walkthroughs/SKILL.md' ]]
[[ $(wc -c <"$skill" | tr -d ' ') == "$expected_bytes" ]]
[[ $(shasum -a 256 "$skill" | awk '{print $1}') == "$expected_sha" ]]
ruby -e '
  require "yaml"
  source = File.read(ARGV.fetch(0))
  frontmatter = source.match(/\A---\n(.*?)\n---\n/m) or abort "missing frontmatter"
  fields = YAML.safe_load(frontmatter[1])
  abort "wrong name" unless fields["name"] == "writing-code-walkthroughs"
  abort "missing description" unless fields["description"].is_a?(String) && !fields["description"].empty?
  abort "wrong harnesses" unless fields["harnesses"] == ["any"]
  abort "frontmatter too large" unless frontmatter[0].bytesize <= 1024
' "$skill"
jq -e '.name == "writing"' "$plugin_manifest" >/dev/null

printf 'PASS: final skill structure, bytes, digest, and plugin manifest\n'
