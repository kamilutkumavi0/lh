//! file_reader module is reads path and metadatas of diroctory.
//! and make every path as a Elemen struct for other elements can filter with it so user can see filtered output

use std::fs::{self, ReadDir, DirEntry};
use std::collections::HashMap;
use crate::tomlread::FileTypeToml;
use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Utc};
use chrono::Datelike;
use chrono::Timelike;
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
	pub is_sym: bool,
	pub file_type: Option<FileTypeToml>,
	pub permisions: String,
	//created
	pub modified: String,
	//access
	pub user_name: String,
	pub group_name: String,
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
		let month_str = match modify_date.month(){
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
			_ => "Dec"
		};
		let modified = format!("{} {} {}:{}", month_str, modify_date.day(), modify_date.hour(), modify_date.minute());
		let uid = metadata_of_file.uid();
		let gid = metadata_of_file.gid();
		let binding = get_user_by_uid(uid).unwrap();
		let user_name_dec= binding.name().to_str();
		let binding = get_group_by_gid(gid).unwrap();
		let group_name_dec = binding.name().to_str();
		let user_name = String::from(user_name_dec.unwrap());
		let group_name = String::from(group_name_dec.unwrap());
		let is_file = metadata_of_file.is_file();
		let is_sym = metadata_of_file.is_symlink();
		let mut is_dir = metadata_of_file.is_dir();
		if !is_file && !is_dir && !is_sym{
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
		} else if is_sym {
			Some(conf_hash.get("sym").unwrap().clone())
		} else {
			None
		};
		let name = name.to_string();
		Self{ name, is_hiden, is_file, is_dir, is_sym, file_type, permisions, modified, user_name, group_name}
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
pub fn get_files(conf_hash: HashMap<String, FileTypeToml>, initial_path: String) -> Vec<Element> {
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

