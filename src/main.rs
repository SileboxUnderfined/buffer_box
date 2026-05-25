use clap::{ArgAction, Parser};
use std::path::{Path,PathBuf};
use std::fs::{File};
use std::fs;
use std::io;
use std::env;
use std::io::{BufWriter};
use arboard::{Clipboard};
use rand::{distr::Alphanumeric, RngExt};

#[derive(Parser, Debug)]
#[command(author, version, about = "Utility for unpacking data from clipboard")]
struct Args {
    ///(Optional) - Path to file (Ex. ~/Downloads/test.txt)
    path: Option<PathBuf>,
    ///(Optional) - Copy final path to clipboard
    #[arg(short, long, action = ArgAction::SetTrue)]
    copy_path: bool,
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

fn save_image_to_path(path: &PathBuf, pixels: &Vec<u8>, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(pixels.as_slice())?; 

    Ok(())
}

fn check_direct_data(dst_path: &Path, clipboard: &mut Clipboard) -> Result<PathBuf,()> {
    match try_to_get_direct_data(clipboard) {
        ClipboardContent::Text(text) => {
            let src_path = Path::new(&text);
            let final_path = match dst_path.extension().is_none() {
                true => { dst_path.with_added_extension("txt") }
                false => { dst_path.to_path_buf() }
            };
            if src_path.exists() {
                match try_to_copy_file(src_path, &final_path) {
                    Ok(()) => {
                        println!("success: copied file to path {}", &final_path.display());
                        return Ok(dst_path.to_path_buf())
                    }
                    Err(err) => {
                        eprintln!("error: error while copying file {}", err);
                        return Err(())
                    }
                }
            } else {
                match try_to_write_string_to_file(&text, &final_path) {
                    Ok(()) => {
                        println!("success: written text to path {}", &final_path.display());
                        return Ok(final_path.to_path_buf())
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

            let final_path = match dst_path.extension().is_none() {
                true => { dst_path.with_added_extension("png") }
                false => { dst_path.to_path_buf() }
            };

            //let final_path_copy = final_path.clone(); // TODO: FIX
            //println!("{}",final_path.display());

            match save_image_to_path(
                &final_path, 
                &raw_bytes, 
                width, 
                height
            ) {
                Ok(()) => {
                    println!("success: written image binary to path {}", &final_path.display());
                    return Ok(final_path)
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
    };
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

    let final_path: PathBuf = check_direct_data(&dst_path, &mut clipboard).unwrap();

    if args.copy_path {
        clipboard.set_text(final_path.to_string_lossy()).unwrap();
    }

    std::process::exit(0);
}