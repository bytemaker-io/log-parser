

mod launcher_params;







use clap::Parser;
use colored::Colorize;
use tokio::task::JoinSet;
use tracing::{error, info};
use tracing_subscriber::fmt::format::{Compact, DefaultFields,Format};

use launcher_params::Args;
use loader::data_loder::data_loder;


#[tokio::main]
async fn main() {
    let x = tokio::task::spawn_blocking(|| {
        init()
    }).await.unwrap();
    let file_vec = x.unwrap();
    info!("File vector: {:?}", file_vec);
    let mut set = JoinSet::new();
    for file in file_vec {
        set.spawn(async move{
            let data_loader = data_loder::new(file).await;
            data_loader.read_file().await;
        }
        );
    }
    // let mut handle_vec = vec!();
    while let Some(handle) = set.join_next().await {
        handle.unwrap();
    }
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

#[cfg(test)]
mod tests {
    
    
    #[test]
    fn test_data_loader() {
        let log = "2014/Oct/24 19:16:44.305556 1709 EXECUTOR - TTCN Logger v2.2 options: TimeStampFormat:=DateTime; LogEntityName:=Yes; LogEventTypes:=Yes; SourceInfoFormat:=Single; *.FileMask:=ACTION | DEFAULTOP | ERROR | EXECUTOR | PARALLEL | TESTCASE | PORTEVENT | STATISTICS | TIMEROP | USER | VERDICTOP | WARNING; *.ConsoleMask:=TESTCASE | STATISTICS; LogFileSize:=0; LogFileNumber:=1; DiskFullAction:=Error";
        println!("{}", log.contains("TIMEROP"));
       let parts: Vec<&str> = log.split_whitespace().collect();
                let timestamp = parts[0..2].join(" ");
            println!("Timestamp: {}", timestamp);
                let _component_name = parts[2];

                let _event_type = parts[3];

                let message = parts[4..].join(" ");
                let message_parts: Vec<&str> = message.split(".").collect();
                let _peer_component = message_parts[0];
                let _Event_description = message_parts[1].split(":").nth(0).unwrap(); // 提取以 ":" 为分割的第一个元素，即事件描述
                let port_and_function= message_parts[1].split(":").nth(1).unwrap();
                let _port=port_and_function.split("(").next().unwrap(); // 提取以 " " 为分割的第一个元素，即端口号

    }

}