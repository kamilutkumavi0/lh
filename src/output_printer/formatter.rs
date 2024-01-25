//! This module creates a tabular structure called Output can be printable tabular format.
use colored::Colorize;
use crate::tomlread::ColorFormat;

/// OutputElement is a structure that carries the easy access the important element for the format as a tabular lenght, color, text. 
#[derive(Clone, Debug)]
pub struct OutputElement{
    len: usize,
    text: String,
	color: ColorFormat,
}

impl OutputElement{
    /// Creates new OutputElement.
    pub fn new(text: String, color: ColorFormat) -> Self {
        let mut len = 0;
		for _i in text.chars(){
			len += 1;
		}
        Self {len,text, color}
    }
}
/// OutputCol is a structure that carries every element in one col and helps the output structure can formated in right order.
#[derive(Clone, Debug)]
struct OutputCol {
    element_max: usize,
    len_vec: Vec<usize>,
    element_vec: Vec<OutputElement>,
}

impl OutputCol{
    /// Creates new OutputCol
    fn new() -> Self{
        let element_max = 0;
        let len_vec = Vec::new();
        let element_vec = Vec::new();
        Self {element_max, len_vec, element_vec}
    }
    /// Checks the all OutputElements maximum in the col so white space can be easyly formated.
    fn check_max(len_vec: &Vec<usize>)->usize{
        let mut element_max = 0;
        for i in len_vec{
            if i > &element_max{
                element_max = *i
            }
        }
        element_max
    }
    /// Adds new OutputElement in OutputCol.
    fn add(self, output_element: OutputElement) -> Self{
        let mut element_vec = self.element_vec;
        let mut len_vec = self.len_vec;
        len_vec.push(output_element.clone().len);
        element_vec.push(output_element);
        let element_max = Self::check_max(&len_vec);
        Self { element_max, len_vec, element_vec}
    }
    /// Delets first OutputElement in OutputCol.
    fn del(self) -> (Self, OutputElement) {
        let mut element_vec = self.element_vec;
        let mut len_vec = self.len_vec;
        let out_element = element_vec[0].clone();
        element_vec.remove(0);
        len_vec.remove(0);
        let element_max = Self::check_max(&len_vec);
        (Self {element_max, len_vec, element_vec}, out_element)
    }
}

/// Output structer is creates a row and col matrix so can be formated and printed as a tabular.
#[derive(Clone, Debug)]
pub struct Output {
    term_size: usize,
    col_max_len_vec:Vec<usize>,
    output_col_vec: Vec<OutputCol>,
    row_size: usize,
}

impl Output {
    /// Creats new Output.
    pub fn new(term_size: usize)->Self{
        let col_max_len_vec:Vec<usize> = Vec::new();
        let output_col_vec: Vec<OutputCol> = Vec::new();
        let row_size = 0;
        Self {term_size, col_max_len_vec, output_col_vec, row_size }
    }
    /// Checks the Output element in the same row fits the widht of the terminal.
    fn is_fit(col_max_len: &Vec<usize>, term_size: usize) -> bool {
        let mut total = 0;
        for i in col_max_len{
            total += i;
        }
        if total >= term_size{
            return false
        }
        true
    }
    /// Creatse new row in first OutputCol and shifts every OutputElement one step back 
    /// if last OutputCol has no OutputElement than delets the last OutputCol.
    fn shift(mut output_col_vec: Vec<OutputCol>, col_count: usize) -> (Vec<OutputCol>,Vec<usize>){
        let mut element: OutputElement;
        let mut col_max_len_vec: Vec<usize> = Vec::new();
		if !output_col_vec.is_empty() {
        	for i in col_count + 1..output_col_vec.len(){
            	(output_col_vec[i], element) = output_col_vec[i].clone().del();
            	output_col_vec[i-1] = output_col_vec[i-1].clone().add(element);
        	}
		}
        for i in &output_col_vec{
            col_max_len_vec.push(i.element_max);
        }
        if output_col_vec[output_col_vec.len()-1].element_vec.is_empty(){
            output_col_vec.pop();
        }
        (output_col_vec, col_max_len_vec)
    }
    /// Adds new OutputElement in the Output structure. 
    pub fn add(self, element: OutputElement)->Self{
		// dbg!(&self);
        let mut output_col_vec = self.output_col_vec;
        let mut col_max_len_vec = self.col_max_len_vec;
        let mut row_size = self.row_size;
        if !output_col_vec.is_empty(){
            let a = output_col_vec.len()-1;
            if output_col_vec[a].element_vec.len() < row_size {
				output_col_vec[a] = output_col_vec[a].clone().add(element);
                col_max_len_vec[a] = output_col_vec[a].element_max;
            } else {
                let new_col = OutputCol::new();
                let new_col = new_col.add(element);
                output_col_vec.push(new_col.clone());
                col_max_len_vec.push(new_col.element_max);
            }
        } else {
            let new_col = OutputCol::new();
            let new_col = new_col.add(element);
            output_col_vec.push(new_col.clone());
            col_max_len_vec.push(new_col.element_max);
			row_size += 1;
        }
        while !Self::is_fit(&col_max_len_vec, self.term_size){
			for row_count in 0..col_max_len_vec.len(){			
	            (output_col_vec, col_max_len_vec) = Self::shift(output_col_vec, row_count);
			}
			// dbg!(&output_col_vec);
            row_size += 1;
            if output_col_vec.len() <= 1{
                break;
            }
        }
        Self {term_size: self.term_size, col_max_len_vec, output_col_vec, row_size}
    }
    /// Formats and prints the Output structure as a tabular in terminal.
    pub fn print_output(self){
        for i in 0..self.output_col_vec[0].element_vec.len(){
	        let mut loc: usize = 0;
            for j in 0..self.output_col_vec.len(){
                if i < self.output_col_vec[j].element_vec.len(){
                        let mut space = String::new();
						for _k in 0..loc{
                            space.push(' ');
                        }
						match &self.output_col_vec[j].element_vec[i].color{
							ColorFormat::Black => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.black());},
							ColorFormat::Red => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.red());},
							ColorFormat::Green => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.green());},
							ColorFormat::Yellow => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.yellow());},
							ColorFormat::Blue => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.blue());},
							ColorFormat::Magenta => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.magenta());},
							ColorFormat::Cyan => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.cyan());},
							ColorFormat::White => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.white());},
							ColorFormat::BrightBlack => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_black());},
							ColorFormat::BrightRed => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_red());},
							ColorFormat::BrightGreen => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_green());},
							ColorFormat::BrightYellow => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_yellow());},
							ColorFormat::BrightBlue => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_blue());},
							ColorFormat::BrightMagenta => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_magenta());},
							ColorFormat::BrightCyan => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_cyan());},
							ColorFormat::BrightWhite => {print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.bright_white());},
						}
                        // print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.black());
						// dbg!(&self.output_col_vec[j].len_vec[i]);
                        loc = self.output_col_vec[j].element_max - self.output_col_vec[j].len_vec[i];
                }
            }
            println!();
        }
    }
}

