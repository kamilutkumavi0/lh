//! parserer module parses the user input with clap crate
use clap::{Parser, ValueEnum};

/// Simple and beautiful way to list diroctory
#[derive(Parser, Debug, Clone)]
#[command(name = "list_dir")]
#[command(author = "Kamil Utku Mavi, <kamilutkumavi0@gmail.com>")]
#[command(version)]
#[command(
    help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
#[command(name = "lists diroctory")]
pub struct Args {
    #[arg(default_value_t=String::from("./"))]
    pub path: String,
    /// Shows hiden files/folder.
    #[arg(long, short = 'H')]
    pub hiden: bool,
    /// Show all.
    #[arg(long, short = 'a')]
    pub all: bool,

    /// Selects printed type of output; only dir, only file or all.
    #[arg(long, short, value_enum,default_value_t = PType::All)]
    pub p_type: PType,

    #[arg(long, short, value_enum,default_value_t = SortType::Name)]
    pub sort: SortType,

    /// One collumn output for grep.
    #[arg(long, short = 'o')]
    pub one_col: bool,

    /// Long type explanation
    #[arg(long, short = 'l')]
    pub long: bool,

    /// Recursive listing
    #[arg(long, short, default_value_t = false)]
    pub recursive: bool,

    /// Filter of type of types of files
    #[arg(long, short, default_value_t = String::new())]
    pub filter: String,

    #[arg(long, short = 'S', default_value_t = String::new())]
    pub search: String,

    #[arg(long, short, default_value_t = false)]
    pub color_test: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PType {
    /// Prints  files and folders
    All,
    /// Prints only files
    File,
    /// Prints only directory
    Dir,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortType {
    /// Sort by name
    Name,
    /// Sort by Size
    Size,
}

pub fn pars_args() -> Args {
    Args::parse()
}
