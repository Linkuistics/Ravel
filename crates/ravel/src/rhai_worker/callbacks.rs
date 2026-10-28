//! Array functions that pass a callback error on instead of dropping it.
//!
//! Rhai 1.27.0 gives `dedup`, `sort` and `order` a comparer closure and discards
//! what the closure fails with: `dedup` turns an error into `false`, and `sort`
//! and `order` turn it into a comparison by type (`src/packages/array_basic.rs`).
//! A configuration script that breaches a call-depth, string-size or variable bound inside
//! such a closure would then go on to select, with no `try` at all. Every other function of the
//! array and map packages that calls a closure returns the closure's error, but
//! wrapped in one that `try` catches.
//!
//! The worker registers these replacements in the engine's own namespace,
//! which Rhai searches before its packages. They behave as the originals do
//! while the closure succeeds. When it fails they stop calling it and return the
//! first error, so a limit ends the evaluation here as it does in `map`.
//!
//! `pointer_calls` replaces the other functions that call a function pointer,
//! for the limits that `try` can still catch there. [`install`] registers both.

use std::cmp::Ordering;
use std::panic::{AssertUnwindSafe, catch_unwind};

use rhai::{Array, Engine, EvalAltResult, FnPtr, NativeCallContext, Position};

use super::pointer_calls::{self, register};

/// Register the replacements of `dedup`, of `sort` and `sort_by` and of `order`
/// and `order_by` with a comparer, and those of `pointer_calls`. Rhai declares
/// these functions impure, so a constant cannot be their receiver, and so do the
/// registrations.
pub fn install(engine: &mut Engine) {
    register!(engine, "dedup", false, dedup);
    register!(engine, "sort", false, sort_by);
    register!(engine, "sort_by", false, sort_by);
    // `order` and `order_by` call the original `sort_by` in Rust, so the
    // replacement of that name does not reach them.
    register!(engine, "order", false, order_by);
    register!(engine, "order_by", false, order_by);
    // The deprecated forms that take the name of the comparer.
    register!(engine, "dedup", false, dedup_named);
    register!(engine, "sort", false, sort_named);
    pointer_calls::install(engine);
}

fn dedup_named(
    context: NativeCallContext,
    array: &mut Array,
    comparer: &str,
) -> Result<(), Box<EvalAltResult>> {
    dedup(context, array, FnPtr::new(comparer)?)
}

fn sort_named(
    context: NativeCallContext,
    array: &mut Array,
    comparer: &str,
) -> Result<(), Box<EvalAltResult>> {
    sort_by(context, array, FnPtr::new(comparer)?)
}

/// `array.dedup(|a, b| ..)`: remove each element that the comparer calls equal to
/// the one before it.
fn dedup(
    context: NativeCallContext,
    array: &mut Array,
    comparer: FnPtr,
) -> Result<(), Box<EvalAltResult>> {
    let mut failed = None;
    array.dedup_by(|later, earlier| {
        if failed.is_some() {
            return false;
        }
        match comparer.call_raw(&context, None, [earlier.clone(), later.clone()]) {
            Ok(equal) => equal.as_bool().unwrap_or(false),
            Err(error) => {
                failed = Some(error);
                false
            }
        }
    });
    failed.map_or(Ok(()), Err)
}

/// `array.order_by(|a, b| ..)`: `sort_by` of a copy, which is the result.
fn order_by(
    context: NativeCallContext,
    array: &mut Array,
    comparer: FnPtr,
) -> Result<Array, Box<EvalAltResult>> {
    let mut ordered = array.clone();
    sort_by(context, &mut ordered, comparer)?;
    Ok(ordered)
}

/// `array.sort(|a, b| ..)`: order the elements by the comparer's integer, or by
/// its boolean (`true` puts `a` first).
fn sort_by(
    context: NativeCallContext,
    array: &mut Array,
    comparer: FnPtr,
) -> Result<(), Box<EvalAltResult>> {
    if array.len() <= 1 {
        return Ok(());
    }
    let mut failed = None;
    // `slice::sort_by` may panic when the comparer is no total order, as a
    // hostile one need not be. Rhai catches that, and so does this.
    let sorted = catch_unwind(AssertUnwindSafe(|| {
        array.sort_by(|left, right| {
            if failed.is_some() {
                return Ordering::Equal;
            }
            match comparer.call_raw(&context, None, [left.clone(), right.clone()]) {
                Ok(order) => order
                    .as_int()
                    .or_else(|_| order.as_bool().map(|first| if first { -1 } else { 1 }))
                    .map_or_else(|_| left.type_id().cmp(&right.type_id()), |n| n.cmp(&0)),
                Err(error) => {
                    failed = Some(error);
                    Ordering::Equal
                }
            }
        });
    }));
    if let Some(error) = failed {
        return Err(error);
    }
    sorted.map_err(|_| {
        EvalAltResult::ErrorRuntime("error in comparer for sorting".into(), Position::NONE).into()
    })
}
