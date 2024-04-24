//! file_reader module is reads path and metadatas of diroctory.
//! and make every path as a Elemen struct for other elements can filter with it so user can see filtered output

use std::fs::{self, ReadDir, DirEntry};
use std::collections::HashMap;
use crate::tomlread::FileTypeToml;
use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Utc};
use chrono::Datelike;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};
/// Element struct collect name of the dir as String, information about hiden, file, dir as bool and
/// file_type as a Option FileTypeToml which is going to configure bye lh.toml in the future.   
#[derive(Debug)]
pub struct Element{
	pub name: String,
	pub is_hiden: bool,
	pub is_file: bool,
	pub is_dir: bool,
	// pub is_sym: bool,
	pub file_type: Option<FileTypeToml>,
	pub permisions: String,
	//created
	//modified
	//access
}

impl Element{
	/// Takes a Dir Entry and transform as a element struct
	fn from_dir_entry(file: DirEntry, initial_path: &str, conf_hash: &HashMap<String, FileTypeToml>) -> Self{
		let path = file.path();
		let name = match &path.to_str(){
			Some(name) => &name[initial_path.len()..],
			None => "Can't read",
		};
		let is_hiden = matches!(&name.chars().nth(0).unwrap(), '.');
		let metadata_of_file = file.metadata().unwrap();
		// println!("{:b} {name}", &metadata_of_file.permissions().mode());
		let permision_of_file = format!("{:b}",&metadata_of_file.permissions().mode());
		let permisions_vec: Vec<char> = permision_of_file.chars().collect();
		let mut permisions = String::new();
		let mut second_count = 0;
		for i in permision_of_file.len()-9..permision_of_file.len(){
			if permisions_vec[i] == '1' && second_count % 3 == 0 {
				permisions.push('r');
			} else if permisions_vec[i] == '1' && second_count % 3 == 1 {
				permisions.push('w');
			} else if permisions_vec[i] == '1' && second_count % 3 == 2 {
				permisions.push('x');
			} else {
				permisions.push('-');
			}
			second_count += 1;
		}
		let modify_date: DateTime<Utc> = metadata_of_file.modified().unwrap().into();
		// dbg!(ab.month()); month day hour:second
		let uid = metadata_of_file.uid();
		let gid = metadata_of_file.gid();
		// dbg!(get_user_by_uid(uid).unwrap().name());
		// dbg!(get_group_by_gid(gid).unwrap().name());
		let is_file = metadata_of_file.is_file();
		let mut is_dir = metadata_of_file.is_dir();
		if !is_file && !is_dir{
			is_dir = !is_dir;
		}
		//created_date
		//modified_date
		//access_date
		let file_type = if conf_hash.contains_key(name){
			Some(conf_hash.get(name).unwrap().clone())
		} else if is_file {
			// dbg!(&name.chars());
			let text_vec = name.chars().collect::<Vec<_>>();
			let name_string = text_vec[1..].iter().cloned().collect::<String>();//&name.to_string()[1..].to_string();
			let num = name_string.find('.');
			let name_string: String = match num {
				Some(n) => {let mut temp = String::from("*");
							temp.push_str(&name_string[n..]);
							temp
						   },
				None => "default".to_string(),
			};
			let out = if conf_hash.contains_key(&name_string){
				Some(conf_hash.get(&name_string).unwrap().clone())
			} else {
			Some(conf_hash.get("default").unwrap().clone())
			};
			out
		} else if is_dir {
			Some(conf_hash.get("dir").unwrap().clone())
		} else {
			None
		};
		let name = name.to_string();
		Self{ name, is_hiden, is_file, is_dir, file_type, permisions}
	}
	/// Takes a ReadDir argumant and send the every DirEntry in from_read_dir function and collect every element in vector
	fn from_read_dir(files: ReadDir, initial_path: &str, conf_hash: HashMap<String, FileTypeToml>) -> Vec<Element>{
		let mut element_vec: Vec<Element> = Vec::new();
		for file in files {
			match file {
				Ok(f) => {
							element_vec.push(Self::from_dir_entry(f, initial_path, &conf_hash));
						 },
				Err(_) => todo!(),
			};
		}
	element_vec
	}
}

/// Takes conf_hash for following the file type and returns vector of elements
pub fn get_files(conf_hash: HashMap<String, FileTypeToml>) -> Vec<Element> {
	let initial_path = "./".to_string();
	let a: Option<ReadDir> = match fs::read_dir(&initial_path){
		Ok(f) => Some(f),
		Err(_) => {
				eprintln!("Not existed path");
				None
			   },	
	};
	let output: Vec<Element> = match a {
		Some(f) => Element::from_read_dir(f, &initial_path, conf_hash),
		None => Vec::new(),
	};
	output
}

