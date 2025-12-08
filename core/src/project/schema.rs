//! VPy Project Schema
//!
//! Defines the structure of .vpyproj files (TOML format).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Root structure of a .vpyproj file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpyProject {
    /// Project metadata
    pub project: ProjectInfo,
    
    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,
    
    /// Source file patterns
    #[serde(default)]
    pub sources: SourcesConfig,
    
    /// Resource file patterns
    #[serde(default)]
    pub resources: ResourcesConfig,
    
    /// External dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, Dependency>,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Project name (used for output filename if not specified)
    pub name: String,
    
    /// Version string (semver recommended)
    #[serde(default = "default_version")]
    pub version: String,
    
    /// Author name/email
    #[serde(default)]
    pub author: Option<String>,
    
    /// Project description
    #[serde(default)]
    pub description: Option<String>,
    
    /// Entry point file (relative to project root)
    #[serde(default = "default_entry")]
    pub entry: String,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

fn default_entry() -> String {
    "src/main.vpy".to_string()
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Output binary path (relative to project root)
    #[serde(default = "default_output")]
    pub output: String,
    
    /// Target platform (currently only "vectrex")
    #[serde(default = "default_target")]
    pub target: String,
    
    /// Optimization level (0-3)
    #[serde(default = "default_optimization")]
    pub optimization: u8,
    
    /// Generate debug symbols (.pdb)
    #[serde(default = "default_true")]
    pub debug_symbols: bool,
    
    /// Additional assembler flags
    #[serde(default)]
    pub asm_flags: Vec<String>,
}

fn default_output() -> String {
    "build/game.bin".to_string()
}

fn default_target() -> String {
    "vectrex".to_string()
}

fn default_optimization() -> u8 {
    2
}

fn default_true() -> bool {
    true
}

/// Source file configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourcesConfig {
    /// VPy source file patterns (glob)
    #[serde(default = "default_vpy_sources")]
    pub vpy: Vec<String>,
    
    /// C source file patterns (glob) - for future C interop
    #[serde(default)]
    pub c: Vec<String>,
    
    /// Assembly source file patterns (glob)
    #[serde(default)]
    pub asm: Vec<String>,
}

fn default_vpy_sources() -> Vec<String> {
    vec!["src/**/*.vpy".to_string()]
}

/// Resource file configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesConfig {
    /// Vector sprite files (.vec)
    #[serde(default)]
    pub vectors: Vec<String>,
    
    /// Binary data files (.dat)
    #[serde(default)]
    pub data: Vec<String>,
    
    /// Sound/music files (future)
    #[serde(default)]
    pub sounds: Vec<String>,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Simple version string: "1.0.0"
    Version(String),
    
    /// Detailed dependency with path or version
    Detailed(DependencyDetails),
}

/// Detailed dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyDetails {
    /// Version requirement (semver)
    #[serde(default)]
    pub version: Option<String>,
    
    /// Local path to library
    #[serde(default)]
    pub path: Option<PathBuf>,
    
    /// Git repository URL (future)
    #[serde(default)]
    pub git: Option<String>,
    
    /// Optional: only include if feature is enabled
    #[serde(default)]
    pub optional: bool,
}

impl VpyProject {
    /// Create a new project with default values
    pub fn new(name: &str) -> Self {
        Self {
            project: ProjectInfo {
                name: name.to_string(),
                version: default_version(),
                author: None,
                description: None,
                entry: default_entry(),
            },
            build: BuildConfig::default(),
            sources: SourcesConfig::default(),
            resources: ResourcesConfig::default(),
            dependencies: HashMap::new(),
        }
    }
    
    /// Get the project name
    pub fn name(&self) -> &str {
        &self.project.name
    }
    
    /// Get the entry point path
    pub fn entry(&self) -> &str {
        &self.project.entry
    }
    
    /// Get the output binary path
    pub fn output(&self) -> &str {
        &self.build.output
    }
    
    /// Check if this is a valid project configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Project name is required and non-empty
        if self.project.name.trim().is_empty() {
            errors.push("project.name is required and cannot be empty".to_string());
        }
        
        // Entry point must end with .vpy
        if !self.project.entry.ends_with(".vpy") {
            errors.push(format!(
                "project.entry must be a .vpy file, got: {}",
                self.project.entry
            ));
        }
        
        // Optimization level must be 0-3
        if self.build.optimization > 3 {
            errors.push(format!(
                "build.optimization must be 0-3, got: {}",
                self.build.optimization
            ));
        }
        
        // Target must be "vectrex" (for now)
        if self.build.target != "vectrex" {
            errors.push(format!(
                "build.target must be 'vectrex', got: {}",
                self.build.target
            ));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output: default_output(),
            target: default_target(),
            optimization: default_optimization(),
            debug_symbols: true,
            asm_flags: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_project() {
        let project = VpyProject::new("test-game");
        assert_eq!(project.name(), "test-game");
        assert_eq!(project.entry(), "src/main.vpy");
        assert_eq!(project.output(), "build/game.bin");
    }
    
    #[test]
    fn test_validate_valid_project() {
        let project = VpyProject::new("my-game");
        assert!(project.validate().is_ok());
    }
    
    #[test]
    fn test_validate_empty_name() {
        let mut project = VpyProject::new("");
        project.project.name = "   ".to_string();
        assert!(project.validate().is_err());
    }
    
    #[test]
    fn test_validate_bad_entry() {
        let mut project = VpyProject::new("test");
        project.project.entry = "main.py".to_string();
        assert!(project.validate().is_err());
    }
    
    #[test]
    fn test_validate_bad_optimization() {
        let mut project = VpyProject::new("test");
        project.build.optimization = 5;
        assert!(project.validate().is_err());
    }
    
    #[test]
    fn test_parse_minimal_toml() {
        let toml = r#"
[project]
name = "my-game"
"#;
        let project: VpyProject = toml::from_str(toml).unwrap();
        assert_eq!(project.name(), "my-game");
        assert_eq!(project.entry(), "src/main.vpy");
    }
    
    #[test]
    fn test_parse_full_toml() {
        let toml = r#"
[project]
name = "space-wars"
version = "1.2.3"
author = "John Doe <john@example.com>"
description = "A space shooter for Vectrex"
entry = "src/game.vpy"

[build]
output = "dist/spacewars.bin"
optimization = 3
debug_symbols = false

[sources]
vpy = ["src/**/*.vpy", "lib/**/*.vpy"]
c = ["native/*.c"]
asm = ["asm/*.asm"]

[resources]
vectors = ["assets/sprites/*.vec"]
data = ["assets/data/*.dat"]

[dependencies]
vectrex-stdlib = "1.0"
my-lib = { path = "../my-library" }
"#;
        let project: VpyProject = toml::from_str(toml).unwrap();
        
        assert_eq!(project.name(), "space-wars");
        assert_eq!(project.project.version, "1.2.3");
        assert_eq!(project.entry(), "src/game.vpy");
        assert_eq!(project.output(), "dist/spacewars.bin");
        assert_eq!(project.build.optimization, 3);
        assert!(!project.build.debug_symbols);
        
        assert_eq!(project.sources.vpy.len(), 2);
        assert_eq!(project.sources.c.len(), 1);
        assert_eq!(project.resources.vectors.len(), 1);
        
        assert!(project.dependencies.contains_key("vectrex-stdlib"));
        assert!(project.dependencies.contains_key("my-lib"));
    }
    
    #[test]
    fn test_serialize_project() {
        let project = VpyProject::new("test-game");
        let toml = toml::to_string_pretty(&project).unwrap();
        
        assert!(toml.contains("[project]"));
        assert!(toml.contains("name = \"test-game\""));
    }
}
