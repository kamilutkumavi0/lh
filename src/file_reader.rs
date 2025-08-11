//! File reader module for reading directory paths and metadata.
//! 
//! This module converts file system entries into `Element` structs that can be
//! filtered and displayed according to user preferences. It handles cross-platform
//! differences in file permissions and ownership information.

use crate::parserer::Args;
use crate::tomlread::FileTypeToml;
use chrono::Datelike;
use chrono::Timelike;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[cfg(unix)]
use users::{get_group_by_gid, get_user_by_uid};

/// Errors that can occur when reading directory contents.
#[derive(Debug)]
pub enum ReadError {
    /// Error reading file metadata with file path and name
    MetadataError(String, String),
    /// The specified path does not exist
    NotExistingPath(String),
    /// Configuration file error
    ConfigError,
}

/// Represents a file system element (file, directory, or symbolic link).
/// 
/// This struct contains all the metadata needed for displaying and filtering
/// directory contents, including permissions, ownership, and file type information.
#[derive(Debug, Clone)]
pub struct Element {
    /// The name of the file or directory
    pub name: String,
    /// The full path to the file or directory
    pub file_path: String,
    /// Whether this is a hidden file (starts with '.')
    pub is_hidden: bool,
    /// Whether this is a regular file
    pub is_file: bool,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Whether this is a symbolic link
    pub is_sym: bool,
    /// Optional file type configuration for styling
    pub file_type: Option<FileTypeToml>,
    /// String representation of file permissions (e.g., "rwxr-xr-x")
    pub permissions: String,
    /// Subdirectories (used for recursive listing)
    pub sub_dir: Vec<Element>,
    /// Last modified date as formatted string
    pub modified: String,
    /// Owner username
    pub user_name: String,
    /// Group name
    pub group_name: String,
    /// File size in bytes
    pub size: u64,
}

impl Element {
    /// Creates new dir
    pub fn new(
        name: String,
        is_file: bool,
        is_dir: bool,
        is_sym: bool,
        file_type: Option<FileTypeToml>,
    ) -> Self {
        let file_path: String = "/home".to_string();
        let sub_dir: Vec<Element> = Vec::new();
        let permissions: String = "rwxrwxrwx".to_string();
        let modified: String = "Jan 1 00:00".to_string();
        let user_name: String = "Test".to_string();
        let group_name: String = "Test".to_string();
        let size: u64 = 1;
        let is_hidden: bool = false;
        Self {
            name,
            file_path,
            is_hidden,
            is_file,
            is_dir,
            is_sym,
            file_type,
            permissions,
            sub_dir,
            modified,
            user_name,
            group_name,
            size,
        }
    }
    /// Takes a Dir Entry and transform as a element struct
    fn from_dir_entry(
        file: DirEntry,
        initial_path: &str,
        conf_hash: &HashMap<String, FileTypeToml>,
    ) -> Result<Self, ReadError> {
        let path = file.path();
        let name = match &path.to_str() {
            Some(name) => &name[initial_path.len()..],
            None => "Can't read",
        };
        let file_path = String::from(initial_path);
        let is_hidden = matches!(&name.chars().nth(0).unwrap_or(' '), '.');
        let metadata_of_file_with_wrap = file.metadata();
        if metadata_of_file_with_wrap.is_err() {
            return Err(ReadError::MetadataError(file_path, name.to_string()));
        }
        let metadata_of_file = metadata_of_file_with_wrap.unwrap();
        // println!("{:b} {name}", &metadata_of_file.permissions().mode());
        let size = metadata_of_file.len();
        
        // Handle permissions in a cross-platform way
        #[cfg(unix)]
        let permissions = {
            let permission_of_file = format!("{:b}", &metadata_of_file.permissions().mode());
            let permissions_vec: Vec<char> = permission_of_file.chars().collect();
            let mut perms = String::new();
            for (count, item) in permissions_vec
                .iter()
                .take(permission_of_file.len())
                .skip(permission_of_file.len() - 9)
                .enumerate()
            {
                if *item == '1' && count % 3 == 0 {
                    perms.push('r');
                } else if *item == '1' && count % 3 == 1 {
                    perms.push('w');
                } else if *item == '1' && count % 3 == 2 {
                    perms.push('x');
                } else {
                    perms.push('-');
                }
            }
            perms
        };
        
        #[cfg(windows)]
        let permissions = {
            // On Windows, we can check read-only attribute
            let readonly = metadata_of_file.permissions().readonly();
            if readonly {
                "r--r--r--".to_string()
            } else {
                "rw-rw-rw-".to_string()
            }
        };
        
        #[cfg(not(any(unix, windows)))]
        let permissions = "rwxrwxrwx".to_string();
        let modify_date: DateTime<Utc> = metadata_of_file.modified().unwrap().into();
        // dbg!(ab.month()); month day hour:second
        let month_str = match modify_date.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Dec",
        };
        let modified = format!(
            "{} {} {}:{}",
            month_str,
            modify_date.day(),
            modify_date.hour(),
            modify_date.minute()
        );
        
        // Handle user/group information in a cross-platform way
        #[cfg(unix)]
        let (user_name, group_name) = {
            let uid = metadata_of_file.uid();
            let gid = metadata_of_file.gid();

            let user_name = match get_user_by_uid(uid) {
                Some(binding) => {
                    let user_name_dec = binding.name().to_str();
                    String::from(user_name_dec.unwrap())
                }
                None => String::from("--"),
            };

            let group_name = match get_group_by_gid(gid) {
                Some(binding) => {
                    let group_name_dec = binding.name().to_str();
                    String::from(group_name_dec.unwrap())
                }
                None => String::from("--"),
            };
            
            (user_name, group_name)
        };
        
        #[cfg(not(unix))]
        let (user_name, group_name) = {
            // On non-Unix systems, we can't easily get user/group names
            (String::from("user"), String::from("group"))
        };

        let is_file = metadata_of_file.is_file();
        let is_sym = metadata_of_file.is_symlink();
        let mut is_dir = metadata_of_file.is_dir();
        if !is_file && !is_dir && !is_sym {
            is_dir = !is_dir;
        }
        //created_date
        //modified_date
        //access_date
        let file_type = if conf_hash.contains_key(name) {
            Some(conf_hash.get(name).unwrap().clone())
        } else if is_file {
            // dbg!(&name.chars());
            let text_vec = name.chars().collect::<Vec<_>>();
            let name_string = text_vec[1..].iter().cloned().collect::<String>(); //&name.to_string()[1..].to_string();
            let num = name_string.find('.');
            let name_string: String = match num {
                Some(n) => {
                    let mut temp = String::from("*");
                    temp.push_str(&name_string[n..]);
                    temp
                }
                None => "default".to_string(),
            };
            if conf_hash.contains_key(&name_string) {
                Some(conf_hash.get(&name_string).unwrap().clone())
            } else {
                Some(conf_hash.get("default").unwrap().clone())
            }
        } else if is_dir {
            Some(conf_hash.get("dir").unwrap().clone())
        } else if is_sym {
            Some(conf_hash.get("sym").unwrap().clone())
        } else {
            None
        };
        let sub_dir: Vec<Element> = Vec::new();
        let name = name.to_string();
        Ok(Self {
            name,
            file_path,
            is_hidden,
            is_file,
            is_dir,
            is_sym,
            file_type,
            permissions,
            sub_dir,
            modified,
            user_name,
            group_name,
            size,
        })
    }
    /// Takes a ReadDir argumant and send the every DirEntry in from_read_dir function and collect every element in vector
    fn from_read_dir(
        files: ReadDir,
        initial_path: &str,
        conf_hash: HashMap<String, FileTypeToml>,
        // parsed_args: &Args,
    ) -> Result<Vec<Element>, ReadError> {
        let mut element_vec: Vec<Element> = Vec::new();
        for file in files.flatten() {
            let unfiltered = Self::from_dir_entry(file, initial_path, &conf_hash);
            if let Ok(file) = unfiltered {
                element_vec.push(file);
            };
        }
        Ok(element_vec)
    }
}

/// Creates a test vector of elements for color testing.
/// 
/// This function generates sample elements for each configured file type
/// to help users preview color schemes and styling options.
/// 
/// # Arguments
/// * `conf_hash` - Configuration map containing file type styling information
/// 
/// # Returns
/// A vector of test elements representing different file types
pub fn get_color_test(conf_hash: HashMap<String, FileTypeToml>) -> Vec<Element> {
    let mut output: Vec<Element> = Vec::new();
    for i in conf_hash {
        // new(name: String, is_file: bool, is_dir: bool, is_sym: bool, file_type: Option<FileTypeToml>)
        if i.0 == "sym" {
            output.push(Element::new(i.0, true, false, true, Some(i.1)));
        } else if i.0 == "dir" {
            output.push(Element::new(i.0, false, true, false, Some(i.1)));
        } else {
            let new_name = if i.0.starts_with('*') {
                format!("{}{}", i.1.name.clone(), &i.0[1..])
            } else {
                i.0
            };
            output.push(Element::new(new_name, true, false, false, Some(i.1)));
        }
    }
    output
}
/// Reads directory contents and returns them as a vector of Element structs.
/// 
/// This function reads the specified directory path and converts each entry
/// into an Element with metadata including permissions, ownership, and styling.
/// 
/// # Arguments
/// * `conf_hash` - Configuration map for file type styling
/// * `parsed_args` - Command line arguments containing the path and options
/// 
/// # Returns
/// * `Ok(Vec<Element>)` - Vector of directory elements on success
/// * `Err(ReadError)` - Error if directory cannot be read or accessed
/// 
/// # Examples
/// ```ignore
/// let config = toml_read();
/// let args = pars_args();
/// let files = get_files(config, args)?;
/// ```
pub fn get_files(
    conf_hash: HashMap<String, FileTypeToml>,
    parsed_args: Args,
) -> Result<Vec<Element>, ReadError> {
    let initial_path: String = String::from(&parsed_args.path);
    let a: Option<ReadDir> = fs::read_dir(&initial_path).ok();
    let output: Result<Vec<Element>, ReadError> = match a {
        Some(f) => Element::from_read_dir(f, &initial_path, conf_hash.clone()),
        None => Ok(Vec::new()),
    };
    output
}

/// Reads directory contents recursively and returns them as a vector of Element structs.
/// 
/// This function works like `get_files` but also recursively reads subdirectories,
/// populating the `sub_dir` field of directory elements with their contents.
/// 
/// # Arguments
/// * `conf_hash` - Configuration map for file type styling
/// * `parsed_args` - Command line arguments containing the path and options
/// 
/// # Returns
/// * `Ok(Vec<Element>)` - Vector of directory elements with subdirectories populated
/// * `Err(ReadError)` - Error if directory cannot be read or accessed
/// 
/// # Note
/// This function may be slow for large directory trees and could potentially
/// consume significant memory for deep hierarchies.
pub fn get_files_recursive(
    conf_hash: HashMap<String, FileTypeToml>,
    parsed_args: Args,
) -> Result<Vec<Element>, ReadError> {
    let initial_path: String = String::from(&parsed_args.path);
    let a: Option<ReadDir> = fs::read_dir(&initial_path).ok();
    let output: Result<Vec<Element>, ReadError> = match a {
        Some(f) => Element::from_read_dir(f, &initial_path, conf_hash.clone()),
        None => Ok(Vec::new()),
    };
    match output {
        Ok(mut o) => {
            for i in &mut o {
                if i.is_dir {
                    if let Ok(rec_elem) = get_recursive(
                        i.clone(),
                        initial_path.clone(),
                        conf_hash.clone(),
                        parsed_args.clone(),
                    ) {
                        *i = rec_elem;
                    }
                }
            }
            Ok(o)
        }
        Err(e) => Err(e),
    }
}

fn get_recursive(
    mut parent_elem: Element,
    old_path: String,
    conf_hash: HashMap<String, FileTypeToml>,
    parsed_args: Args,
) -> Result<Element, ReadError> {
    let initial_path = format!("{}{}/", &old_path, &parent_elem.name);
    let a: Option<ReadDir> = fs::read_dir(&initial_path).ok();
    let output: Result<Vec<Element>, ReadError> = match a {
        Some(f) => Element::from_read_dir(f, &initial_path, conf_hash.clone()),
        None => Ok(Vec::new()),
    };
    match output {
        Ok(o) => {
            for i in o {
                if i.is_dir {
                    if let Ok(rec_elem) = get_recursive(
                        i,
                        initial_path.clone(),
                        conf_hash.clone(),
                        parsed_args.clone(),
                    ) {
                        parent_elem.sub_dir.push(rec_elem);
                    }
                } else {
                    parent_elem.sub_dir.push(i);
                }
            }
            Ok(parent_elem)
        }
        Err(e) => Err(e),
    }
}
