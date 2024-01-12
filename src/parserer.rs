use clap::{Parser, ValueEnum};


/// Simple and beautiful way to list diroctory
#[derive(Parser, Debug)]
#[command(name = "list_dir")]
#[command(author = "Kamil Utku Mavi <kamilutkumavi0@gmail.com>")]
#[command(version = "0.1")]
#[command(name = "lists diroctory")]
pub struct Args {
    ///Shows hiden files/folder
    #[arg(long, short='g')]
    pub hiden: bool,
    ///Show all
    #[arg(long, short='a')]
    pub all: bool,

    ///Selects printed type of output; only dir, only file or all
    #[arg(long, short, value_enum,default_value_t=PType::All)]
    pub p_type: PType,

    ///Select the print type of output
    #[arg(long, short, value_enum,default_value_t=OType::Dict)]
    pub o_type: OType,
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


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OType{
    /// Lists output verticaly
    List,
    /// Lists output horizonty
    Dict,
    /// List output with block
    Matrix,
}

pub fn pars_args() -> Args{
	Args::parse()
}
