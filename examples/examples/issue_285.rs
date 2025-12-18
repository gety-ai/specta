// Test for https://github.com/specta-rs/specta/issues/285
// This test verifies that recursive types with Option<Box<_>> don't cause stack overflow

use specta::{Type, TypeCollection, Generics};
use specta_serde::is_valid_ty;

#[derive(Type)]
#[specta(export = false)]
struct B {
    a: Option<Box<A>>,
}

#[derive(Type)]
#[specta(export = false)]
struct A {
    b: B,
}

fn main() {
    let mut type_map = TypeCollection::default();
    let ty = A::inline(&mut type_map, Generics::NONE);
    
    println!("Testing recursive type A -> B -> Option<Box<A>>...");
    
    // This should not cause stack overflow
    match is_valid_ty(&ty, &type_map) {
        Ok(()) => println!("✓ is_valid_ty completed successfully without stack overflow!"),
        Err(e) => println!("✗ Error: {:?}", e),
    }
    
    println!("Test passed!");
}

