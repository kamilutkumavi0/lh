//! file_reader module is reads path and metadatas of diroctory.
//! and make every path as a Elemen struct for other elements can filter with it so user can see filtered output

use crate::parserer::Args;
use crate::tomlread::FileTypeToml;
use chrono::Datelike;
use chrono::Timelike;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::{self, DirEntry, ReadDir};
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub enum ReadError {
    MetadataError(String, String),
    NotExistingPath(String),
    ConfigError,
}

/// Element struct collect name of the dir as String, information about hiden, file, dir as bool and
/// file_type as a Option FileTypeToml which is going to configure bye lh.toml in the future.   
#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub file_path: String,
    pub is_hiden: bool,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_sym: bool,
    pub file_type: Option<FileTypeToml>,
    pub permisions: String,
    pub sub_dir: Vec<Element>,
    pub modified: String,
    pub user_name: String,
    pub group_name: String,
    pub size: u64,
}

impl Element {
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
        let is_hiden = matches!(&name.chars().nth(0).unwrap_or(' '), '.');
        let metadata_of_file_with_wrap = file.metadata();
        if metadata_of_file_with_wrap.is_err() {
            return Err(ReadError::MetadataError(file_path, name.to_string()));
        }
        let metadata_of_file = metadata_of_file_with_wrap.unwrap();
        // println!("{:b} {name}", &metadata_of_file.permissions().mode());
        let size = metadata_of_file.len();
        let permision_of_file = format!("{:b}", &metadata_of_file.permissions().mode());
        let permisions_vec: Vec<char> = permision_of_file.chars().collect();
        let mut permisions = String::new();
        for (count, item) in permisions_vec
            .iter()
            .take(permision_of_file.len())
            .skip(permision_of_file.len() - 9)
            .enumerate()
        {
            if *item == '1' && count % 3 == 0 {
                permisions.push('r');
            } else if *item == '1' && count % 3 == 1 {
                permisions.push('w');
            } else if *item == '1' && count % 3 == 2 {
                permisions.push('x');
            } else {
                permisions.push('-');
            }
        }
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
            let out = if conf_hash.contains_key(&name_string) {
                Some(conf_hash.get(&name_string).unwrap().clone())
            } else {
                Some(conf_hash.get("default").unwrap().clone())
            };
            out
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
            is_hiden,
            is_file,
            is_dir,
            is_sym,
            file_type,
            permisions,
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

/// Takes conf_hash for following the file type and returns vector of elements
pub fn get_files(
    conf_hash: HashMap<String, FileTypeToml>,
    parsed_args: Args,
) -> Result<Vec<Element>, ReadError> {
    let initial_path: String = String::from(&parsed_args.path);
    let a: Option<ReadDir> = match fs::read_dir(&initial_path) {
        Ok(f) => Some(f),
        Err(_) => {
            // eprintln!("{} Not a existing path", &initial_path);
            None
        }
    };
    let output: Result<Vec<Element>, ReadError> = match a {
        Some(f) => Element::from_read_dir(f, &initial_path, conf_hash.clone()),
        None => Ok(Vec::new()),
    };
    output
}

pub fn get_files_recursive(
    conf_hash: HashMap<String, FileTypeToml>,
    parsed_args: Args,
) -> Result<Vec<Element>, ReadError> {
    let initial_path: String = String::from(&parsed_args.path);
    let a: Option<ReadDir> = match fs::read_dir(&initial_path) {
        Ok(f) => Some(f),
        Err(_) => {
            // eprintln!("{} Not a existing path", &initial_path);
            None
        }
    };
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
    let a: Option<ReadDir> = match fs::read_dir(&initial_path) {
        Ok(f) => Some(f),
        Err(_) => {
            // eprintln!("{} Not a existing path", &initial_path);
            None
        }
    };
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
