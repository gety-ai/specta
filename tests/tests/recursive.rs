use std::collections::HashMap;

use specta::Type;
use specta_serde::SerdeError;
use specta_typescript::ExportError;

use crate::ts::{assert_ts, assert_ts_export};

#[derive(Type)]
#[specta(export = false)]
pub struct Recursive {
    demo: Box<Recursive>,
}

// https://github.com/specta-rs/specta/issues/285
// Test for recursive types with Option<Box<_>> pattern that previously caused stack overflow
#[derive(Type)]
#[specta(export = false)]
pub struct RecursiveA {
    b: RecursiveB,
}

#[derive(Type)]
#[specta(export = false)]
pub struct RecursiveB {
    a: Option<Box<RecursiveA>>,
}

#[derive(Type)]
#[specta(transparent, export = false)]
pub struct RecursiveMapKeyTrick(RecursiveMapKey);

#[derive(Type)]
#[specta(export = false)]
pub struct RecursiveMapKey {
    demo: HashMap<RecursiveMapKeyTrick, String>,
}

#[derive(Type)]
#[specta(export = false)]
pub struct RecursiveMapValue {
    demo: HashMap<String, RecursiveMapValue>,
}

#[derive(Type)]
#[specta(export = false)]
pub struct RecursiveInline {
    #[specta(flatten)]
    demo: Box<RecursiveInline>,
}

#[derive(Type)]
#[specta(transparent, export = false)]
pub struct RecursiveTransparent(Box<RecursiveInline>);

#[derive(Type)]
#[specta(export = false)]
pub enum RecursiveInEnum {
    A {
        #[specta(flatten)]
        demo: Box<RecursiveInEnum>,
    },
}

#[test]
fn test_recursive_types() {
    assert_ts!(Recursive, "{ demo: Recursive }");
    assert_ts_export!(Recursive, "export type Recursive = { demo: Recursive }");

    // Just check it doesn't overflow while doing this check
    assert_ts!(error; RecursiveMapKey, ExportError::Serde(SerdeError::InvalidMapKey));
    assert_ts_export!(
        error;
        RecursiveMapKey,
        ExportError::Serde(SerdeError::InvalidMapKey)
    );

    assert_ts!(
        RecursiveMapValue,
        "{ demo: Partial<{ [key in string]: RecursiveMapValue }> }"
    );
    assert_ts_export!(
        RecursiveMapValue,
        "export type RecursiveMapValue = { demo: Partial<{ [key in string]: RecursiveMapValue }> }"
    );
}

// https://github.com/specta-rs/specta/issues/285
// Test for mutual recursive types with Option<Box<_>> that previously caused stack overflow
// The bug was in is_valid_ty_internal: Nullable case called is_valid_ty instead of is_valid_ty_internal,
// which reset the checked_references HashSet and caused infinite recursion.
#[test]
fn test_recursive_with_option_box() {
    // These should not cause stack overflow
    assert_ts!(RecursiveA, "{ b: RecursiveB }");
    assert_ts!(RecursiveB, "{ a: RecursiveA | null }");
    assert_ts_export!(RecursiveA, "export type RecursiveA = { b: RecursiveB }");
    assert_ts_export!(RecursiveB, "export type RecursiveB = { a: RecursiveA | null }");
}

#[test]
#[should_panic]
fn test_recursive_types_panic1() {
    assert_ts!(RecursiveTransparent, "");
}

#[test]
#[should_panic]
fn test_recursive_types_panic2() {
    assert_ts_export!(RecursiveTransparent, "");
}

#[test]
#[should_panic]
fn test_recursive_types_panic3() {
    assert_ts!(RecursiveInEnum, "");
}

#[test]
#[should_panic]
fn test_recursive_types_panic4() {
    assert_ts_export!(RecursiveInEnum, "");
}
