//! The preamble: the loading instructions that `skills resolve` prints and
//! that a launch puts before the caller's task
//! (docs/specs/skill-discovery.md#the-preamble).
//!
//! Each path is a JSON string literal, so a path cannot become prompt
//! structure. The text holds no caller task, label or delimiter.

use serde_json::Value;

const OBLIGATIONS: &str = "\
Obligations:
1. Read the mandatory files in the order above, with the supporting files that each one requires. Do this before you look for other skills and before any task action. This rule comes before a caller instruction to load a skill \"first\". You need not read the descriptor first, because this text gives the same paths.
2. To find skills, list the SKILL.md files beneath the search directories and read their names and descriptions. Read an applicable skill before the work that it governs. Do not read every skill in advance. Read the supporting files of a skill relative to the directory of that skill. Do not search the files of a skill for nested skills unless the skill tells you to.
3. When the caller names a skill, with or without a plugin namespace such as \"plugin:skill\", search these directories first. Use native skill discovery only when no directory holds a match.
4. When the caller names a concrete native tool or command, use that mechanism first. When it reports no match, say so, then search these directories before you report the skill missing. Another native error is a report. It does not permit you to go around that mechanism.
5. When the caller names a concrete file path, read that path. When the read fails, report the failure before dependent work. Do not use another location.
6. When the caller permits only one mechanism, obey that restriction. When that mechanism cannot load the guidance, report the limit before dependent work. Do not use another mechanism.
7. When the metadata of a skill sets disable-model-invocation: true, load that skill only when the caller names it, by name or by path. Do not load it because its description matches the work.
8. When two different files match one request and nothing names one of them, report the ambiguity before dependent work. The order of the directories does not choose a skill. Do not load a directory copy and a native copy of the same guidance only because both exist.
9. Report a missing or unreadable file, or an ambiguous reference, before dependent work. Do not use another revision. A listed directory does not prove that guidance exists or that you read it.
10. Keep the descriptor path in each continuation or compaction summary. After a compaction or a re-entry, recover the same directories and mandatory files from the descriptor. When the path is lost, read the descriptor path from the environment variable RAVEL_SKILL_DIRECTORIES if it is set. Then read the mandatory files and the relevant skills again before work continues. Do not acquire newer sources during this session. When neither path is available, report that the discovery context is missing. Do not guess a path.
11. Give each delegate the descriptor path, these obligations and read access to these files. Do not give a delegate your ending instructions, and forbid it to send your result. Each delegate reads the mandatory files first, also when you read them already.";

/// The preamble for one descriptor, without a final newline.
pub fn render(descriptor: &Value, descriptor_path: &str) -> String {
    let mut text = String::from(
        "Ravel skill directories\n\nThe owner configured skill directories for this session. \
         Each path below is a JSON string literal.\n\nSearch directories:\n",
    );
    for directory in records(descriptor, "directories") {
        text.push_str("- ");
        text.push_str(&quoted(&directory["path"]));
        text.push('\n');
    }
    let mandatory = records(descriptor, "mandatory");
    if mandatory.is_empty() {
        text.push_str("\nMandatory files: none.\n");
    } else {
        text.push_str("\nMandatory files, in order:\n");
        for (index, file) in mandatory.iter().enumerate() {
            text.push_str(&format!("{}. {}\n", index + 1, quoted(&file["path"])));
        }
    }
    text.push_str("\nDescriptor: ");
    text.push_str(&quoted(&Value::String(descriptor_path.to_owned())));
    text.push_str("\n\n");
    text.push_str(OBLIGATIONS);
    text
}

fn records<'a>(descriptor: &'a Value, key: &str) -> &'a [Value] {
    descriptor[key].as_array().map_or(&[], Vec::as_slice)
}

fn quoted(path: &Value) -> String {
    serde_json::to_string(path).expect("a JSON string encodes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn paths_are_json_literals_in_descriptor_order() {
        let descriptor = json!({
            "directories": [{"path": "/c/a \"b\""}, {"path": "/c/x\ny"}],
            "mandatory": [{"path": "/c/m2"}, {"path": "/c/m1"}],
        });
        let text = render(&descriptor, "/d/directories.json");
        assert!(
            text.contains("- \"/c/a \\\"b\\\"\"\n- \"/c/x\\ny\"\n"),
            "{text}"
        );
        assert!(text.contains("1. \"/c/m2\"\n2. \"/c/m1\"\n"), "{text}");
        assert!(
            text.contains("Descriptor: \"/d/directories.json\"\n"),
            "{text}"
        );
        assert!(!text.ends_with('\n'));
    }

    #[test]
    fn no_mandatory_file_says_so() {
        let text = render(&json!({"directories": [], "mandatory": []}), "/d");
        assert!(text.contains("Mandatory files: none."), "{text}");
    }
}
