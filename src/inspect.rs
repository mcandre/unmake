//! inspect generates metadata reports on makefiles.

extern crate lazy_static;
extern crate regex;
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
        (".gyp".to_string(), "gyp".to_string()),
        ("makefile.pl".to_string(), "perl".to_string()),
    ].into_iter().collect::<HashMap<String, String>>();

    /// INCLUDE_FILENAME_PATTERN matches common filenames for makefiles intended
    /// for inclusion into other makefiles.
    pub static ref INCLUDE_FILENAME_PATTERN: regex::Regex = regex::Regex::new(r"^(sys\.mk|(.*\.include\.mk))$").unwrap();
}

/// Metadata collects information about a file path
/// regarding its candidacy as a potential POSIX makefile.
///
/// Some of the information may be left at a default value,
/// when scanning detects that the file is less sutiable for
/// linting as a POSIX compliant makefile.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
    /// path denotes some file path.
    pub path: String,

    /// filename denotes the basename.
    pub filename: String,

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

    /// is_include_file denotes whether the makefile is detected as an include file.
    /// For example, "sys.mk" or "*.include.mk"
    pub is_include_file: bool,

    /// is_empty denotes whether the file contains any data or not.
    pub is_empty: bool,

    /// lines denotes the number of LF's in the file.
    pub lines: usize,

    /// has_final_eol denotes whether a final eol has been read from the file.
    pub has_final_eol: bool,
}

impl Metadata {
    /// new constructs a Metadata point.
    pub fn new() -> Metadata {
        Metadata {
            path: String::new(),
            filename: String::new(),
            is_makefile: false,
            build_system: String::new(),
            is_machine_generated: false,
            is_include_file: false,
            is_empty: true,
            lines: 0,
            has_final_eol: false,
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
/// May present some false positives for nnmake makefiles,
/// which are not in the POSIX make family.
///
/// May present some false negatives for makefile assets
/// when manually written makefiles are used
/// interchangeably with parent build systems.
///
/// Certain fields are left with default values,
/// when scanning detects files not suitable for POSIX make linting.
pub fn analyze(pth: &path::Path) -> Result<Metadata, String> {
    let pth_abs: path::PathBuf = pth
        .canonicalize()
        .map_err(|err| format!("error: {}: {}", pth.display(), err))?;

    let mut metadata: Metadata = Metadata::new();
    metadata.path = pth.display().to_string();

    let filename: String = pth
        .file_name()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    metadata.filename = filename;

    let filename_lower: String = metadata.filename.to_lowercase();

    let file_extension: String = pth
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    let file_extension_lower: String = file_extension.to_lowercase();

    if !LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS.contains_key(&file_extension_lower)
        && !LOWER_FILENAMES_TO_IMPLEMENTATIONS.contains_key(&filename_lower)
    {
        return Ok(metadata);
    }

    if let Some(implementation) =
        LOWER_FILE_EXTENSIONS_TO_IMPLEMENTATIONS.get(&file_extension_lower)
    {
        metadata.is_makefile = true;
        metadata.build_system = implementation.to_string();
    }

    if let Some(implementation) = LOWER_FILENAMES_TO_IMPLEMENTATIONS.get(&filename_lower) {
        metadata.is_makefile = true;
        metadata.build_system = implementation.to_string();
    }

    if !metadata.is_makefile || metadata.build_system != "make" {
        return Ok(metadata);
    }

    let parent_dir_option: Option<&path::Path> = pth_abs.parent();

    if parent_dir_option.is_none() {
        return Ok(metadata);
    }

    let parent_dir: &path::Path = parent_dir_option.unwrap();

    for sibling_entry_result in parent_dir
        .read_dir()
        .map_err(|err| format!("error: {}: {}", parent_dir.display(), err))?
    {
        let sibling_entry: fs::DirEntry = sibling_entry_result
            .map_err(|err| format!("error: {}: {}", parent_dir.display(), err))?;
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
            return Ok(metadata);
        }
    }

    let grandparent_dir_option: Option<&path::Path> = parent_dir.parent();

    if grandparent_dir_option.is_none() {
        return Ok(metadata);
    }

    let grandparent_dir: &path::Path = grandparent_dir_option.unwrap();

    for aunt_entry_result in grandparent_dir
        .read_dir()
        .map_err(|err| format!("error: {}: {}", grandparent_dir.display(), err))?
    {
        let aunt_entry: fs::DirEntry = aunt_entry_result
            .map_err(|err| format!("error: {}: {}", grandparent_dir.display(), err))?;
        let aunt_string: String = aunt_entry
            .path()
            .file_name()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if let Some(grandparent_build_system) =
            LOWER_FILENAMES_TO_PARENT_BUILD_SYSTEMS.get(&aunt_string)
        {
            metadata.is_machine_generated = true;
            metadata.build_system = grandparent_build_system.to_string();
            return Ok(metadata);
        }
    }

    metadata.is_include_file = INCLUDE_FILENAME_PATTERN.is_match(&metadata.filename);

    let byte_len: u64 = fs::metadata(&pth_abs)
        .map_err(|err| format!("error: {}: {}", pth_abs.display(), err))?
        .len();

    metadata.is_empty = byte_len == 0;

    if !metadata.is_empty {
        let makefile_str: &str = &fs::read_to_string(&pth_abs)
            .map_err(|err| format!("error: {}: {}", pth_abs.display(), err))?;
        metadata.lines = 1 + makefile_str.matches('\n').count();
        let last_char: char = makefile_str.chars().last().unwrap_or(' ');
        metadata.has_final_eol = last_char == '\n';
    }

    Ok(metadata)
}
