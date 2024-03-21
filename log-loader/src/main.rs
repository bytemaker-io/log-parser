

mod launcher_params;

use clap::Parser;
use launcher_params::Args;


fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("Hello, world!");
}
