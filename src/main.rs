use inquire::{Text};
use clap::{Parser};
use std::path::{Path,PathBuf};
use std::fs;
use std::env;
use std::io;
use arboard::{Clipboard,ImageData};
use rand::{distr::Alphanumeric, Rng, RngExt};

#[derive(Parser, Debug)]
#[command(author, version, about = "Utility for unpacking data from clipboard")]
struct Args {
    #[arg(short, long, value_name = "FILEPATH")]
    path: Option<PathBuf>
}

struct MyImageData {
    width: usize,
    height: usize,
    bytes: Vec<u8>,
}

enum ClipboardContent {
    Text(String),
    Image(MyImageData),
    UnkownOrEmpty,
}

fn try_to_get_direct_data(clipboard: &mut Clipboard) -> ClipboardContent {
    match (clipboard.get_text(), clipboard.get_image()) {
        (Ok(text), _) => ClipboardContent::Text(text),
        (_, Ok(image)) => ClipboardContent::Image(MyImageData { width: image.width, height: image.height, bytes: image.bytes.into_owned() }),
        _ => ClipboardContent::UnkownOrEmpty
    }
}

fn try_to_copy_file(source_path: &Path, dst_path: &Path) -> io::Result<()> {
    fs::copy(source_path, dst_path)?;
    Ok(())
}

fn try_to_write_string_to_file(text: &String, dst_path: &Path) -> io::Result<()> {
    fs::write(dst_path, text)?;
    Ok(())
}

fn check_direct_data(dst_path: &Path, clipboard: &mut Clipboard) -> Result<(),()> {
    match try_to_get_direct_data(clipboard) {
        ClipboardContent::Text(text) => {
            let src_path = Path::new(&text);
            if src_path.exists() {
                match try_to_copy_file(src_path, dst_path) {
                    Ok(()) => {
                        println!("success: copied file to path {}", dst_path.display());
                    }
                    Err(err) => {
                        eprintln!("error: error while copying file {}", err);
                        return Err(())
                    }
                }
            } else {
                match try_to_write_string_to_file(&text, dst_path) {
                    Ok(()) => {
                        println!("success: written text to path {}", dst_path.display());
                    }
                    Err(err) => {
                        eprintln!("error: error while writing text to file {}", err);
                        return Err(())
                    }
                }
            }
        }
        ClipboardContent::Image(image) => {
            let width = image.width as u32;
            let height = image.height as u32;

            let raw_bytes = image.bytes.to_owned();

            let final_path = dst_path.with_extension("png");
            println!("{}",final_path.display());

            match image::save_buffer(
                final_path,
                &raw_bytes,
                width,
                height,
                image::ColorType::Rgba8
            ) {
                Ok(()) => {
                    println!("success: written image binary to path {}", dst_path.display());
                }
                Err(e) => {
                    eprintln!("error: error while writing image to file {}", e);
                    return Err(())
                }
            }
        }
        ClipboardContent::UnkownOrEmpty => {
            eprintln!("error: unkown or empty buffer");
            return Err(())
        }
    }
    Ok(())
}

fn generate_random_filename(length: usize) -> PathBuf {
    let random_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    let filename = format!("{}", random_string);
    PathBuf::from(filename)
}

fn main() {
    let args = Args::parse();
    //let dst_path = &args.path;

    let dst_path: PathBuf = args.path.unwrap_or_else(| | {
        let current_dir = env::current_dir().unwrap_or_else(|e| {
            eprintln!("error: could not get current directory {}", e);
            std::process::exit(1);
        });

        let filename = generate_random_filename(12);

        current_dir.join(filename)
    });

    let mut clipboard = Clipboard::new().expect("error: coul not initialize clipboard!");

    check_direct_data(&dst_path, &mut clipboard);
}