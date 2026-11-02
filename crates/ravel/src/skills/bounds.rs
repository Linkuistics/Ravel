//! The count and byte bounds of skill resolution
//! (docs/specs/skill-discovery.md#bounds).
//!
//! Each bound is a maximum: a value equal to the bound passes, and one more
//! refuses as `skill_limit_exceeded`. A bound never truncates caller text,
//! omits a directory or a mandatory file, or disables delivery. The prepared
//! prompt has its own check in `launch`, and the frame bound has its own in
//! `frame`.

use super::refuse;
use crate::limits::{Bound, Origin};
use crate::refusal::Refusal;

const MIB: u64 = 1024 * 1024;

/// The bounds of one resolution. Only the unit tests use other values, so
/// that they can cross a bound that no public fixture can reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub sources: u64,
    pub search_directories: u64,
    pub mandatory_files: u64,
    pub mandatory_file_bytes: u64,
    pub resource_bytes: u64,
    pub source_bytes: u64,
    pub total_bytes: u64,
    pub source_entries: u64,
    pub descriptor_bytes: u64,
    pub preamble_bytes: u64,
}

impl Limits {
    /// The values of the specification.
    pub const SPECIFIED: Limits = Limits {
        sources: 16,
        search_directories: 256,
        mandatory_files: 256,
        mandatory_file_bytes: 65_536,
        resource_bytes: 16 * MIB,
        source_bytes: 128 * MIB,
        total_bytes: 512 * MIB,
        source_entries: 20_000,
        descriptor_bytes: MIB,
        preamble_bytes: MIB,
    };

    /// The bounds in effect for this resolution.
    pub fn current() -> Limits {
        #[cfg(test)]
        if let Some(limits) = super::faults::limits() {
            return limits;
        }
        Limits::SPECIFIED
    }
}

/// The value that a refusal reports: exact, or the count when Ravel stopped
/// at the bound. Capture stops at the first entry or byte that crosses a
/// bound, so the rest of a source is not counted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Actual {
    Exact(u64),
    AtLeast(u64),
}

impl Actual {
    fn text(self, unit: &str) -> String {
        match self {
            Actual::Exact(value) => format!("{value} {unit}"),
            Actual::AtLeast(value) => format!("at least {value} {unit}"),
        }
    }
}

/// One bound of the table, with its JSON name and unit, and the remedy that
/// suits it.
#[derive(Clone, Copy, Debug)]
pub struct Subject {
    pub name: &'static str,
    pub unit: &'static str,
    pub remedy: &'static str,
}

pub const SOURCES: Subject = Subject {
    name: "skillSources",
    unit: "sources",
    remedy: "configure fewer skill sources; Ravel never omits a configured source",
};
pub const SEARCH_DIRECTORIES: Subject = Subject {
    name: "skillSearchDirectories",
    unit: "directories",
    remedy: "configure fewer roots, for example one parent directory in place of its children; \
             Ravel never omits a configured directory",
};
pub const MANDATORY_FILES: Subject = Subject {
    name: "skillMandatoryFiles",
    unit: "files",
    remedy: "configure fewer mandatory files, for example one file that refers to the others; \
             Ravel never omits mandatory guidance",
};
pub const MANDATORY_FILE_BYTES: Subject = Subject {
    name: "mandatoryFileBytes",
    unit: "bytes",
    remedy: "make the mandatory file smaller, and move its detail to supporting files that it \
             refers to; Ravel never truncates mandatory guidance",
};
pub const RESOURCE_BYTES: Subject = Subject {
    name: "resourceBytes",
    unit: "bytes",
    remedy: "move the large file out of the skill source, or make it smaller; Ravel never omits \
             a file of a source",
};
pub const SOURCE_BYTES: Subject = Subject {
    name: "sourceBytes",
    unit: "bytes",
    remedy: "configure a smaller source directory, or move large files out of it; Ravel never \
             omits a file of a source",
};
pub const TOTAL_BYTES: Subject = Subject {
    name: "totalSourceBytes",
    unit: "bytes",
    remedy: "configure fewer or smaller skill sources; Ravel never omits a configured source",
};
pub const SOURCE_ENTRIES: Subject = Subject {
    name: "sourceEntries",
    unit: "entries",
    remedy: "configure a source directory with fewer files, directories and links; Ravel never \
             omits an entry of a source",
};
pub const DESCRIPTOR_BYTES: Subject = Subject {
    name: "descriptorBytes",
    unit: "bytes",
    remedy: "configure fewer search directories or mandatory files, or shorter paths; Ravel \
             never omits a configured directory or mandatory file",
};
pub const PREAMBLE_BYTES: Subject = Subject {
    name: "preambleBytes",
    unit: "bytes",
    remedy: "configure fewer search directories or mandatory files, or shorter paths; Ravel \
             never omits a configured directory or mandatory file",
};

/// The refusal for `actual` over `bound`. `what` names the subject, such as
/// `the regular files of the source "a"`, and the caller adds a location or
/// a source path.
pub fn exceeded(subject: Subject, bound: u64, actual: Actual, what: &str) -> Refusal {
    refuse(
        "skill_limit_exceeded",
        format!(
            "{what}: {}, over the bound of {bound} {}",
            actual.text(subject.unit),
            subject.unit
        ),
        subject.remedy,
    )
    .bound(Bound {
        name: subject.name,
        unit: subject.unit,
        value: bound,
        origin: Origin::Fixed,
    })
}

/// Refuse a count over its bound.
pub fn count(subject: Subject, bound: u64, count: usize, what: &str) -> Result<(), Refusal> {
    let count = count as u64;
    if count > bound {
        return Err(exceeded(subject, bound, Actual::Exact(count), what));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_count_equal_to_its_bound_passes_and_one_more_refuses() {
        assert!(count(SOURCES, 16, 16, "the configured sources").is_ok());
        let refusal = count(SOURCES, 16, 17, "the configured sources").unwrap_err();
        assert_eq!(refusal.code, "skill_limit_exceeded");
        assert_eq!(refusal.exit, 3);
        assert_eq!(
            refusal.message,
            "the configured sources: 17 sources, over the bound of 16 sources"
        );
        let bound = refusal.bound.unwrap();
        assert_eq!((bound.name, bound.value), ("skillSources", 16));
    }

    #[test]
    fn a_value_counted_to_a_stop_reports_at_least() {
        let refusal = exceeded(SOURCE_ENTRIES, 3, Actual::AtLeast(4), "the source \"a\"");
        assert!(
            refusal.message.contains("at least 4 entries"),
            "{}",
            refusal.message
        );
    }
}
