// Section emission helpers for linker object file generation
// Used when CodegenOptions.emit_sections == true

use super::CodegenOptions;

/// Emit a section marker if emit_sections is enabled
/// 
/// # Arguments
/// * `out` - Output string buffer
/// * `opts` - Code generation options (contains emit_sections flag)
/// * `name` - Section name (e.g., ".text.main", ".rodata", ".bss")
/// * `flags` - Section flags:
///   - "a" = allocatable (loaded into memory)
///   - "w" = writable (RAM)
///   - "x" = executable (code)
/// * `section_type` - Section type:
///   - "@progbits" = initialized data (code or constants in ROM)
///   - "@nobits" = uninitialized data (BSS - no data in file)
/// 
/// # Example
/// ```
/// emit_section(out, opts, ".text.main", "ax", "@progbits");
/// // Emits: .section .text.main, "ax", @progbits
/// ```
pub fn emit_section(
    out: &mut String,
    opts: &CodegenOptions,
    name: &str,
    flags: &str,
    section_type: &str
) {
    if opts.emit_sections {
        out.push_str(&format!(".section {}, \"{}\", {}\n", name, flags, section_type));
    }
}

/// Emit header section marker
pub fn emit_header_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".text.header", "ax", "@progbits");
}

/// Emit main function section marker
pub fn emit_main_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".text.main", "ax", "@progbits");
}

/// Emit loop function section marker
pub fn emit_loop_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".text.loop", "ax", "@progbits");
}

/// Emit generic function section marker
pub fn emit_function_section(out: &mut String, opts: &CodegenOptions, func_name: &str) {
    let section_name = format!(".text.{}", func_name.to_lowercase());
    emit_section(out, opts, &section_name, "ax", "@progbits");
}

/// Emit helper functions section marker (builtins, wrappers, etc.)
pub fn emit_helpers_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".text.fixed", "ax", "@progbits");
}

/// Emit read-only data section marker (strings, const arrays)
pub fn emit_rodata_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".rodata", "a", "@progbits");
}

/// Emit BSS section marker (uninitialized variables)
pub fn emit_bss_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".bss", "aw", "@nobits");
}

/// Emit data section marker (initialized variables)
pub fn emit_data_section(out: &mut String, opts: &CodegenOptions) {
    emit_section(out, opts, ".data", "aw", "@progbits");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::CodegenOptions;
    
    fn test_opts(emit_sections: bool) -> CodegenOptions {
        CodegenOptions {
            title: "test".into(),
            auto_loop: false,
            diag_freeze: false,
            force_extended_jsr: false,
            _bank_size: 0,
            per_frame_silence: false,
            debug_init_draw: false,
            blink_intensity: false,
            exclude_ram_org: false,
            fast_wait: false,
            emit_sections,
            source_path: None,
            output_name: None,
            assets: vec![],
            const_values: std::collections::BTreeMap::new(),
            const_arrays: std::collections::BTreeMap::new(),
            const_string_arrays: std::collections::BTreeSet::new(),
            mutable_arrays: std::collections::BTreeSet::new(),
            structs: std::collections::HashMap::new(),
            type_context: std::collections::HashMap::new(),
            bank_config: None,
            buffer_requirements: None,
            function_bank_map: std::collections::HashMap::new(),
        }
    }
    
    #[test]
    fn test_emit_section_disabled() {
        let mut out = String::new();
        let opts = test_opts(false);
        
        emit_section(&mut out, &opts, ".text.main", "ax", "@progbits");
        assert_eq!(out, ""); // No output when disabled
    }
    
    #[test]
    fn test_emit_section_enabled() {
        let mut out = String::new();
        let opts = test_opts(true);
        
        emit_section(&mut out, &opts, ".text.main", "ax", "@progbits");
        assert_eq!(out, ".section .text.main, \"ax\", @progbits\n");
    }
    
    #[test]
    fn test_emit_main_section() {
        let mut out = String::new();
        let opts = test_opts(true);
        
        emit_main_section(&mut out, &opts);
        assert_eq!(out, ".section .text.main, \"ax\", @progbits\n");
    }
    
    #[test]
    fn test_emit_rodata_section() {
        let mut out = String::new();
        let opts = test_opts(true);
        
        emit_rodata_section(&mut out, &opts);
        assert_eq!(out, ".section .rodata, \"a\", @progbits\n");
    }
    
    #[test]
    fn test_emit_bss_section() {
        let mut out = String::new();
        let opts = test_opts(true);
        
        emit_bss_section(&mut out, &opts);
        assert_eq!(out, ".section .bss, \"aw\", @nobits\n");
    }
}
