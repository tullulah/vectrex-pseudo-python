//! Module resolver for multi-file VPy projects.
//! 
//! Handles import resolution, module loading, and symbol resolution
//! across multiple source files in a project.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use anyhow::{bail, Result};
use crate::ast::{Module, ImportDecl, ImportSymbols};
use crate::lexer;
use crate::parser;

use crate::library::LibraryRegistry;

/// Resolved symbol from an imported module
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    /// Original name in the source module
    pub name: String,
    /// Alias if `import X as Y` was used
    pub alias: Option<String>,
    /// Path to the source module
    pub source_module: PathBuf,
}

/// A loaded and parsed module
#[derive(Debug)]
pub struct LoadedModule {
    /// Filesystem path to the module
    pub path: PathBuf,
    /// Parsed AST
    pub module: Module,
    /// Symbols exported by this module
    pub exports: HashSet<String>,
}

/// Module resolver with caching and library support
pub struct ModuleResolver {
    /// Root directory of the project (where src/ is)
    project_root: PathBuf,
    /// Cache of already loaded modules
    cache: HashMap<PathBuf, LoadedModule>,
    /// Set of modules currently being loaded (for cycle detection)
    loading: HashSet<PathBuf>,
    /// Library registry for resolving library imports
    libraries: LibraryRegistry,
}

impl ModuleResolver {
    /// Create a new resolver with the given project root
    pub fn new(project_root: PathBuf) -> Self {
        let mut libraries = LibraryRegistry::new();
        
        // Add default library search paths
        let deps_dir = project_root.join("dependencies");
        if deps_dir.exists() {
            libraries.add_search_path(deps_dir);
        }
        
        Self {
            project_root,
            cache: HashMap::new(),
            loading: HashSet::new(),
            libraries,
        }
    }
    
    /// Add a library search path
    pub fn add_library_path(&mut self, path: PathBuf) {
        self.libraries.add_search_path(path);
    }
    
    /// Load a library by path
    pub fn load_library(&mut self, path: &Path) -> Result<()> {
        self.libraries.load_library(path)
    }
    
    /// Resolve the path to a module given an import declaration
    pub fn resolve_module_path(
        &mut self,
        import: &ImportDecl,
        from_file: &Path,
    ) -> Result<PathBuf> {
        let module_path = &import.module_path;
        
        if import.is_relative {
            // Relative import (.module or ..parent.module)
            self.resolve_relative_import(module_path, import.relative_level, from_file)
        } else {
            // Absolute import (from project src/ or libraries)
            self.resolve_absolute_import(module_path)
        }
    }
    
    /// Resolve a relative import path
    fn resolve_relative_import(
        &self,
        module_path: &[String],
        relative_level: usize,
        from_file: &Path,
    ) -> Result<PathBuf> {
        // Start from the directory containing the current file
        let mut base = from_file.parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get parent directory of {:?}", from_file))?
            .to_path_buf();
        
        // Go up directories based on relative_level (.. means go up 2, etc.)
        // Level 1 = current dir (.), Level 2 = parent (..), etc.
        for _ in 1..relative_level {
            base = base.parent()
                .ok_or_else(|| anyhow::anyhow!("Cannot navigate up from {:?}", base))?
                .to_path_buf();
        }
        
        // Add the module path components
        for part in module_path {
            base = base.join(part);
        }
        
        // Try .vpy extension
        let with_ext = base.with_extension("vpy");
        if with_ext.exists() {
            return Ok(with_ext);
        }
        
        // Try as directory with __init__.vpy
        let init_file = base.join("__init__.vpy");
        if init_file.exists() {
            return Ok(init_file);
        }
        
        bail!("Cannot resolve relative import: {:?} from {:?}", module_path, from_file)
    }
    
    /// Resolve an absolute import path (from src/ or from libraries)
    fn resolve_absolute_import(&mut self, module_path: &[String]) -> Result<PathBuf> {
        // First try from project src/
        let mut path = self.project_root.join("src");
        
        for part in module_path {
            path = path.join(part);
        }
        
        // Try .vpy extension
        let with_ext = path.with_extension("vpy");
        if with_ext.exists() {
            return Ok(with_ext);
        }
        
        // Try as directory with __init__.vpy
        let init_file = path.join("__init__.vpy");
        if init_file.exists() {
            return Ok(init_file);
        }
        
        // If not found in src/, try to resolve from libraries
        // First component is the library name, rest is module path
        if !module_path.is_empty() {
            let library_name = &module_path[0];
            
            if let Ok(lib) = self.libraries.resolve_library(library_name) {
                if module_path.len() == 1 {
                    // Import the library's main module (lib.vpy)
                    if let Some(lib_path) = lib.module_path("lib") {
                        return Ok(lib_path);
                    }
                } else {
                    // Import a specific module from the library
                    let module_name = &module_path[1];
                    if let Some(mod_path) = lib.module_path(module_name) {
                        return Ok(mod_path);
                    }
                }
            }
        }
        
        bail!("Cannot resolve import: {:?} (looked in {:?} and libraries)", module_path, path)
    }
    
    /// Load and parse a module, with cycle detection and caching
    pub fn load_module(&mut self, path: &Path) -> Result<&LoadedModule> {
        let canonical = path.canonicalize()
            .unwrap_or_else(|_| path.to_path_buf());
        
        // Check if already cached
        if self.cache.contains_key(&canonical) {
            return Ok(self.cache.get(&canonical).unwrap());
        }
        
        // Check for import cycle
        if self.loading.contains(&canonical) {
            bail!("Import cycle detected: {:?}", canonical);
        }
        
        // Mark as loading
        self.loading.insert(canonical.clone());
        
        // Read and parse the file
        let source = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Cannot read {:?}: {}", path, e))?;
        
        let tokens = lexer::lex(&source)?;
        let module = parser::parse_with_filename(&tokens, &path.display().to_string())?;
        
        // Collect exports
        let mut exports = HashSet::new();
        for item in &module.items {
            match item {
                crate::ast::Item::Export(e) => {
                    for sym in &e.symbols {
                        exports.insert(sym.clone());
                    }
                }
                crate::ast::Item::Function(f) => {
                    // Functions are exported by default if no explicit exports
                    exports.insert(f.name.clone());
                }
                crate::ast::Item::Const { name, .. } => {
                    exports.insert(name.clone());
                }
                crate::ast::Item::GlobalLet { name, .. } => {
                    exports.insert(name.clone());
                }
                _ => {}
            }
        }
        
        // Remove from loading set
        self.loading.remove(&canonical);
        
        // Cache the result
        self.cache.insert(canonical.clone(), LoadedModule {
            path: canonical.clone(),
            module,
            exports,
        });
        
        Ok(self.cache.get(&canonical).unwrap())
    }
    
    /// Resolve all imports for a module
    pub fn resolve_imports(
        &mut self,
        module: &Module,
        source_path: &Path,
    ) -> Result<Vec<ResolvedSymbol>> {
        let mut resolved = Vec::new();
        
        for import in &module.imports {
            let module_path = self.resolve_module_path(import, source_path)?;
            let loaded = self.load_module(&module_path)?;
            
            match &import.symbols {
                ImportSymbols::All => {
                    // import * - get all exports
                    for name in &loaded.exports {
                        resolved.push(ResolvedSymbol {
                            name: name.clone(),
                            alias: None,
                            source_module: loaded.path.clone(),
                        });
                    }
                }
                ImportSymbols::Named(symbols) => {
                    // import specific symbols
                    for sym in symbols {
                        if !loaded.exports.contains(&sym.name) {
                            bail!(
                                "Symbol '{}' not found in module {:?}",
                                sym.name,
                                module_path
                            );
                        }
                        resolved.push(ResolvedSymbol {
                            name: sym.name.clone(),
                            alias: sym.alias.clone(),
                            source_module: loaded.path.clone(),
                        });
                    }
                }
                ImportSymbols::Module { alias } => {
                    // import entire module
                    // For now, we'll need to handle this differently
                    // as it means prefixing all symbols with module name
                    resolved.push(ResolvedSymbol {
                        name: module_path.file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_default(),
                        alias: alias.clone(),
                        source_module: loaded.path.clone(),
                    });
                }
            }
        }
        
        Ok(resolved)
    }
    
    /// Get all loaded modules (for unified AST generation)
    pub fn get_all_modules(&self) -> Vec<&LoadedModule> {
        self.cache.values().collect()
    }
    
    /// Load a project starting from the entry point
    pub fn load_project(&mut self, entry_path: &Path) -> Result<()> {
        // Load the entry module
        let _ = self.load_module(entry_path)?;
        
        // Process imports recursively
        let mut to_process: Vec<PathBuf> = vec![entry_path.to_path_buf()];
        let mut processed: HashSet<PathBuf> = HashSet::new();
        
        while let Some(current) = to_process.pop() {
            let canonical = current.canonicalize().unwrap_or(current.clone());
            
            if processed.contains(&canonical) {
                continue;
            }
            processed.insert(canonical.clone());
            
            // Get the module from cache
            let module = if let Some(m) = self.cache.get(&canonical) {
                m.module.clone()
            } else {
                continue;
            };
            
            // Resolve imports and add to processing queue
            for import in &module.imports {
                if let Ok(module_path) = self.resolve_module_path(import, &current) {
                    if !processed.contains(&module_path) {
                        let _ = self.load_module(&module_path);
                        to_process.push(module_path);
                    }
                }
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_resolver_creation() {
        let resolver = ModuleResolver::new(PathBuf::from("/tmp/test"));
        assert!(resolver.cache.is_empty());
    }
    
    #[test]
    fn test_absolute_import_resolution() {
        // Create a temp project structure
        let temp = TempDir::new().unwrap();
        let src = temp.path().join("src");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("utils.vpy"), "def clamp(x, lo, hi):\n    return x\n").unwrap();
        
        let resolver = ModuleResolver::new(temp.path().to_path_buf());
        let result = resolver.resolve_absolute_import(&["utils".to_string()]);
        
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("utils.vpy"));
    }
}
