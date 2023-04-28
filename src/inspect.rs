//! inspect generates metadata reports on makefiles.

extern crate lazy_static;
extern crate serde;
extern crate serde_json;

use self::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path;

lazy_static::lazy_static! {
    /// LOWER_FILENAMES_TO_IMPLEMENTATIONS maps common filenames to make implementation flavors.
    pub static ref LOWER_FILENAMES_TO_IMPLEMENTATIONS: HashMap<String, String> = vec![
        ("bsdmakefile".to_string(), "bmake".to_string()),
        ("gnumakefile".to_string(), "gmake".to_string()),
        ("makefile".to_string(), "make".to_string()),
    ].into_iter().collect::<HashMap<String, String>>();

    /// LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS maps common file extensions to make implementation flavors.
    pub static ref LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS: HashMap<String, String> = vec![
        ("bsdmakefile".to_string(), "bmake".to_string()),
        ("gnumakefile".to_string(), "gmake".to_string()),
        ("makefile".to_string(), "make".to_string()),
        ("mk".to_string(), "make".to_string()),
    ].into_iter().collect::<HashMap<String, String>>();

    /// LOWER_FILENAMES_TO_PARENT_BUILD_SYSTEMS maps common filenames to build systems
    /// that may generate makefiles as intermediate build artifacts.
    pub static ref LOWER_FILENAMES_TO_PARENT_BUILD_SYSTEMS: HashMap<String, String> = vec![
        ("cmakelists.txt".to_string(), "cmake".to_string()),
        ("configure".to_string(), "autotools".to_string()),
        ("makefile.pl".to_string(), "perl".to_string()),
    ].into_iter().collect::<HashMap<String, String>>();
}

/// Metadata collects information about a file path
/// regarding its candidacy as a potential POSIX makefile.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
    /// path denotes some file path.
    pub path: String,

    /// is_makefile denotes whether the file path appears to be a makefile,
    /// or some other kind of file.
    pub is_makefile: bool,

    /// build_system denotes a common build system,
    /// such as (POSIX) "make", "bmake", "gmake",
    /// "autotools", "cmake", "perl", etc.
    pub build_system: String,

    /// is_machine_generated denotes whether the file is likely to have been
    /// written by an automated process, as a secondary artifact
    /// created by some higher level build system.
    pub is_machine_generated: bool,
}

impl Metadata {
    /// new constructs a Metadata point.
    pub fn new() -> Metadata {
        Metadata {
            path: String::new(),
            is_makefile: false,
            build_system: String::new(),
            is_machine_generated: false,
        }
    }
}

impl Default for Metadata {
    /// default generates a basic Metadata point.
    fn default() -> Self {
        Metadata::new()
    }
}

impl fmt::Display for Metadata {
    /// fmt renders a Metadata point for console use.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json: String = serde_json::to_string(&self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json)
    }
}

/// metadata summaries high level attributes of a file path,
/// such as whether the file path appears to represent a conventional makefile,
/// whether the makefile is likely to use extensions beyond pure POSIX,
/// and whether the makefile is likely to be machine generated.
///
/// May present false positives for nmake
/// and other makefiles unrelated to the POSIX family.
pub fn analyze(pth: &path::Path) -> Result<Metadata, String> {
    let pth_abs: path::PathBuf = pth
        .canonicalize()
        .map_err(|_| "error: unable to resolve file path".to_string())?;

    let mut metadata: Metadata = Metadata::new();
    metadata.path = pth_abs.display().to_string();

    let filename: String = pth_abs
        .file_name()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let file_extension: String = pth_abs
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !LOWER_FILENAMES_TO_IMPLEMENTATIONS.contains_key(&filename)
        && !LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS.contains_key(&file_extension)
    {
        return Ok(metadata);
    }

    metadata.is_makefile = true;

    if let Some(implementation) = LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS.get(&file_extension) {
        metadata.build_system = implementation.to_string();
    }

    if let Some(implementation) = LOWER_FILENAMES_TO_IMPLEMENTATIONS.get(&filename) {
        metadata.build_system = implementation.to_string();

        if let Some(parent_dir) = pth_abs.parent() {
            for sibling_entry_result in parent_dir
                .read_dir()
                .map_err(|_| "error: unable to read directory".to_string())?
            {
                let sibling_entry: fs::DirEntry = sibling_entry_result
                    .map_err(|_| "error: unable to read directory".to_string())?;
                let sibling_string: String = sibling_entry
                    .path()
                    .file_name()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if let Some(parent_build_system) =
                    LOWER_FILENAMES_TO_PARENT_BUILD_SYSTEMS.get(&sibling_string)
                {
                    metadata.is_machine_generated = true;
                    metadata.build_system = parent_build_system.to_string();
                }
            }
        }
    }

    Ok(metadata)
}
