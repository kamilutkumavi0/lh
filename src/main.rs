use lh::parserer::pars_args;
use lh::file_reader::get_files;
use lh::filter_output::filter;
use lh::output_printer::output_print;
use std::collections::HashMap;
use lh::tomlread::{toml_read, FileTypeToml};
fn main() {
    let conf_hash: HashMap<String, FileTypeToml> = toml_read();
    let parsed_args = pars_args();
    let files = get_files(conf_hash);
    let filtered_files = filter(&parsed_args, files);
    output_print(&parsed_args, filtered_files);
}
