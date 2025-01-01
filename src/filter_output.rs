//! fiter_output filters and argumant with user inputs work in progress planing to filter can filter by time, file type etc.
use crate::file_reader::Element;
use crate::parserer::{Args, PType};
fn search_filter(parsed_args: &Args, file: &Element) -> bool {
    if parsed_args.search.is_empty(){
        true
    } else {
        file.name.rfind(&parsed_args.search).is_some()
    }
}

fn new_type_filter(parsed_args: &Args, file: &Element) -> bool {
    if let Some(file_type) = &file.file_type {
        if !parsed_args.filter.is_empty() && parsed_args.filter != "default" {
            parsed_args.filter == file_type.name || file_type.track.contains(&parsed_args.filter)
        } else {
            true
        }
    } else {
        true
    }
}

fn file_type_filter(parsed_args: &Args, file: &Element) -> bool {
    match parsed_args.p_type {
        PType::All => new_type_filter(parsed_args, file),
        PType::File => file.is_file,
        PType::Dir => file.is_dir,
    }
}

/// Takes parsed argumants end element vector filter files with argumant which user gives and return fitered element vector.
pub fn filter(parsed_args: &Args, file: &Element) -> bool {
    if parsed_args.all {
        file_type_filter(parsed_args, file)
    } else if parsed_args.hiden {
        if file.is_hiden {
            file_type_filter(parsed_args, file) && search_filter(parsed_args, file)
        } else {
            false
        }
    } else if !(file.is_hiden) {
        file_type_filter(parsed_args, file) && search_filter(parsed_args, file)
    } else {
        false
    }
}
