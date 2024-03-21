use clap::{arg, Parser};


#[derive(Parser, Debug)]
#[command(about = "This program is used to load the log file and parse it to frame the data. It will also upload the data to ElasticSearch.")]
#[command(author="Fei wang")]
#[command(version)]
pub struct Args {
    /// Name of the file
    #[arg(short='f', long)]
    file_name: String,
    /// Path of the file,if not provided, the file will be searched in the log_file directory
    #[arg[short='p', long]]
    file_path: Option<String>,

}

