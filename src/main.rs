mod funcs;
mod structs;

use clap::{ArgAction, Parser};
use arboard::{Clipboard};
use std::path::PathBuf;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about = "Utility for unpacking data from clipboard")]
struct Args {
    ///(Optional) - Path to file (Ex. ~/Downloads/test.txt)
    path: Option<PathBuf>,
    ///(Optional) - Copy final path to clipboard
    #[arg(short, long, action = ArgAction::SetTrue)]
    copy_path: bool,
}

fn main() {
    let args = Args::parse();
    //let dst_path = &args.path;

    let dst_path: PathBuf = args.path.unwrap_or_else(| | {
        let current_dir = env::current_dir().unwrap_or_else(|e| {
            eprintln!("error: could not get current directory {}", e);
            std::process::exit(1);
        });

        let filename = funcs::generate_random_filename(12);

        current_dir.join(filename)
    });

    let mut clipboard = Clipboard::new().expect("error: coul not initialize clipboard!");

    let final_path: PathBuf = funcs::check_direct_data(&dst_path, &mut clipboard).unwrap();

    if args.copy_path {
        clipboard.set_text(final_path.to_string_lossy()).unwrap();
    }

    std::process::exit(0);
}