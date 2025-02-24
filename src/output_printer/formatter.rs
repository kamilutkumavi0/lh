//! This module creates a tabular structure called Output can be printable tabular format.
use crate::tomlread::{ColorFormat, FontFormat};
use colored::Colorize;

/// OutputElement is a structure that carries the easy access the important element for the format as a tabular lenght, color, text.
#[derive(Clone, Debug)]
pub struct OutputElement {
    len: usize,
    text: String,
    color: ColorFormat,
    bg_color: Option<ColorFormat>,
    font: FontFormat,
}

impl OutputElement {
    /// Creates new OutputElement.
    pub fn new(
        text: String,
        color: ColorFormat,
        bg_color: Option<ColorFormat>,
        font: FontFormat,
    ) -> Self {
        let mut len = 0;
        for _i in text.chars() {
            len += 1;
        }
        Self {
            len,
            text,
            color,
            bg_color,
            font,
        }
    }
}
/// OutputCol is a structure that carries every element in one col and helps the output structure can formated in right order.
#[derive(Clone, Debug)]
struct OutputCol {
    element_max: usize,
    len_vec: Vec<usize>,
    element_vec: Vec<OutputElement>,
}

impl OutputCol {
    /// Creates new OutputCol
    fn new() -> Self {
        let element_max = 0;
        let len_vec = Vec::new();
        let element_vec = Vec::new();
        Self {
            element_max,
            len_vec,
            element_vec,
        }
    }
    /// Checks the all OutputElements maximum in the col so white space can be easyly formated.
    fn check_max(len_vec: &Vec<usize>) -> usize {
        let mut element_max = 0;
        for i in len_vec {
            if i > &element_max {
                element_max = *i
            }
        }
        element_max
    }
    /// Adds new OutputElement in OutputCol.
    fn add(self, output_element: OutputElement) -> Self {
        let mut element_vec = self.element_vec;
        let mut len_vec = self.len_vec;
        len_vec.push(output_element.clone().len);
        element_vec.push(output_element);
        let element_max = Self::check_max(&len_vec);
        Self {
            element_max,
            len_vec,
            element_vec,
        }
    }
    /// Delets first OutputElement in OutputCol.
    fn del(self) -> (Self, OutputElement) {
        let mut element_vec = self.element_vec;
        let mut len_vec = self.len_vec;
        let out_element = element_vec[0].clone();
        element_vec.remove(0);
        len_vec.remove(0);
        let element_max = Self::check_max(&len_vec);
        (
            Self {
                element_max,
                len_vec,
                element_vec,
            },
            out_element,
        )
    }
}

/// Output structer is creates a row and col matrix so can be formated and printed as a tabular.
#[derive(Clone, Debug)]
pub struct Output {
    term_size: usize,
    col_max_len_vec: Vec<usize>,
    output_col_vec: Vec<OutputCol>,
    row_size: usize,
    one_col: bool,
    long: bool,
}

impl Output {
    /// Creats new Output.
    pub fn new(term_size: usize, one_col: bool, long: bool) -> Self {
        let col_max_len_vec: Vec<usize> = Vec::new();
        let output_col_vec: Vec<OutputCol> = Vec::new();
        let row_size = 0;
        Self {
            term_size,
            col_max_len_vec,
            output_col_vec,
            row_size,
            one_col,
            long,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.output_col_vec.is_empty()
    }
    /// Checks the Output element in the same row fits the widht of the terminal.
    fn is_fit(col_max_len: &Vec<usize>, term_size: usize) -> bool {
        let mut total = 0;
        for i in col_max_len {
            total += i;
        }
        if total >= term_size {
            return false;
        }
        true
    }
    /// Creatse new row in first OutputCol and shifts every OutputElement one step back
    /// if last OutputCol has no OutputElement than delets the last OutputCol.
    fn shift(mut output_col_vec: Vec<OutputCol>, col_count: usize) -> (Vec<OutputCol>, Vec<usize>) {
        let mut element: OutputElement;
        let mut col_max_len_vec: Vec<usize> = Vec::new();
        if !output_col_vec.is_empty() {
            for i in col_count + 1..output_col_vec.len() {
                (output_col_vec[i], element) = output_col_vec[i].clone().del();
                output_col_vec[i - 1] = output_col_vec[i - 1].clone().add(element);
            }
        }
        for i in &output_col_vec {
            col_max_len_vec.push(i.element_max);
        }
        if output_col_vec[output_col_vec.len() - 1]
            .element_vec
            .is_empty()
        {
            output_col_vec.pop();
        }
        (output_col_vec, col_max_len_vec)
    }
    /// Adds new OutputElement in the Output structure.
    pub fn add(self, element: OutputElement) -> Self {
        // dbg!(&self);
        let mut output_col_vec = self.output_col_vec;
        let mut col_max_len_vec = self.col_max_len_vec;
        let mut row_size = self.row_size;
        if self.one_col || self.long {
            if !output_col_vec.is_empty() {
                let a = output_col_vec.len() - 1;
                output_col_vec[a] = output_col_vec[a].clone().add(element);
                col_max_len_vec[a] = output_col_vec[a].element_max;
            } else {
                let new_col = OutputCol::new();
                let new_col = new_col.add(element);
                output_col_vec.push(new_col.clone());
                col_max_len_vec.push(new_col.element_max);
                row_size += 1;
            }
        } else if !output_col_vec.is_empty() {
            let a = output_col_vec.len() - 1;
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
        while !Self::is_fit(&col_max_len_vec, self.term_size) {
            for row_count in 0..col_max_len_vec.len() {
                (output_col_vec, col_max_len_vec) = Self::shift(output_col_vec, row_count);
            }
            // dbg!(&output_col_vec);
            row_size += 1;
            if output_col_vec.len() <= 1 {
                break;
            }
        }
        Self {
            term_size: self.term_size,
            col_max_len_vec,
            output_col_vec,
            row_size,
            one_col: self.one_col,
            long: self.long,
        }
    }
    /// Formats and prints the Output structure as a tabular in terminal.
    pub fn print_output(self) {
        if !self.output_col_vec.is_empty() {
            for i in 0..self.output_col_vec[0].element_vec.len() {
                let mut loc: usize = 0;
                for j in 0..self.output_col_vec.len() {
                    if i < self.output_col_vec[j].element_vec.len() {
                        let mut space = String::new();
                        for _k in 0..loc {
                            space.push(' ');
                        }
                        match &self.output_col_vec[j].element_vec[i].bg_color {
                            None => match &self.output_col_vec[j].element_vec[i].color {
                                ColorFormat::Black => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.black()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .black()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .black()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Red => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.red()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .red()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .red()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Green => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.green()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .green()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .green()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Yellow => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.yellow()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .yellow()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .yellow()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Blue => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.blue()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .blue()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .blue()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Magenta => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.magenta()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .magenta()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .magenta()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::Cyan => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.cyan()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .cyan()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .cyan()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::White => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i].text.white()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .white()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .white()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightBlack => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_black()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_black()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_black()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightRed => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_red()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_red()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_red()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightGreen => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_green()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_green()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_green()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightYellow => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_yellow()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_yellow()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_yellow()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightBlue => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_blue()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_blue()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_blue()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightMagenta => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_magenta()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_magenta()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_magenta()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightCyan => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_cyan()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_cyan()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_cyan()
                                                .italic()
                                        ),
                                    }
                                }
                                ColorFormat::BrightWhite => {
                                    match &self.output_col_vec[j].element_vec[i].font {
                                        FontFormat::Regular => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_white()
                                        ),
                                        FontFormat::Bold => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_white()
                                                .bold()
                                        ),
                                        FontFormat::Italic => print!(
                                            "{}{}",
                                            space,
                                            &self.output_col_vec[j].element_vec[i]
                                                .text
                                                .bright_white()
                                                .italic()
                                        ),
                                    }
                                }
                            },
                            Some(bg_color) => match bg_color {
                                ColorFormat::Black => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_black()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Red => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_red()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Green => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_green()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Yellow => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_yellow()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Blue => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_blue()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Magenta => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_magenta()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::Cyan => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_cyan()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::White => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_white()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightBlack => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_black()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_black()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightRed => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_red()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_red()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightGreen => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_green()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_green()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightYellow => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_yellow()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_yellow()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightBlue => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_blue()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_blue()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightMagenta => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_magenta()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_magenta()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightCyan => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_cyan()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_cyan()
                                                ),
                                            }
                                        }
                                    }
                                }
                                ColorFormat::BrightWhite => {
                                    match &self.output_col_vec[j].element_vec[i].color {
                                        ColorFormat::Black => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .black()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Red => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .red()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Green => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .green()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Yellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .yellow()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Blue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .blue()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Magenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .magenta()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::Cyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .cyan()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::White => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .white()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlack => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_black()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightRed => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_red()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightGreen => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_green()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightYellow => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_yellow()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightBlue => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_blue()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightMagenta => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_magenta()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightCyan => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_cyan()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                        ColorFormat::BrightWhite => {
                                            match &self.output_col_vec[j].element_vec[i].font {
                                                FontFormat::Regular => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Bold => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .bold()
                                                        .on_bright_white()
                                                ),
                                                FontFormat::Italic => print!(
                                                    "{}{}",
                                                    space,
                                                    &self.output_col_vec[j].element_vec[i]
                                                        .text
                                                        .bright_white()
                                                        .italic()
                                                        .on_bright_white()
                                                ),
                                            }
                                        }
                                    }
                                }
                            },
                        }
                        // print!("{}{}",space, &self.output_col_vec[j].element_vec[i].text.black());
                        // dbg!(&self.output_col_vec[j].len_vec[i]);
                        loc =
                            self.output_col_vec[j].element_max - self.output_col_vec[j].len_vec[i];
                    }
                }
                println!();
            }
        }
    }
}
