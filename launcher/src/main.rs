mod launcher_params;

use clap::Parser;
use colored::Colorize;
use tokio::task::JoinSet;
use tracing::{error, info};
use tracing_subscriber::fmt::format::{Compact, DefaultFields, Format};

use launcher_params::Args;
use loader::data_loder::DataLoader;

#[tokio::main]
async fn main() {
    //load tokio
	let tokio = tokio::task::spawn_blocking(|| init()).await.unwrap();
    let file_vec = tokio.unwrap();
    info!("File vector: {:?}", file_vec);
    let mut set = JoinSet::new();
    for file in file_vec {
        set.spawn(async move {
            let data_loader = DataLoader::new(file).await;
            let _ = data_loader.read_file().await;
        });
    }
    //calculate the time elapsed in processing all files
    let start = std::time::Instant::now();
    while let Some(handle) = set.join_next().await {
        handle.unwrap();
    }
    let duration = start.elapsed();
    info!("Time elapsed in processing all files is: {:?}", duration);
    info!("All files have been processed successfully");
    info!("please check the output directory for the output files");
}
fn init() -> Result<Vec<String>, ()> {
    let params = {
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
    info!(
        "Path of the file: {:?}",
        &params.file_path.as_ref().unwrap()
    );
    // Print the current directory
    let current_dir = std::env::current_dir().unwrap();
    info!("Current directory is: {:?}", current_dir);

    //file names is a vector,check each file is exist or not
    let mut file_vec: Vec<String> = vec![];
    for file_name in &params.file_names {
        let file = format!("{}/{}", &params.file_path.as_ref().unwrap(), file_name);
        info!("File path is: {:?}", file);
        //check the file is exist or not
        if !std::path::Path::new(&file).exists() {
            error!("File {:?} does not exist", file);
            //throw an error to the caller
        } else {
            info!("File {:?} exist", file);
            file_vec.push(file);
        }
    }
    Ok(file_vec)
}
fn print_banner() {
    //load banner from banner.txt，from current directory
    let banner = std::fs::read_to_string("banner.txt").unwrap();
    println!("{}", banner.color("blue"))
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
