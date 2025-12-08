//! VPy Project Loader
//!
//! Load and save .vpyproj files from disk.

use std::fs;
use std::path::{Path, PathBuf};
use crate::project::VpyProject;

/// Error type for project operations
#[derive(Debug)]
pub enum ProjectError {
    /// File not found
    NotFound(PathBuf),
    /// IO error reading/writing file
    IoError(std::io::Error),
    /// TOML parsing error
    ParseError(String),
    /// Validation error
    ValidationError(Vec<String>),
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::NotFound(path) => write!(f, "Project file not found: {}", path.display()),
            ProjectError::IoError(e) => write!(f, "IO error: {}", e),
            ProjectError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProjectError::ValidationError(errors) => {
                write!(f, "Validation errors:\n")?;
                for err in errors {
                    write!(f, "  - {}\n", err)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ProjectError {}

impl From<std::io::Error> for ProjectError {
    fn from(e: std::io::Error) -> Self {
        ProjectError::IoError(e)
    }
}

impl From<toml::de::Error> for ProjectError {
    fn from(e: toml::de::Error) -> Self {
        ProjectError::ParseError(e.to_string())
    }
}

impl From<toml::ser::Error> for ProjectError {
    fn from(e: toml::ser::Error) -> Self {
        ProjectError::ParseError(e.to_string())
    }
}

/// Project file extension
pub const PROJECT_EXTENSION: &str = "vpyproj";

/// Project file name (default)
pub const DEFAULT_PROJECT_FILE: &str = "project.vpyproj";

/// Load a project from a .vpyproj file
pub fn load_project(path: &Path) -> Result<VpyProject, ProjectError> {
    if !path.exists() {
        return Err(ProjectError::NotFound(path.to_path_buf()));
    }
    
    let content = fs::read_to_string(path)?;
    let project: VpyProject = toml::from_str(&content)?;
    
    // Validate the loaded project
    if let Err(errors) = project.validate() {
        return Err(ProjectError::ValidationError(errors));
    }
    
    Ok(project)
}

/// Save a project to a .vpyproj file
pub fn save_project(project: &VpyProject, path: &Path) -> Result<(), ProjectError> {
    // Validate before saving
    if let Err(errors) = project.validate() {
        return Err(ProjectError::ValidationError(errors));
    }
    
    let content = toml::to_string_pretty(project)?;
    fs::write(path, content)?;
    
    Ok(())
}

/// Find a .vpyproj file in a directory or its parents
pub fn find_project_file(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    
    loop {
        // Look for any .vpyproj file in current directory
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == PROJECT_EXTENSION).unwrap_or(false) {
                    return Some(path);
                }
            }
        }
        
        // Move to parent directory
        if !current.pop() {
            break;
        }
    }
    
    None
}

/// Create a new project with the given name in the specified directory
pub fn create_project(name: &str, dir: &Path) -> Result<PathBuf, ProjectError> {
    // Create project structure
    let project_dir = dir.join(name);
    let src_dir = project_dir.join("src");
    let assets_dir = project_dir.join("assets");
    let build_dir = project_dir.join("build");
    
    // Create directories
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&assets_dir.join("sprites"))?;
    fs::create_dir_all(&assets_dir.join("data"))?;
    fs::create_dir_all(&build_dir)?;
    
    // Create project file
    let project = VpyProject::new(name);
    let project_file = project_dir.join(format!("{}.vpyproj", name));
    save_project(&project, &project_file)?;
    
    // Create main.vpy with template
    let main_content = format!(r#"# {name} - Main entry point
# VPy game for Vectrex

def setup():
    """Called once at startup"""
    pass

def loop():
    """Called every frame"""
    # Your game logic here
    pass
"#);
    fs::write(src_dir.join("main.vpy"), main_content)?;
    
    // Create .gitignore
    let gitignore = r#"# Build artifacts
/build/
*.bin
*.o
*.pdb

# IDE
.vscode/
*.swp
*~
"#;
    fs::write(project_dir.join(".gitignore"), gitignore)?;
    
    Ok(project_file)
}

/// Loaded project with resolved paths
#[derive(Debug, Clone)]
pub struct LoadedProject {
    /// The project configuration
    pub config: VpyProject,
    
    /// Path to the .vpyproj file
    pub project_file: PathBuf,
    
    /// Root directory of the project
    pub root_dir: PathBuf,
}

impl LoadedProject {
    /// Load a project from a .vpyproj file path
    pub fn load(project_file: &Path) -> Result<Self, ProjectError> {
        let config = load_project(project_file)?;
        let project_file = project_file.to_path_buf();
        let root_dir = project_file.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        
        Ok(Self {
            config,
            project_file,
            root_dir,
        })
    }
    
    /// Get absolute path to entry point
    pub fn entry_path(&self) -> PathBuf {
        self.root_dir.join(&self.config.project.entry)
    }
    
    /// Get absolute path to output binary
    pub fn output_path(&self) -> PathBuf {
        self.root_dir.join(&self.config.build.output)
    }
    
    /// Get absolute path to build directory
    pub fn build_dir(&self) -> PathBuf {
        self.output_path().parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| self.root_dir.join("build"))
    }
    
    /// Get project name
    pub fn name(&self) -> &str {
        self.config.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    
    #[test]
    fn test_create_and_load_project() {
        let temp = temp_dir().join("vpy_test_project");
        let _ = fs::remove_dir_all(&temp); // Clean up from previous runs
        fs::create_dir_all(&temp).unwrap();
        
        // Create project
        let project_file = create_project("test-game", &temp).unwrap();
        assert!(project_file.exists());
        
        // Load project
        let loaded = LoadedProject::load(&project_file).unwrap();
        assert_eq!(loaded.name(), "test-game");
        assert!(loaded.entry_path().ends_with("src/main.vpy"));
        
        // Check main.vpy was created
        assert!(loaded.entry_path().exists());
        
        // Clean up
        let _ = fs::remove_dir_all(&temp);
    }
    
    #[test]
    fn test_find_project_file() {
        let temp = temp_dir().join("vpy_find_test");
        let _ = fs::remove_dir_all(&temp);
        let nested = temp.join("src").join("deep");
        fs::create_dir_all(&nested).unwrap();
        
        // Create project file in root
        let project = VpyProject::new("find-test");
        let project_file = temp.join("find-test.vpyproj");
        save_project(&project, &project_file).unwrap();
        
        // Find from nested directory
        let found = find_project_file(&nested);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), project_file);
        
        // Clean up
        let _ = fs::remove_dir_all(&temp);
    }
}
