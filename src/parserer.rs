//! parserer module parses the user input with clap crate
use clap::{Parser, ValueEnum};


/// Simple and beautiful way to list diroctory
#[derive(Parser, Debug)]
#[command(name = "list_dir")]
#[command(author = "Kamil Utku Mavi, <kamilutkumavi0@gmail.com>")]
#[command(version)]
#[command(
    help_template = "{author-with-newline} {about-section}Version: {version} \n {usage-heading} {usage} \n {all-args} {tab}"
)]
#[command(name = "lists diroctory")]
pub struct Args {
    /// Shows hiden files/folder.
    #[arg(long, short='H')]
    pub hiden: bool,
    /// Show all.
    #[arg(long, short='a')]
    pub all: bool,

    /// Selects printed type of output; only dir, only file or all.
    #[arg(long, short, value_enum,default_value_t=PType::All)]
    pub p_type: PType,

    /// One collumn output for grep.
    #[arg(long, short='o')]
    pub one_col: bool,

    #[arg(long, short='l')]
    pub long: bool,

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PType{
    ///Prints  files and folders
    All,
    ///Prints only files
    File,
    ///Prints only directory
    Dir,
}


pub fn pars_args() -> Args{
	Args::parse()
}
