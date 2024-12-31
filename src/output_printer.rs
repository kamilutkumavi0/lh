//! Prints output with handsomely
mod formatter;
use crate::file_reader::Element;
use crate::parserer::{Args, SortType};
use crate::tomlread::{ColorFormat, FontFormat};
use formatter::{Output, OutputElement};
use crate::filter_output::filter;
use terminal_size::{terminal_size, Height, Width};
/// Output print is a function that takes the file reader element and makes a tabular like output element and
/// creates a tabular structure called output push every element into output structure then prints the tabular structure.
pub fn output_printer(parsed_args: &Args, mut filtered_files: Vec<Element>) -> Output {
    // dbg!(parsed_args.one_col);
    match parsed_args.sort {
        SortType::Name => filtered_files.sort_by(|a, b| a.name.cmp(&b.name)),
        SortType::Size => filtered_files.sort_by(|a, b| a.size.cmp(&b.size)),
    }
    let mut width = 0;
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        width = w;
    } else {
        eprintln!("Unable to get terminal size");
    }
    let mut output = Output::new(width as usize, parsed_args.one_col, parsed_args.long);
    for i in filtered_files {
        if filter(parsed_args, &i) {
            let element = match i.file_type {
                Some(f) => {
                    if parsed_args.long {
                        let element_text = format!(
                            "{} {} {} {} {} {} {} ",
                            i.permisions,
                            i.user_name,
                            i.group_name,
                            i.size,
                            i.modified,
                            f.symbol,
                            i.name
                        ); //,i.file_type.clone().unwrap().symbol
                        OutputElement::new(element_text, f.color, f.font)
                    } else {
                        let element_text = format!("{} {}  ", f.symbol, i.name); //,i.file_type.clone().unwrap().symbol
                        OutputElement::new(element_text, f.color, f.font)
                    }
                }
                None => {
                    OutputElement::new("Can't read".to_string(), ColorFormat::Red, FontFormat::Bold)
                }
            };
        output = output.add(element);
        }
    }
    output
}

pub fn output_print(parsed_args: &Args, filtered_files: Vec<Element>) {
    output_printer(parsed_args, filtered_files).print_output();
}

pub fn output_print_recursive(parsed_args: &Args, filtered_files: Vec<Element>) {
    let out = output_printer(parsed_args, filtered_files.clone());
    if !out.is_empty() {
        println!("\n{}",&filtered_files[0].file_path);
        out.print_output();
    }
    for i in filtered_files {
        if i.is_dir {
            // println!("{:?}",&i);
            output_print_recursive(parsed_args, i.sub_dir.clone());
        }
    }
}
