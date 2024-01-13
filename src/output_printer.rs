use crate::parserer::Args;
use crate::file_reader::Element;
use colored::Colorize;
use crate::tomlread::ColorFormat;
pub fn output_print(_parsed_args: &Args, 
					filtered_files: Vec<Element>){
	for i in filtered_files{
		match i.file_type.clone().unwrap().color{
			ColorFormat::Black => {print!("{} {} ",i.file_type.unwrap().symbol.black(), i.name.black());},
			ColorFormat::Red => {print!("{} {} ",i.file_type.unwrap().symbol.red(), i.name.red());},
			ColorFormat::Green => {print!("{} {} ",i.file_type.unwrap().symbol.green(), i.name.green());},
			ColorFormat::Yellow => {print!("{} {} ",i.file_type.unwrap().symbol.yellow(), i.name.yellow());},
			ColorFormat::Blue => {print!("{} {} ",i.file_type.unwrap().symbol.blue(), i.name.blue());},
			ColorFormat::Magenta => {print!("{} {} ",i.file_type.unwrap().symbol.magenta(), i.name.magenta());},
			ColorFormat::Cyan => {print!("{} {} ",i.file_type.unwrap().symbol.cyan(), i.name.cyan());},
			ColorFormat::White => {print!("{} {} ",i.file_type.unwrap().symbol.white(), i.name.white());},
			ColorFormat::BrightBlack => {print!("{} {} ",i.file_type.unwrap().symbol.bright_black(), i.name.bright_black());},
			ColorFormat::BrightRed => {print!("{} {} ",i.file_type.unwrap().symbol.bright_red(), i.name.bright_red());},
			ColorFormat::BrightGreen => {print!("{} {} ",i.file_type.unwrap().symbol.bright_green(), i.name.bright_green());},
			ColorFormat::BrightYellow => {print!("{} {} ",i.file_type.unwrap().symbol.bright_yellow(), i.name.bright_yellow());},
			ColorFormat::BrightBlue => {print!("{} {} ",i.file_type.unwrap().symbol.bright_blue(), i.name.bright_blue());},
			ColorFormat::BrightMagenta => {print!("{} {} ",i.file_type.unwrap().symbol.bright_magenta(), i.name.bright_magenta());},
			ColorFormat::BrightCyan => {print!("{} {} ",i.file_type.unwrap().symbol.bright_cyan(), i.name.bright_cyan());},
			ColorFormat::BrightWhite => {print!("{} {} ",i.file_type.unwrap().symbol.bright_white(), i.name.bright_white());},
		}
	}
}
