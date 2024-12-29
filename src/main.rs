use lh::file_reader::{get_files, get_files_recursive};
// use lh::filter_output::filter;
use lh::output_printer::{output_print, output_print_recursive};
use lh::parserer::pars_args;
use lh::tomlread::{toml_read, FileTypeToml};
use std::collections::HashMap;
// fn recursive(files: &mut Vec<Element>, conf_hash: HashMap<String, FileTypeToml>,path: String) {
//     for i in files{
//         if i.is_dir {
//             let new_path = format!("{}{}/",path, &i.name);
//             let new_files = get_files(conf_hash.clone(), new_path);

//         }
//     }
// }
fn main() {
    let conf_hash: HashMap<String, FileTypeToml> = toml_read();
    let parsed_args = pars_args();
    if parsed_args.recursive {
        let files = get_files_recursive(conf_hash.clone(), parsed_args.clone());
        // let filtered_files = filter(&parsed_args, files);
        output_print_recursive(&parsed_args, files);
    } else {
        let files = get_files(conf_hash.clone(), parsed_args.clone());
        // recursive(&mut files, conf_hash, parsed_args.path.clone());
        // let filtered_files = filter(&parsed_args, files);
        output_print(&parsed_args, files);
    }
}
