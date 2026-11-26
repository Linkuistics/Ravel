//! The values at the WebAssembly boundary that need more than a derive:
//! `docs/adr/typed-objects-at-the-webassembly-boundary.md`.
//!
//! The other types that cross the boundary carry their derives themselves,
//! behind the feature `wasm`. A unit enum crosses as the `kebab-case` name of
//! its variant. The tests of this module check that each such name is the code
//! of the KDL form, so the page and the files use the same codes.

use crate::trace::{AttributeValue, Float};

impl AttributeValue {
    /// The text of a number, as the trace file writes it. An integer has its
    /// decimal text. A float has the text of the `kdl` crate, such as `2.5`,
    /// `1e300` or `#nan`. Any other value has no number text.
    pub fn number_text(&self) -> Option<String> {
        match self {
            Self::Integer(n) => Some(n.to_string()),
            Self::Float(Float(f)) => Some(kdl::KdlValue::Float(*f).to_string()),
            Self::String(_) | Self::Bool(_) | Self::Null => None,
        }
    }
}

/// The form of the section "The WebAssembly boundary" of the tracer spec: a
/// number crosses as its text, so no value loses precision. The declaration
/// of the type sits on `AttributeValue`.
#[cfg(feature = "wasm")]
impl serde::Serialize for AttributeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let fields = if matches!(self, Self::Null) { 1 } else { 2 };
        let mut value = serializer.serialize_struct("AttributeValue", fields)?;
        match self {
            Self::String(text) => {
                value.serialize_field("type", "string")?;
                value.serialize_field("value", text)?;
            }
            Self::Integer(_) | Self::Float(_) => {
                let kind = if matches!(self, Self::Integer(_)) { "integer" } else { "float" };
                value.serialize_field("type", kind)?;
                value.serialize_field("value", &self.number_text())?;
            }
            Self::Bool(b) => {
                value.serialize_field("type", "bool")?;
                value.serialize_field("value", b)?;
            }
            Self::Null => value.serialize_field("type", "null")?,
        }
        value.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        checked::{Badge, Code, Severity},
        diagram::{FlowRole, GatewayRole, LoopLink},
        trace::{LoopKind, TraceCode},
    };

    /// The rule of serde for `rename_all = "kebab-case"` on a variant name.
    fn kebab(variant: &str) -> String {
        let mut text = String::new();
        for (i, c) in variant.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                text.push('-');
            }
            text.extend(c.to_lowercase());
        }
        text
    }

    #[test]
    fn each_unit_enum_crosses_as_its_kdl_code() {
        for code in Code::ALL {
            assert_eq!(kebab(&format!("{code:?}")), code.as_str());
        }
        for code in TraceCode::ALL {
            assert_eq!(kebab(&format!("{code:?}")), code.as_str());
        }
        for badge in [Badge::NoSelfEmbedding, Badge::MayNeedAStack] {
            assert_eq!(kebab(&format!("{badge:?}")), badge.as_str());
        }
        for kind in [LoopKind::Repetition, LoopKind::TailReferences, LoopKind::HeadReferences] {
            assert_eq!(kebab(&format!("{kind:?}")), kind.as_str());
        }
        for severity in [Severity::Error, Severity::Warning] {
            assert_eq!(kebab(&format!("{severity:?}")), severity.as_str());
        }
        for role in GatewayRole::ALL {
            assert_eq!(kebab(&format!("{role:?}")), role.as_str());
        }
        for role in FlowRole::ALL {
            assert_eq!(kebab(&format!("{role:?}")), role.as_str());
        }
        for link in LoopLink::ALL {
            assert_eq!(kebab(&format!("{link:?}")), link.as_str());
        }
    }

    #[test]
    fn a_number_crosses_as_the_text_of_the_trace_file() {
        let text = |value: AttributeValue| value.number_text();
        assert_eq!(text(AttributeValue::Integer(16)), Some("16".into()));
        assert_eq!(text(AttributeValue::Integer(i128::MIN)), Some(i128::MIN.to_string()));
        assert_eq!(text(AttributeValue::Float(Float(2.5))), Some("2.5".into()));
        assert_eq!(text(AttributeValue::Float(Float(1e300))), Some("1e300".into()));
        assert_eq!(text(AttributeValue::Float(Float(f64::NAN))), Some("#nan".into()));
        assert_eq!(text(AttributeValue::Float(Float(f64::NEG_INFINITY))), Some("#-inf".into()));
        assert_eq!(text(AttributeValue::Bool(true)), None);
    }
}
