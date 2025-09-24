// Tests for DelayedValueStore module
// Following 1:1 structure from Vectrexy

use vectrex_emulator_v2::core::delayed_value_store::DelayedValueStore;

#[test]
fn test_immediate_assignment() {
    // C++ Original behavior: if CyclesToUpdateValue == 0, value is assigned immediately
    let mut store = DelayedValueStore::<u8>::new();
    store.cycles_to_update_value = 0;
    
    store.assign(42);
    assert_eq!(*store.value(), 42);
}

#[test]
fn test_delayed_assignment() {
    // C++ Original behavior: value is assigned after specified cycles
    let mut store = DelayedValueStore::<u8>::with_delay(2);
    
    store.assign(42);
    assert_eq!(*store.value(), 0); // Still default value
    
    store.update(1);
    assert_eq!(*store.value(), 0); // Still waiting
    
    store.update(1);
    assert_eq!(*store.value(), 42); // Now updated
}

#[test]
fn test_update_cycles_assertion() {
    // C++ Original: assert(cycles == 1)
    let mut store = DelayedValueStore::<u8>::with_delay(1);
    
    // This should not panic
    store.update(1);
}

#[test]
#[should_panic]
fn test_update_cycles_assertion_fail() {
    // C++ Original: assert(cycles == 1) - should fail for cycles != 1
    let mut store = DelayedValueStore::<u8>::with_delay(1);
    
    // This should panic
    store.update(2);
}

#[test]
fn test_specialized_constructors() {
    let store_u8 = DelayedValueStore::new_u8(5);
    assert_eq!(store_u8.cycles_to_update_value, 5);
    assert_eq!(*store_u8.value(), 0);
    
    let store_u16 = DelayedValueStore::new_u16(3);
    assert_eq!(store_u16.cycles_to_update_value, 3);
    assert_eq!(*store_u16.value(), 0);
    
    let store_bool = DelayedValueStore::new_bool(1);
    assert_eq!(store_bool.cycles_to_update_value, 1);
    assert_eq!(*store_bool.value(), false);
}