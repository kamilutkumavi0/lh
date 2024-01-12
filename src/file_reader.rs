use std::fs::{self, ReadDir, DirEntry};
use std::collections::HashMap;
use crate::tomlread::FileTypeToml;
#[derive(Debug)]
pub struct Element{
	pub name: String,
	pub is_hiden: bool,
	pub is_file: bool,
	pub is_dir: bool,
	pub file_type: Option<FileTypeToml>,
	//created
	//modified
	//access
}

impl Element{
	fn from_dir_entry(file: DirEntry, initial_path: &str, conf_hash: &HashMap<String, FileTypeToml>) -> Self{
		let path = file.path();
		let name = &path.to_str().unwrap()[initial_path.len()..];
		let is_hiden = match &name.chars().nth(0).unwrap(){
			'.' => true,
			_ => false,
		};
		let metadata_of_file = file.metadata().unwrap();
		let is_file = metadata_of_file.is_file();
		let is_dir = metadata_of_file.is_dir();
		//created_date
		//modified_date
		//access_date
		let file_type = if conf_hash.contains_key(name){
			Some(conf_hash.get(name).unwrap().clone())
		} else if is_file {
			let name_string = &name.to_string()[1..].to_string();
			let num = name_string.find(".");
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
		Self{ name, is_hiden, is_file, is_dir, file_type}
	}

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

