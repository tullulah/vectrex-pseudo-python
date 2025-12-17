/// Vector resource tests
/// Separated from vecres.rs to keep production code clean

use vectrex_lang::vecres::{VecResource, VecPath, Point};

#[test]
fn test_create_resource() {
    let res = VecResource::new("test-sprite");
    assert_eq!(res.name, "test-sprite");
    assert_eq!(res.layers.len(), 1);
}

#[test]
fn test_compile_to_asm() {
    let mut res = VecResource::new("ship");
    res.layers[0].paths.push(VecPath {
        name: "hull".to_string(),
        intensity: 127,
        closed: true,
        points: vec![
            Point { x: 0, y: 20, intensity: None },
            Point { x: -10, y: -10, intensity: None },
            Point { x: 10, y: -10, intensity: None },
        ],
    });
    
    let asm = res.compile_to_asm();
    assert!(asm.contains("_SHIP_VECTORS:"));  // Main label
    assert!(asm.contains("FCB 127"));          // intensity
    assert!(asm.contains("; Generated from")); // Header comment
}
