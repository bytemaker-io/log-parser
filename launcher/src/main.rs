

mod launcher_params;


use std::env::args;
use std::error::Error;
use std::thread::panicking;
use clap::ArgAction::Version;
use clap::builder::Str;
use clap::Parser;
use colored::Colorize;
use tracing::{error, info, Instrument, span};
use tracing_subscriber::fmt::format::{Compact, DefaultFields,Format};

use launcher_params::Args;
use loader::data_loder::data_loder;


#[tokio::main]
async fn main() {
    let x = tokio::task::spawn_blocking(|| {
        init()
    }).await.unwrap();
    let file_vec = x.unwrap();
}
fn init()-> Result<Vec<String>,()>{
    let params =  {
        // Initialize the tracing subscriber
        let subscriber = init_tracing();
        // Set the subscriber as the global default
        tracing::subscriber::set_global_default(subscriber).unwrap();
        let params = get_params();
        params
    };
    // Print the banner
    print_banner();
    //print the log file names and the path of the file
    info!("Checking the log file names and the path of the file...");

    info!("log file names: {:?}", params.file_names);
    info!("Path of the file: {:?}", &params.file_path.as_ref().unwrap());
    // Print the current directory
    let current_dir = std::env::current_dir().unwrap();
    info!("Current directory is: {:?}", current_dir);

    //file names is a vector,check each file is exist or not
    let mut file_vec:Vec<String>=vec!();
    for file_name in &params.file_names {
        let file = format!("{}/{}/{}", current_dir.display(), &params.file_path.as_ref().unwrap(), file_name);
        info!("File path is: {:?}", file);
        //check the file is exist or not
        if !std::path::Path::new(&file).exists() {
            error!("File {:?} does not exist", file);
            //throw an error to the caller

        }
        else {
            info!("File {:?} exist", file);
            file_vec.push(file);
        }
    }
    Ok(file_vec)
}
fn print_banner(){
    let banner= r#"              _
            /~_)                                                      /'
        ~-/'-~                                                      /'
        /'      ____     ____         ____     ,____     ____     /'          ____     O  ____
      /'      /'    )--/'    )      /'    )   /'    )  /'    )  /' /'    /  /'    )--/' /'    )--
 /~\,'   _  /'    /' /'    /'     /'    /'  /'    /' /'    /' /' /'    /'  '---,   /'  '---,
(,/'`\____)(___,/'  (___,/(__    (___,/(__/'    /(__(___,/(__(__(___,/(__(___,/   (__(___,/
                       /'                                          /'
               /     /'                                    /     /'
              (___,/'                                     (___,/'
                                    Author: Fei Wang
                                    Version: v1.0.0""#;
    println!("{}",banner.color("red"));
}
fn get_params() -> Args {
    Args::parse()
}

fn init_tracing() -> tracing_subscriber::fmt::Subscriber<DefaultFields, Format<Compact>> {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    subscriber
}
