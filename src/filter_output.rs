//! fiter_output filters and argumant with user inputs work in progress planing to filter can filter by time, file type etc.
use crate::file_reader::Element;
use crate::parserer::{Args, PType};


fn file_type_filter(parsed_args: &Args, file: Element, output: &mut Vec::<Element>) {
    match parsed_args.p_type {
        PType::All => {
            output.push(file);
        }
        PType::File => {
            if file.is_file {
                output.push(file);
            }
        }
        PType::Dir => {
            if file.is_dir {
                output.push(file);
            }
        }
    }
}

/// Takes parsed argumants end element vector filter files with argumant which user gives and return fitered element vector.
pub fn filter(parsed_args: &Args, files: Vec<Element>) -> Vec<Element> {
    let mut output: Vec<Element> = Vec::new();

    for file in files {
        if parsed_args.all {
            file_type_filter(parsed_args, file, &mut output);
        } else if parsed_args.hiden {
            if file.is_hiden {
                file_type_filter(parsed_args, file, &mut output);
            }
        } else if !(file.is_hiden) {
            file_type_filter(parsed_args, file, &mut output);
        }
    }
    output
}
