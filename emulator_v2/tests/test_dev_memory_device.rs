// Test for DevMemoryDevice printf functionality
use vectrex_emulator_v2::core::*;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_dev_memory_device_printf_functionality() {
    let memory_bus_rc = Rc::new(RefCell::new(memory_bus::MemoryBus::new()));
    let dev_device = Rc::new(RefCell::new(dev_memory_device::DevMemoryDevice::new()));
    
    // Create a simple RAM device for the test string
    let ram_device = Rc::new(RefCell::new(ram::Ram::new()));
    
    // Connect RAM to memory bus for address range 0x8000-0x8FFF
    memory_bus_rc.borrow_mut().connect_device(
        ram_device.clone(), 
        (0x8000, 0x8FFF), 
        memory_bus::EnableSync::False
    );
    
    // Initialize the DevMemoryDevice with memory bus connection
    dev_memory_device::DevMemoryDevice::init_memory_bus(
        dev_device.clone(),
        &mut memory_bus_rc.borrow_mut(),
        Rc::downgrade(&memory_bus_rc)
    );
    
    // Set up a test string in RAM 
    let test_string = "Hello, Vectrex World!\0";
    let string_address = 0x8000u16;
    
    // Write the test string to RAM via memory bus
    for (i, &byte) in test_string.as_bytes().iter().enumerate() {
        memory_bus_rc.borrow_mut().write(string_address + i as u16, byte);
    }
    
    // Test printf operation sequence
    // First write: set first byte of format address
    dev_device.borrow_mut().write(dev_memory_device::DEV_PRINTF_FORMAT[0], 0x80);
    
    // Second write: set second byte and trigger printf
    dev_device.borrow_mut().write(dev_memory_device::DEV_PRINTF_FORMAT[1], 0x00);
    
    // The printf functionality should have been triggered and printed to stdout
    // Since we can't easily capture stdout in this test, we just verify the operation completed
    println!("✅ Printf functionality test completed successfully");
}

#[test]
fn test_dev_memory_device_printf_without_memory_bus() {
    let dev_device = dev_memory_device::DevMemoryDevice::new();
    let mut dev_device = dev_device;
    
    // Test printf operation without memory bus connection
    dev_device.write(dev_memory_device::DEV_PRINTF_FORMAT[0], 0x80);
    dev_device.write(dev_memory_device::DEV_PRINTF_FORMAT[1], 0x00);
    
    println!("✅ Printf without memory bus test completed successfully");
}

#[test]
fn test_dev_memory_device_printf_push_arg() {
    let mut dev_device = dev_memory_device::DevMemoryDevice::new();
    
    // Test printf push arg8 register
    dev_device.write(dev_memory_device::DEV_PRINTF_PUSH_ARG8, 0x42);
    
    // Test printf push arg16 registers
    dev_device.write(dev_memory_device::DEV_PRINTF_PUSH_ARG16[0], 0x12);
    dev_device.write(dev_memory_device::DEV_PRINTF_PUSH_ARG16[1], 0x34);
    
    println!("✅ Printf push arg registers test completed successfully");
}