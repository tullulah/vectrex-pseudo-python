//! VPy Library format and management.
//!
//! Libraries (`.vpylib`) are packages of reusable VPy code that can be
//! shared across projects.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

/// Library manifest file name
pub const LIBRARY_MANIFEST: &str = "library.vpylib";

/// Library metadata from the manifest file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryManifest {
    /// Library section
    pub library: LibraryInfo,
    /// Exports section (optional)
    #[serde(default)]
    pub exports: ExportsConfig,
    /// Dependencies (optional)
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}

/// Core library information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryInfo {
    /// Library name (should be kebab-case)
    pub name: String,
    /// Version string (semver recommended)
    pub version: String,
    /// Author name/email
    #[serde(default)]
    pub author: String,
    /// Description of the library
    #[serde(default)]
    pub description: String,
    /// License (e.g., "MIT", "GPL-3.0")
    #[serde(default)]
    pub license: String,
    /// Repository URL
    #[serde(default)]
    pub repository: String,
    /// Keywords for discovery
    #[serde(default)]
    pub keywords: Vec<String>,
}

/// Exports configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportsConfig {
    /// List of public modules (default: all)
    #[serde(default)]
    pub modules: Vec<String>,
    /// Re-exports from dependencies
    #[serde(default)]
    pub re_exports: Vec<String>,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    /// Simple version string
    Version(String),
    /// Detailed specification
    Detailed {
        version: Option<String>,
        path: Option<String>,
        git: Option<String>,
        branch: Option<String>,
    },
}

/// A loaded library
#[derive(Debug)]
pub struct Library {
    /// Path to the library root
    pub root: PathBuf,
    /// Parsed manifest
    pub manifest: LibraryManifest,
    /// Cached list of available modules
    pub modules: Vec<String>,
}

impl Library {
    /// Load a library from a directory containing library.vpylib
    pub fn load(path: &Path) -> Result<Self> {
        let manifest_path = if path.is_file() && path.file_name().map(|n| n == LIBRARY_MANIFEST).unwrap_or(false) {
            path.to_path_buf()
        } else {
            path.join(LIBRARY_MANIFEST)
        };
        
        if !manifest_path.exists() {
            bail!("Library manifest not found: {:?}", manifest_path);
        }
        
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: LibraryManifest = toml::from_str(&content)?;
        
        let root = manifest_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get library root"))?
            .to_path_buf();
        
        // Discover modules in src/
        let mut modules = Vec::new();
        let src_dir = root.join("src");
        if src_dir.exists() {
            for entry in std::fs::read_dir(&src_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map(|e| e == "vpy").unwrap_or(false) {
                    if let Some(stem) = path.file_stem() {
                        let module_name = stem.to_string_lossy().to_string();
                        // Skip __init__.vpy
                        if module_name != "__init__" {
                            modules.push(module_name);
                        }
                    }
                }
            }
        }
        
        Ok(Library {
            root,
            manifest,
            modules,
        })
    }
    
    /// Get the path to a module's source file
    pub fn module_path(&self, module_name: &str) -> Option<PathBuf> {
        let path = self.root.join("src").join(format!("{}.vpy", module_name));
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
    
    /// Check if a module is exported (public)
    pub fn is_module_exported(&self, module_name: &str) -> bool {
        if self.manifest.exports.modules.is_empty() {
            // If no explicit exports, all modules are public
            true
        } else {
            self.manifest.exports.modules.contains(&module_name.to_string())
        }
    }
    
    /// Get the library name
    pub fn name(&self) -> &str {
        &self.manifest.library.name
    }
    
    /// Get the library version
    pub fn version(&self) -> &str {
        &self.manifest.library.version
    }
}

/// Library registry for managing installed libraries
#[derive(Debug, Default)]
pub struct LibraryRegistry {
    /// Libraries by name
    libraries: HashMap<String, Library>,
    /// Search paths for libraries
    search_paths: Vec<PathBuf>,
}

impl LibraryRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a search path for libraries
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }
    
    /// Load a library from a path and register it
    pub fn load_library(&mut self, path: &Path) -> Result<()> {
        let lib = Library::load(path)?;
        let name = lib.name().to_string();
        self.libraries.insert(name, lib);
        Ok(())
    }
    
    /// Find a library by name
    pub fn find_library(&self, name: &str) -> Option<&Library> {
        // First check already loaded
        if let Some(lib) = self.libraries.get(name) {
            return Some(lib);
        }
        None
    }
    
    /// Search for and load a library by name from search paths
    pub fn resolve_library(&mut self, name: &str) -> Result<&Library> {
        if self.libraries.contains_key(name) {
            return Ok(self.libraries.get(name).unwrap());
        }
        
        // Search in registered paths
        for search_path in &self.search_paths.clone() {
            let lib_path = search_path.join(name);
            if lib_path.exists() && lib_path.join(LIBRARY_MANIFEST).exists() {
                self.load_library(&lib_path)?;
                return Ok(self.libraries.get(name).unwrap());
            }
        }
        
        bail!("Library '{}' not found", name)
    }
    
    /// List all loaded libraries
    pub fn list_libraries(&self) -> Vec<&Library> {
        self.libraries.values().collect()
    }
    
    /// Get the module path for a library module
    pub fn resolve_module(&self, library_name: &str, module_name: &str) -> Option<PathBuf> {
        self.libraries.get(library_name)
            .and_then(|lib| lib.module_path(module_name))
    }
}

/// Create a new library skeleton
pub fn create_library(name: &str, path: &Path) -> Result<PathBuf> {
    let lib_dir = path.join(name);
    std::fs::create_dir_all(&lib_dir)?;
    
    // Create manifest
    let manifest = LibraryManifest {
        library: LibraryInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            author: String::new(),
            description: format!("A VPy library: {}", name),
            license: "MIT".to_string(),
            repository: String::new(),
            keywords: vec!["vectrex".to_string(), "vpy".to_string()],
        },
        exports: ExportsConfig::default(),
        dependencies: HashMap::new(),
    };
    
    let manifest_content = toml::to_string_pretty(&manifest)?;
    std::fs::write(lib_dir.join(LIBRARY_MANIFEST), manifest_content)?;
    
    // Create src directory
    let src_dir = lib_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;
    
    // Create initial module
    let initial_module = format!(r#"# {name} - Main module
#
# This is the main module of your library.
# Export functions and constants for users.

export example_function

def example_function():
    """An example function from {name}."""
    return 42
"#, name = name);
    
    std::fs::write(src_dir.join("lib.vpy"), initial_module)?;
    
    // Create README
    let readme = format!(r#"# {}

{}

## Installation

Add to your project's `project.vpyproj`:

```toml
[dependencies]
{} = {{ path = "../path/to/{}" }}
```

## Usage

```python
from {} import example_function

def main():
    let x = example_function()
```

## License

{}
"#, name, manifest.library.description, name, name, name, manifest.library.license);
    
    std::fs::write(lib_dir.join("README.md"), readme)?;
    
    Ok(lib_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_library() {
        let temp = TempDir::new().unwrap();
        let result = create_library("test-lib", temp.path());
        assert!(result.is_ok());
        
        let lib_path = result.unwrap();
        assert!(lib_path.join(LIBRARY_MANIFEST).exists());
        assert!(lib_path.join("src").join("lib.vpy").exists());
        assert!(lib_path.join("README.md").exists());
    }
    
    #[test]
    fn test_load_library() {
        let temp = TempDir::new().unwrap();
        let lib_path = create_library("my-lib", temp.path()).unwrap();
        
        let lib = Library::load(&lib_path).unwrap();
        assert_eq!(lib.name(), "my-lib");
        assert_eq!(lib.version(), "0.1.0");
    }
}
