

mod launcher_params;

use clap::Parser;
use tracing::{info, Subscriber};
use tracing_subscriber::fmt::format::{Compact, DefaultFields, Format};
use launcher_params::Args;


fn main() {
    tracing::subscriber::set_global_default(init_tracing()).unwrap();
    //打印当前目录路径
    let current_dir = std::env::current_dir().unwrap();
    info!("Current directory is: {:?}", current_dir);
    let args = Args::parse();
    println!("{:?}", args);
    println!("Hello, world!");
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
