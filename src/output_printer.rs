//! Prints output with handsomely
mod formatter;
use formatter::{OutputElement, Output};
use crate::parserer::Args;
use crate::file_reader::Element;
use terminal_size::{Width, Height, terminal_size};
use crate::tomlread::ColorFormat;
/// Output print is a function that takes the file reader element and makes a tabular like output element and
/// creates a tabular structure called output push every element into output structure then prints the tabular structure.
pub fn output_print(parsed_args: &Args, 
					filtered_files: Vec<Element>){
	// dbg!(parsed_args.one_col);
	let mut width = 0;
	let size = terminal_size();
	if let Some((Width(w), Height(_h))) = size {
		width = w;
	} else {
    	eprintln!("Unable to get terminal size");
	}
	let mut output = Output::new(width as usize, parsed_args.one_col);
	for i in filtered_files{
		// println!("{}",&element_text);
		// dbg!(&i);
		let element = match i.file_type{
			Some(f) => {
				let element_text = format!("{} {}  ", f.symbol, i.name);//,i.file_type.clone().unwrap().symbol
				OutputElement::new(element_text, f.color)
			},
			None => OutputElement::new("Can't read".to_string(), ColorFormat::Red )
		};
		output = output.add(element);
	}	
	output.print_output();
}
