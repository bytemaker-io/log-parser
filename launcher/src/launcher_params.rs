use std::cmp::Ord;
use clap::{arg, Parser};


#[derive(Parser, Debug)]
#[command(about = "This program is used to load the log file and parse it to frame the data. It will also upload the data to ElasticSearch.")]
#[command(author="Fei wang")]
#[command(version)]
pub struct Args {
    /// Name of the file,you can provide multiple file names, each file name should be separated by a space. Example: -f file1.log file2.log file3.log
    #[arg(short='f', long,required = true, num_args = 1..)]
    pub(crate) file_names: Vec<String>,
    /// Path of the file,if not provided, the file will be searched in the log_file directory
    #[arg[short='p', long,default_value = "log_file"]]
    pub(crate) file_path: Option<String>,
}

