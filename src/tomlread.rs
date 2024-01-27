//! reads user configiration for prints outpu handsomely
use toml::from_str;
use std::collections::HashMap;
use serde_derive::Deserialize;
// use std::env;
use std::fs;
use home::home_dir;
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
	// Symbol of the file .
    pub symbol: String,
	// Formated color of the file type.
    pub color: ColorFormat,
	// Exention tracking for the file type.
    track: Vec<String>,
}

/// All config of user in progress
#[derive(Deserialize,Debug)]
pub struct Config{// General custom settings like wanna see logos or not
    // Configiration parser of the cli app.
	pub file_type: Vec<FileTypeToml>,
}

/// Tract conf to hash table for easy to use for filtering the output.
fn track_hash(config: &Config) -> HashMap<String, FileTypeToml> {
	// conf_hash is a hash table for detect file extentions.
	let mut conf_hash: HashMap<String, FileTypeToml> = HashMap::new(); 
	
	// All file type's exention as a key file format as value
	// in config parserer inserts in conf_hash.
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
	let home_diroctory = home_dir();
	let config: Option<String> = match home_diroctory {
		Some(dir) => {
			let mut new_dir = dir.as_os_str().to_str().unwrap().to_string();
			new_dir.push_str("/.config/lh.toml");
			match fs::read_to_string(new_dir) {
				Ok(f) => Some(f),
				Err(_) => None,
			}
		},
			None => None,
	};
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

		[[file_type]]
		name = "c"
		symbol = ""
		color = "Blue"
		track = ["*.c","*.h"]

		[[file_type]]
		name = "docker"
		symbol = ""
		color = "BrightBlue"
		track = ["Dockerfile"]

		[[file_type]]
		name = "go"
		symbol = "󰟓"
		color = "BrightBlue"
		track = ["*.go"]

		[[file_type]]
		name = "haskel"
		symbol = ""
		color = "Magenta"
		track = ["*.hs"]

		[[file_type]]
		name = "java"
		symbol = ""
		color = "Red"
		track = ["*java"]

		[[file_type]]
		name = "julia"
		symbol = ""
		color = "Green"
		track = ["*.jl"]

		[[file_type]]
		name = "kotlin"
		symbol = ""
		color = "Cyan"
		track = ["*.kt", "*.kts"]

		[[file_type]]
		name = "lua"
		symbol = ""
		color = "Blue"
		track = ["*.lua"]

		[[file_type]]
		name = "ocaml"
		symbol = ""
		color = "BrightRed"
		track = ["*.opam"]

		[[file_type]]
		name = "perl"
		symbol = ""
		color = "BrightBlue"
		track = ["*.pl"]

		[[file_type]]
		name = "php"
		symbol = ""
		color = "Blue"
		track = ["*.php"]

		[[file_type]]
		name = "ruby"
		symbol = ""
		color = "Red"
		track = ["*.rb"]
		
		[[file_type]]
		name = "r"
		symbol = ""
		color = "Blue"
		track = [".*R","*.Rd","*.Rmd", "*.Rproj", "*.Rxs"]

		[[file_type]]
		name = "swift"
		symbol = ""
		color = "BrightRed"
		track = ["*.swift"]

		[[file_type]]
		name = "zig"
		symbol = ""
		color = "Yellow"
		track = ["*.zig"]

		[[file_type]]
		name = "javascript"
		symbol = ""
		color = "Yellow"
		track = ["*.js"]

		[[file_type]]
		name = "html"
		symbol = ""
		color = "BrightCyan"
		track = ["*.html"]

		[[file_type]]
		name = "css"
		symbol = ""
		color = "BrightYellow"
		track = ["*.css"]

		[[file_type]]
		name = "C++"
		symbol = ""
		color = "Blue"
		track = ["*.cpp"]

		[[file_type]]
		name = "C#"
		symbol = "󰌛"
		color = "BrightYellow"
		track = ["*.cs"]
"#;
	let conf_str = match config{
		Some(t) => t,
		None => default.to_string(), 
	};
    let config = from_str(&conf_str).unwrap();
	track_hash(&config)
}
// General custom settings like wanna see logos or not
