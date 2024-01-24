//! Prints output with handsomely
mod formatter;
use formatter::{OutputElement, Output};
use crate::parserer::Args;
use crate::file_reader::Element;
use terminal_size::{Width, Height, terminal_size};
/// Output print is a function that takes the file reader element and makes a tabular like output element and
/// creates a tabular structure called output push every element into output structure then prints the tabular structure.
pub fn output_print(_parsed_args: &Args, 
					filtered_files: Vec<Element>){
	let mut width = 0;
	let size = terminal_size();
	if let Some((Width(w), Height(_h))) = size {
		width = w;
	} else {
    	eprintln!("Unable to get terminal size");
	}
	let mut output = Output::new(width as usize);
	for i in filtered_files{
		let element_text = format!("{} {}  ", i.file_type.clone().unwrap().symbol, i.name);//,i.file_type.clone().unwrap().symbol
		// println!("{}",&element_text);
		let element = OutputElement::new(element_text, i.file_type.unwrap().color);
		output = output.add(element);
	}	
	output.print_output();
}
