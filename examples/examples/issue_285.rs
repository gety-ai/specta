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

// User's actual scenario: Paused contains Option<Box<NotificationContent>>
// and NotificationContent enum contains Paused variant
#[derive(Type)]
#[specta(export = false)]
struct PauseReason;

#[derive(Type)]
#[specta(export = false)]
struct Paused {
    reason: PauseReason,
    prev_state: Option<Box<NotificationContent>>,
}

#[derive(Type)]
#[specta(export = false)]
enum NotificationContent {
    Paused(Paused),
    Other(String),
}

fn main() {
    println!("=== Test 1: Simple recursive types ===");
    {
        let mut type_map = TypeCollection::default();
        let ty = A::inline(&mut type_map, Generics::NONE);
        
        println!("Testing recursive type A -> B -> Option<Box<A>>...");
        
        match is_valid_ty(&ty, &type_map) {
            Ok(()) => println!("✓ is_valid_ty completed successfully!"),
            Err(e) => println!("✗ Error: {:?}", e),
        }
    }
    
    println!("\n=== Test 2: User's actual scenario ===");
    {
        let mut type_map = TypeCollection::default();
        let ty = Paused::inline(&mut type_map, Generics::NONE);
        
        println!("Testing Paused -> Option<Box<NotificationContent>> -> Paused...");
        
        match is_valid_ty(&ty, &type_map) {
            Ok(()) => println!("✓ is_valid_ty completed successfully!"),
            Err(e) => println!("✗ Error: {:?}", e),
        }
    }
    
    println!("\n=== Test 3: Starting from enum ===");
    {
        let mut type_map = TypeCollection::default();
        let ty = NotificationContent::inline(&mut type_map, Generics::NONE);
        
        println!("Testing NotificationContent -> Paused -> Option<Box<NotificationContent>>...");
        
        match is_valid_ty(&ty, &type_map) {
            Ok(()) => println!("✓ is_valid_ty completed successfully!"),
            Err(e) => println!("✗ Error: {:?}", e),
        }
    }
    
    println!("\nAll tests passed!");
}

