//! reads user configiration for prints outpu handsomely
use toml::from_str;
use std::collections::HashMap;
use serde_derive::Deserialize;

///Selection of the color of output.
#[derive(Deserialize, Debug, Clone)]
pub enum ColorFormat{
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	BrightBlack,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
	BrightWhite,
}
/// Color and symbol of the file
#[derive(Deserialize, Debug, Clone)]
pub struct FileTypeToml{
    // name: String,
    pub symbol: String,
    pub color: ColorFormat,
    track: Vec<String>,
}
/// All config of user in progress
#[derive(Deserialize,Debug)]
pub struct Config{
    pub file_type: Vec<FileTypeToml>,
}
///tract conf to hash table for easy to use for filtering the output.
fn track_hash(config: &Config) -> HashMap<String, FileTypeToml> {
	let mut conf_hash: HashMap<String, FileTypeToml> = HashMap::new(); 
	for file_types in &config.file_type{
		for tracks in &file_types.track{
			//println!("{tracks}");
			conf_hash.insert(tracks.to_string(), file_types.clone());
		}
	}
	conf_hash
}
///read lh.toml or uses the defaut toml file.
pub fn toml_read()-> HashMap<String, FileTypeToml>{
	let default = 
r#"


		[[file_type]]
		name = "dir"
		symbol = ""
		color = "White"
		track = ["dir"]
		
		[[file_type]]
		name = "default"
		symbol = ""
		color = "White"
		track = ["default"]
				
		[[file_type]]
		name = "python"
		symbol = ""
		color = "Yellow"
		track = ["*.py", "*.pyc"]
		
		[[file_type]]
		name = "git folder"
		symbol = ""
		color = "BrightRed"
		track = [".git", ".gitignore"]
		
		[[file_type]]
		name = "rust"
		symbol = ""
		color = "BrightRed"
		track = ["*.rs"]
		
		[[file_type]]
		name = "toml"
		symbol = ""
		color = "Blue"
		track = ["*.toml"]

"#;
		
    let config = from_str(default).unwrap();
	track_hash(&config)
}
