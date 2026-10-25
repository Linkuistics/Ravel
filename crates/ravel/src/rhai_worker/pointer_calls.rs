//! Array and map functions that return a limit error of a function-pointer call
//! unwrapped, so that `try` cannot catch it.
//!
//! Rhai 1.27.0 passes an engine limit (call depth, data size, variable count)
//! up through a script function without wrapping it, and `try` does not catch
//! it. But `FnPtr::call_raw_with_extra_args` wraps every error in
//! `ErrorInFunctionCall`, and `EvalAltResult::is_catchable` is true for that
//! wrapper whatever it holds (`src/types/fn_ptr.rs:653` and
//! `src/types/error.rs:320`). Sixteen functions of the array and map packages
//! call a pointer that way, so `try { [1].map(|x| up(0)) } catch (e) { }`
//! swallowed a breach. The deprecated forms that take a function *name*
//! (`[1].map("f")`) call the same Rust functions.
//!
//! The worker registers a replacement for each of them in the engine's own
//! namespace, which Rhai searches before its packages. A replacement does what
//! the original does, with one change at the end of the call: an error that
//! `try` could not catch comes back as it is, and every other error is wrapped
//! as before. So an ordinary `throw` in a closure is still caught, and a breach
//! ends the evaluation.
//!
//! The list is the call sites of `call_raw_with_extra_args` in 1.27.0, which the
//! pin `=1.27.0` fixes. The tests of this module run each replacement against
//! the original, and `engine.rs` runs each one under `try` with a breach.

use std::mem;

use rhai::{Array, Dynamic, Engine, EvalAltResult, FnPtr, INT, Map, NativeCallContext, Position};

/// Register `function` as `name` in the engine's own namespace. `pure` is what
/// the original function declares: a pure method may be called on a constant.
/// A macro, because the bound of the registration names a type Rhai keeps
/// private.
macro_rules! register {
    ($engine:expr, $name:literal, $pure:literal, $function:expr) => {
        ::rhai::FuncRegistration::new($name)
            .with_purity($pure)
            .register_into_engine($engine, $function)
    };
}
pub(super) use register;

/// What a function of the array and map packages returns.
type Outcome<T> = Result<T, Box<EvalAltResult>>;

/// Register every replacement of this module.
pub fn install(engine: &mut Engine) {
    // Array functions that take a function pointer.
    register!(engine, "for_each", false, for_each);
    register!(engine, "map", true, map);
    register!(engine, "filter", true, filter);
    register!(engine, "find", true, find);
    register!(engine, "find", true, find_from);
    register!(engine, "find_map", true, find_map);
    register!(engine, "find_map", true, find_map_from);
    register!(engine, "index_of", true, index_of);
    register!(engine, "index_of", true, index_of_from);
    register!(engine, "some", true, some);
    register!(engine, "all", true, all);
    register!(engine, "reduce", true, reduce);
    register!(engine, "reduce", true, reduce_with);
    register!(engine, "reduce_rev", true, reduce_rev);
    register!(engine, "reduce_rev", true, reduce_rev_with);
    register!(engine, "zip", true, zip);
    register!(engine, "drain", false, drain);
    register!(engine, "retain", false, retain);
    // The deprecated forms that take the name of a function.
    register!(engine, "map", false, map_named);
    register!(engine, "filter", false, filter_named);
    register!(engine, "index_of", true, index_of_named);
    register!(engine, "index_of", true, index_of_named_from);
    register!(engine, "some", true, some_named);
    register!(engine, "all", true, all_named);
    register!(engine, "reduce", true, reduce_named);
    register!(engine, "reduce", true, reduce_named_with);
    register!(engine, "reduce_rev", true, reduce_rev_named);
    register!(engine, "reduce_rev", true, reduce_rev_named_with);
    // Map functions that take a function pointer.
    register!(engine, "map", true, map_of_map);
    register!(engine, "filter", true, filter_of_map);
    register!(engine, "drain", false, drain_of_map);
    register!(engine, "retain", false, retain_of_map);
}

/// `FnPtr::call_raw_with_extra_args` of Rhai 1.27.0, with one change: an error
/// that `try` could not catch is not wrapped.
///
/// The call tries the arguments alone first. When the pointer names no function
/// of that arity it tries again with `extras` (the index of the element) after
/// them. `move_this` is `Some` when `this` may also move into the arguments, at
/// that position.
fn call<const N: usize, const E: usize>(
    pointer: &FnPtr,
    caller: &str,
    context: &NativeCallContext,
    mut this: Option<&mut Dynamic>,
    args: [Dynamic; N],
    extras: [Dynamic; E],
    move_this: Option<usize>,
) -> Outcome<Dynamic> {
    let name = pointer.fn_name();
    let insert = |this: &Dynamic, arguments: Vec<Dynamic>, position: usize| {
        let mut moved = Vec::with_capacity(arguments.len() + 1);
        if position == 0 {
            moved.push(this.clone());
            moved.extend(arguments);
        } else {
            moved.extend(arguments);
            moved.insert(position, this.clone());
        }
        moved
    };
    pointer
        .call_raw(context, this.as_deref_mut(), args.clone())
        .or_else(|error| match *error {
            EvalAltResult::ErrorFunctionNotFound(ref signature, ..)
                if move_this.is_some() && this.is_some() && signature.starts_with(name) =>
            {
                let moved = insert(
                    this.as_deref().unwrap(),
                    args.to_vec(),
                    move_this.unwrap_or(0),
                );
                pointer.call_raw(context, None, moved)
            }
            _ => Err(error),
        })
        .or_else(|error| match *error {
            EvalAltResult::ErrorFunctionNotFound(ref signature, ..)
                if signature.starts_with(name) =>
            {
                let all: Vec<Dynamic> = args.into_iter().chain(extras).collect();
                if let (Some(position), Some(this)) = (move_this, this.as_deref()) {
                    return pointer.call_raw(context, None, insert(this, all, position));
                }
                pointer.call_raw(context, this, all)
            }
            _ => Err(error),
        })
        .map_err(|error| {
            if error.is_catchable() {
                Box::new(EvalAltResult::ErrorInFunctionCall(
                    caller.to_owned(),
                    context.call_source().unwrap_or("").to_owned(),
                    error,
                    Position::NONE,
                ))
            } else {
                error
            }
        })
}

/// The index of an element, as the extra argument of a call.
fn at(index: usize) -> [Dynamic; 1] {
    [INT::try_from(index).unwrap_or(INT::MAX).into()]
}

/// Where a search that starts at `start` begins: a negative `start` counts from
/// the end, and one past the end finds nothing (`calc_offset_len` in Rhai).
fn offset(length: usize, start: INT) -> usize {
    if start < 0 {
        usize::try_from(start.unsigned_abs()).map_or(0, |back| length - back.min(length))
    } else {
        usize::try_from(start).map_or(length, |start| start.min(length))
    }
}

/// `pointer` of a function name, as the deprecated forms build it.
fn named(name: &str) -> Outcome<FnPtr> {
    FnPtr::new(name)
}

fn for_each(context: NativeCallContext, array: &mut Array, map: FnPtr) -> Outcome<()> {
    for (index, item) in array.iter_mut().enumerate() {
        let _ = call(&map, "for_each", &context, Some(item), [], at(index), None)?;
    }
    Ok(())
}

fn map(context: NativeCallContext, array: &mut Array, map: FnPtr) -> Outcome<Array> {
    let mut mapped = Array::with_capacity(array.len());
    for (index, item) in array.iter_mut().enumerate() {
        mapped.push(call(
            &map,
            "map",
            &context,
            Some(item),
            [],
            at(index),
            Some(0),
        )?);
    }
    Ok(mapped)
}

fn filter(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<Array> {
    let mut kept = Array::new();
    for (index, item) in array.iter_mut().enumerate() {
        let keep = call(
            &filter,
            "filter",
            &context,
            Some(&mut *item),
            [],
            at(index),
            Some(0),
        )?;
        if keep.as_bool().unwrap_or(false) {
            kept.push(item.clone());
        }
    }
    Ok(kept)
}

/// The index of the first element from `start` for which `filter` is true, or -1.
fn search(
    context: &NativeCallContext,
    array: &mut Array,
    filter: &FnPtr,
    start: INT,
) -> Outcome<INT> {
    let first = offset(array.len(), start);
    for (index, item) in array.iter_mut().enumerate().skip(first) {
        let found = call(
            filter,
            "index_of",
            context,
            Some(item),
            [],
            at(index),
            Some(0),
        )?;
        if found.as_bool().unwrap_or(false) {
            return Ok(INT::try_from(index).unwrap_or(INT::MAX));
        }
    }
    Ok(-1)
}

fn index_of(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<INT> {
    search(&context, array, &filter, 0)
}

fn index_of_from(
    context: NativeCallContext,
    array: &mut Array,
    filter: FnPtr,
    start: INT,
) -> Outcome<INT> {
    search(&context, array, &filter, start)
}

fn find_from(
    context: NativeCallContext,
    array: &mut Array,
    filter: FnPtr,
    start: INT,
) -> Outcome<Dynamic> {
    let index = search(&context, array, &filter, start)?;
    Ok(usize::try_from(index)
        .ok()
        .and_then(|index| array.get(index))
        .cloned()
        .unwrap_or(Dynamic::UNIT))
}

fn find(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<Dynamic> {
    find_from(context, array, filter, 0)
}

fn find_map_from(
    context: NativeCallContext,
    array: &mut Array,
    filter: FnPtr,
    start: INT,
) -> Outcome<Dynamic> {
    let first = offset(array.len(), start);
    for (index, item) in array.iter_mut().enumerate().skip(first) {
        let value = call(
            &filter,
            "find_map",
            &context,
            Some(item),
            [],
            at(index),
            Some(0),
        )?;
        if !value.is_unit() {
            return Ok(value);
        }
    }
    Ok(Dynamic::UNIT)
}

fn find_map(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<Dynamic> {
    find_map_from(context, array, filter, 0)
}

fn some(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<bool> {
    for (index, item) in array.iter_mut().enumerate() {
        let hit = call(
            &filter,
            "some",
            &context,
            Some(item),
            [],
            at(index),
            Some(0),
        )?;
        if hit.as_bool().unwrap_or(false) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn all(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<bool> {
    for (index, item) in array.iter_mut().enumerate() {
        let hit = call(&filter, "all", &context, Some(item), [], at(index), Some(0))?;
        if !hit.as_bool().unwrap_or(false) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn reduce_with(
    context: NativeCallContext,
    array: &mut Array,
    reducer: FnPtr,
    initial: Dynamic,
) -> Outcome<Dynamic> {
    array
        .iter_mut()
        .enumerate()
        .try_fold(initial, |result, (index, item)| {
            call(
                &reducer,
                "reduce",
                &context,
                Some(item),
                [result],
                at(index),
                Some(1),
            )
        })
}

fn reduce(context: NativeCallContext, array: &mut Array, reducer: FnPtr) -> Outcome<Dynamic> {
    reduce_with(context, array, reducer, Dynamic::UNIT)
}

fn reduce_rev_with(
    context: NativeCallContext,
    array: &mut Array,
    reducer: FnPtr,
    initial: Dynamic,
) -> Outcome<Dynamic> {
    let length = array.len();
    array
        .iter_mut()
        .rev()
        .enumerate()
        .try_fold(initial, |result, (index, item)| {
            call(
                &reducer,
                "reduce_rev",
                &context,
                Some(item),
                [result],
                at(length - 1 - index),
                Some(1),
            )
        })
}

fn reduce_rev(context: NativeCallContext, array: &mut Array, reducer: FnPtr) -> Outcome<Dynamic> {
    reduce_rev_with(context, array, reducer, Dynamic::UNIT)
}

fn zip(context: NativeCallContext, array: &mut Array, other: Array, map: FnPtr) -> Outcome<Array> {
    array
        .iter_mut()
        .zip(other)
        .enumerate()
        .map(|(index, (left, right))| {
            call(
                &map,
                "zip",
                &context,
                None,
                [left.clone(), right],
                at(index),
                None,
            )
        })
        .collect()
}

fn drain(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<Array> {
    let mut drained = Array::with_capacity(array.len());
    let (mut position, mut index) = (0, 0);
    while position < array.len() {
        let hit = call(
            &filter,
            "drain",
            &context,
            Some(&mut array[position]),
            [],
            at(index),
            Some(0),
        )?;
        if hit.as_bool().unwrap_or(false) {
            drained.push(array.remove(position));
        } else {
            position += 1;
        }
        index += 1;
    }
    Ok(drained)
}

fn retain(context: NativeCallContext, array: &mut Array, filter: FnPtr) -> Outcome<Array> {
    let mut drained = Array::new();
    let (mut position, mut index) = (0, 0);
    while position < array.len() {
        let hit = call(
            &filter,
            "retain",
            &context,
            Some(&mut array[position]),
            [],
            at(index),
            Some(0),
        )?;
        if hit.as_bool().unwrap_or(false) {
            position += 1;
        } else {
            drained.push(array.remove(position));
        }
        index += 1;
    }
    Ok(drained)
}

fn map_named(context: NativeCallContext, array: &mut Array, mapper: &str) -> Outcome<Array> {
    map(context, array, named(mapper)?)
}

fn filter_named(
    context: NativeCallContext,
    array: &mut Array,
    filter_name: &str,
) -> Outcome<Array> {
    filter(context, array, named(filter_name)?)
}

fn some_named(context: NativeCallContext, array: &mut Array, filter: &str) -> Outcome<bool> {
    some(context, array, named(filter)?)
}

fn all_named(context: NativeCallContext, array: &mut Array, filter: &str) -> Outcome<bool> {
    all(context, array, named(filter)?)
}

fn reduce_named(context: NativeCallContext, array: &mut Array, reducer: &str) -> Outcome<Dynamic> {
    reduce(context, array, named(reducer)?)
}

fn reduce_named_with(
    context: NativeCallContext,
    array: &mut Array,
    reducer: &str,
    initial: Dynamic,
) -> Outcome<Dynamic> {
    reduce_with(context, array, named(reducer)?, initial)
}

fn reduce_rev_named(
    context: NativeCallContext,
    array: &mut Array,
    reducer: &str,
) -> Outcome<Dynamic> {
    reduce_rev(context, array, named(reducer)?)
}

fn reduce_rev_named_with(
    context: NativeCallContext,
    array: &mut Array,
    reducer: &str,
    initial: Dynamic,
) -> Outcome<Dynamic> {
    reduce_rev_with(context, array, named(reducer)?, initial)
}

/// `index_of` with a string: a function name when the engine has such a
/// function for the elements, and a value to look for otherwise.
fn index_of_named_from(
    context: NativeCallContext,
    array: &mut Array,
    filter: &str,
    start: INT,
) -> Outcome<INT> {
    if array.is_empty() {
        return Ok(-1);
    }
    if let Ok(pointer) = named(filter) {
        match search(&context, array, &pointer, start) {
            Ok(found) => return Ok(found),
            Err(error) => match *error {
                EvalAltResult::ErrorFunctionNotFound(ref signature, ..)
                    if signature.starts_with(filter) => {}
                EvalAltResult::ErrorInFunctionCall(_, _, ref inner, _) => match **inner {
                    EvalAltResult::ErrorFunctionNotFound(ref signature, ..)
                        if signature.starts_with(filter) => {}
                    _ => return Err(error),
                },
                _ => return Err(error),
            },
        }
    }
    let value = Dynamic::from(filter.to_owned());
    let first = offset(array.len(), start);
    for (index, item) in array.iter_mut().enumerate().skip(first) {
        let same = context
            .call_native_fn_raw("==", true, &mut [item, &mut value.clone()])
            .or_else(|error| match *error {
                EvalAltResult::ErrorFunctionNotFound(ref signature, ..)
                    if signature.starts_with("==") =>
                {
                    // Elements of one type with no `==` are an error, as in Rhai.
                    if item.type_id() == value.type_id() {
                        Err(error)
                    } else {
                        Ok(Dynamic::FALSE)
                    }
                }
                _ => Err(error),
            })?;
        if same.as_bool().unwrap_or(false) {
            return Ok(INT::try_from(index).unwrap_or(INT::MAX));
        }
    }
    Ok(-1)
}

fn index_of_named(context: NativeCallContext, array: &mut Array, filter: &str) -> Outcome<INT> {
    index_of_named_from(context, array, filter, 0)
}

fn map_of_map(context: NativeCallContext, map: &mut Map, mapper: FnPtr) -> Outcome<Map> {
    let mut mapped = Map::new();
    for (key, item) in map.iter_mut() {
        // Rhai 1.27.0 names the caller `filter` here, and so do we.
        let value = call(
            &mapper,
            "filter",
            &context,
            Some(item),
            [key.clone().into()],
            [],
            Some(1),
        )?;
        mapped.insert(key.clone(), value);
    }
    Ok(mapped)
}

fn filter_of_map(context: NativeCallContext, map: &mut Map, filter: FnPtr) -> Outcome<Map> {
    let mut kept = Map::new();
    for (key, item) in map.iter_mut() {
        let keep = call(
            &filter,
            "filter",
            &context,
            Some(&mut *item),
            [key.clone().into()],
            [],
            Some(1),
        )?;
        if keep.as_bool().unwrap_or(false) {
            kept.insert(key.clone(), item.clone());
        }
    }
    Ok(kept)
}

/// `drain` and `retain` of a map. `drain` removes the entries that `filter` is
/// true for, and `retain` removes the others. Both return what they removed.
///
/// The first error stops the calls. The entry that failed is dropped, and every
/// later entry stays in `map`, as in Rhai.
fn divide(
    context: &NativeCallContext,
    map: &mut Map,
    filter: &FnPtr,
    caller: &str,
    retaining: bool,
) -> Outcome<Map> {
    let (mut drained, mut retained) = (Map::new(), Map::new());
    let mut failed = None;
    for (key, mut value) in mem::take(map) {
        if failed.is_some() {
            retained.insert(key, value);
            continue;
        }
        match call(
            filter,
            caller,
            context,
            Some(&mut value),
            [key.clone().into()],
            [],
            Some(1),
        ) {
            Ok(answer) if answer.as_bool().unwrap_or(false) != retaining => {
                drained.insert(key, value);
            }
            Ok(_) => {
                retained.insert(key, value);
            }
            Err(error) => failed = Some(error),
        }
    }
    *map = retained;
    failed.map_or(Ok(drained), Err)
}

fn drain_of_map(context: NativeCallContext, map: &mut Map, filter: FnPtr) -> Outcome<Map> {
    divide(&context, map, &filter, "drain", false)
}

fn retain_of_map(context: NativeCallContext, map: &mut Map, filter: FnPtr) -> Outcome<Map> {
    divide(&context, map, &filter, "retain", true)
}

#[cfg(test)]
mod tests {
    use super::super::callbacks;
    use super::*;

    /// Functions the scripts of these tests call by name.
    const PRELUDE: &str = r#"
        fn dbl(x) { x * 2 }
        fn big(x) { x > 1 }
        fn add(a, b) { a + b }
        fn same(a, b) { a == b }
        fn cmp(a, b) { a - b }
        fn up(n) { up(n + 1) }
        fn up2(a, b) { up2(a, b + 1) }
    "#;

    /// Rhai's own engine, and the engine of the worker's replacements, with the
    /// same small call depth so that a recursion breaches quickly.
    fn engines() -> (Engine, Engine) {
        let mut original = Engine::new();
        original.set_max_call_levels(16);
        let mut replaced = Engine::new();
        replaced.set_max_call_levels(16);
        callbacks::install(&mut replaced);
        (original, replaced)
    }

    fn run(engine: &Engine, script: &str) -> String {
        match engine.eval::<Dynamic>(&format!("{PRELUDE}\n{script}")) {
            Ok(value) => format!("ok {value:?}"),
            Err(error) => format!("err {error}"),
        }
    }

    #[test]
    fn each_replacement_does_what_the_original_does_while_nothing_fails() {
        // The scripts cover each replaced function, each way of calling the
        // pointer (arguments alone, with the index, bound to `this`, a name) and
        // the receiver's mutation. The original is the reference of each answer,
        // and an ordinary error must read the same, with the same message.
        let scripts = [
            "[1, 2, 3].map(|x| x * 2)",
            "[1, 2, 3].map(|x, i| x * 10 + i)",
            "[1, 2, 3].map(|| this * 3)",
            "[1, 2, 3].map(Fn(\"dbl\"))",
            "[1, 2, 3].map(\"dbl\")",
            "[].map(|x| x)",
            "[1, 2, 3, 4].filter(|x| x % 2 == 0)",
            "[1, 2, 3, 4].filter(|x, i| i > 1)",
            "[1, 2, 3, 4].filter(\"big\")",
            "let a = [1, 2, 3]; a.for_each(|| this *= 2); a",
            "let a = [1, 2, 3]; a.for_each(|i| this *= i); a",
            "[1, 5, 3].find(|x| x > 2)",
            "[1, 5, 3, 7].find(|x| x > 2, 2)",
            "[1, 5, 3, 7].find(|x| x > 2, -1)",
            "[1, 5, 3].find(|x| x > 9)",
            "[1, 5, 3].find_map(|x| if x > 2 { x * 100 })",
            "[1, 5, 3, 7].find_map(|x| if x > 2 { x * 100 }, 2)",
            "[1, 5, 3, 7].index_of(|x| x > 2)",
            "[1, 5, 3, 7].index_of(|x| x > 2, 2)",
            "[1, 5, 3, 7].index_of(|x| x > 2, -2)",
            "[1, 5, 3, 7].index_of(|x| x > 2, 99)",
            "[1, 5, 3, 7].index_of(\"big\")",
            "[1, 5, 3, 7].index_of(\"big\", 2)",
            "[\"a\", \"b\", \"c\"].index_of(\"b\")",
            "[\"a\", \"b\", \"c\"].index_of(\"b\", 2)",
            "[\"a\", \"b\", \"c\"].index_of(\"z\")",
            "[1, 2, 3].some(|x| x > 2)",
            "[1, 2, 3].some(\"big\")",
            "[].some(|x| true)",
            "[1, 2, 3].all(|x| x > 0)",
            "[1, 2, 3].all(\"big\")",
            "[].all(|x| false)",
            "[1, 2, 3, 4].reduce(|acc, x| acc + x)",
            "[1, 2, 3, 4].reduce(|acc, x| acc + x, 100)",
            "[1, 2, 3, 4].reduce(|acc, x, i| acc + x * i, 100)",
            "[1, 2, 3, 4].reduce(\"add\")",
            "[1, 2, 3, 4].reduce(\"add\", 100)",
            "[1, 2, 3, 4].reduce_rev(|acc, x| acc * 10 + x)",
            "[1, 2, 3, 4].reduce_rev(|acc, x, i| acc * 10 + x + i, 5)",
            "[1, 2, 3, 4].reduce_rev(\"add\")",
            "[1, 2, 3, 4].reduce_rev(\"add\", 100)",
            "[].reduce(|acc, x| acc + x, 7)",
            "[1, 2, 3].zip([10, 20, 30], |a, b| a + b)",
            "[1, 2, 3].zip([10, 20], |a, b, i| a + b + i)",
            "let a = [1, 2, 3, 4]; let r = a.drain(|x| x % 2 == 0); [a, r]",
            "let a = [1, 2, 3, 4]; let r = a.drain(|x, i| i < 2); [a, r]",
            "let a = [1, 2, 3, 4]; let r = a.retain(|x| x % 2 == 0); [a, r]",
            "let a = [1, 2, 3, 4]; let r = a.retain(|x, i| i < 2); [a, r]",
            "let a = [3, 1, 2]; a.sort(\"cmp\"); a",
            "let a = [1, 1, 2, 2, 3]; a.dedup(\"same\"); a",
            "let m = #{ a: 1, b: 2 }; m.map(|v| v * 3)",
            "let m = #{ a: 1, b: 2 }; m.map(|v, k| k + v)",
            "let m = #{ a: 1, b: 2 }; m.filter(|v| v > 1)",
            "let m = #{ a: 1, b: 2 }; m.filter(|v, k| k == \"a\")",
            "let m = #{ a: 1, b: 2, c: 3 }; let r = m.drain(|v| v > 1); [m, r]",
            "let m = #{ a: 1, b: 2, c: 3 }; let r = m.drain(|v, k| k == \"c\"); [m, r]",
            "let m = #{ a: 1, b: 2, c: 3 }; let r = m.retain(|v| v > 1); [m, r]",
            "let m = #{ a: 1, b: 2, c: 3 }; let r = m.retain(|v, k| k == \"c\"); [m, r]",
            // Ordinary errors read the same, caught and uncaught.
            "[1].map(|x| throw \"boom\")",
            "try { [1].map(|x| throw \"boom\") } catch (e) { e }",
            "try { [1].map(|x| 1 / 0) } catch (e) { e }",
            "try { [1].filter(Fn(\"nope\")) } catch (e) { e }",
            "try { [1, 2].map(|x, y, z| x) } catch (e) { e }",
            "try { [1].index_of(\"nope\", 0) } catch (e) { e }",
            "try { #{ a: 1 }.map(|v| throw 42) } catch (e) { e }",
            "let m = #{ a: 1, b: 2, c: 3 }; try { m.drain(|v| if v == 2 { throw 1 } else { true }) } catch (e) { }; m",
            "let a = [1, 2, 3, 4]; try { a.drain(|x| if x == 3 { throw 1 } else { true }) } catch (e) { }; a",
            // A constant is no receiver of a function that changes it.
            "const A = [3, 1, 2]; A.sort(|a, b| a - b)",
            "const A = [1, 2, 3]; A.drain(|x| x == 1)",
            "const A = [1, 2, 3]; A.for_each(|| this += 1)",
            "const A = [3, 1, 2]; A.sort(\"cmp\")",
            "const A = [1, 2]; A.map(|x| x + 1)",
            "const A = [1, 2]; A.reduce(|a, x| a + x)",
            "const A = [1, 2]; A.map(\"dbl\")",
        ];
        let (original, replaced) = engines();
        for script in scripts {
            assert_eq!(
                run(&replaced, script),
                run(&original, script),
                "script: {script}"
            );
        }
    }

    #[test]
    fn each_replacement_lets_a_breach_through_a_try_that_swallows_it_in_the_original() {
        // Every call below makes the pointer recurse past the call depth. Under
        // `try` the original catches the wrapped breach, which is the control
        // that selects. The replacement ends the script with the breach. A call
        // the list leaves out would be caught, so the list is the proof that no
        // replaced function is missing. `[1, 2]` has two elements for the
        // comparers and the reducers.
        let calls = [
            "[1].for_each(|x| up(0))",
            "[1].map(|x| up(0))",
            "[1].filter(|x| up(0))",
            "[1].find(|x| up(0))",
            "[1].find(|x| up(0), 0)",
            "[1].find_map(|x| up(0))",
            "[1].find_map(|x| up(0), 0)",
            "[1].index_of(|x| up(0))",
            "[1].index_of(|x| up(0), 0)",
            "[1].some(|x| up(0))",
            "[1].all(|x| up(0))",
            "[1, 2].reduce(|a, x| up(0))",
            "[1, 2].reduce(|a, x| up(0), 0)",
            "[1, 2].reduce_rev(|a, x| up(0))",
            "[1, 2].reduce_rev(|a, x| up(0), 0)",
            "[1, 2].zip([3, 4], |a, b| up(0))",
            "[1].drain(|x| up(0))",
            "[1].retain(|x| up(0))",
            "[1, 2].dedup(|a, b| up(0))",
            "[1, 2].sort(|a, b| up(0))",
            "[1, 2].sort_by(|a, b| up(0))",
            "[1].map(\"up\")",
            "[1].filter(\"up\")",
            "[1].index_of(\"up\")",
            "[1].index_of(\"up\", 0)",
            "[1].some(\"up\")",
            "[1].all(\"up\")",
            "[1, 2].reduce(\"up2\")",
            "[1, 2].reduce(\"up2\", 0)",
            "[1, 2].reduce_rev(\"up2\")",
            "[1, 2].reduce_rev(\"up2\", 0)",
            "[1, 2].dedup(\"up2\")",
            "[1, 2].sort(\"up2\")",
            "#{ a: 1 }.map(|v| up(0))",
            "#{ a: 1 }.filter(|v| up(0))",
            "#{ a: 1 }.drain(|v| up(0))",
            "#{ a: 1 }.retain(|v| up(0))",
        ];
        let (original, replaced) = engines();
        for call in calls {
            let script = format!(
                "let r = \"none\"; try {{ {call}; r = \"ended\"; }} catch (e) {{ r = \"caught\"; }} r"
            );
            let swallowed = run(&original, &script);
            let ended = run(&replaced, &script);
            // `dedup` and `sort` of the original drop the error with no `try`, so
            // they end the script in neither engine.
            let direct = call.contains("dedup") || call.contains("sort");
            if !direct {
                assert_eq!(swallowed, "ok \"caught\"", "the control: {call}");
            }
            assert!(
                ended.starts_with("err ") && ended.contains("Stack overflow"),
                "{call}: {ended}"
            );
        }
    }
}
