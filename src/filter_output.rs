use crate::parserer::{Args, PType};
use crate::file_reader::Element;

pub fn filter(parsed_args: &Args, files: Vec<Element>) -> Vec<Element> {
	let mut output: Vec<Element> = Vec::new();
	for file in files{
		if parsed_args.all{
			match parsed_args.p_type {
				PType::All => {
					output.push(file);
				},
				PType::File => {
					if file.is_file{
						output.push(file);
					}
				},
				PType::Dir => {
					if file.is_dir{
						output.push(file);
					}
				},
			}
		} else if parsed_args.hiden{
			if file.is_hiden{
				match parsed_args.p_type {
					PType::All => {
						output.push(file);
					},
					PType::File => {			
						if file.is_file{
							output.push(file);
						}
					},
					PType::Dir => {					
						if file.is_dir{
							output.push(file);
						}
					},
				}
			}
		} else {
			if !(file.is_hiden){
				match parsed_args.p_type {
					PType::All => {
						output.push(file);
					},
					PType::File => {			
						if file.is_file{
							output.push(file);
						}
					},
					PType::Dir => {					
						if file.is_dir{
							output.push(file);
						}
					},
				}
			}
		}
	}
	output
}
