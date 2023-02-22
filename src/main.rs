/*
Copyright (c) 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use clap::Arg;
use std::io::Write;
use std::process::exit;

/// returns a character from a character set that corresponds to given brightness
fn get_char(charset: &[char], brightness: &u8) -> char {
    let charset_length = charset.len();
    return charset[((charset_length * *brightness as usize) as u32 / 256) as usize];
}

/// returns a complete character map where each character corresponds to brightness in an image point
fn charify(charset: &[char], brightness_map: &image::GrayImage) -> Vec<char> {
    let mut character_map: Vec<char> = Vec::with_capacity(brightness_map.len());
    for point in brightness_map.iter() {
        let character = get_char(charset, point);
        character_map.push(character);
    }

    return character_map;
}

fn main() {
    let mut charset: Vec<char> = vec![' ', '░', '▒', '▓', '█'];

    let matches = clap::Command::new("charify")
        .version("0.2")
        .author("Kasyanov Nikolay Alexeyevich (Unbewohnte)")
        .arg(
            Arg::new("image")
                .help("Path to an existing image")
                .takes_value(true)
                .required(true)
                .long("image")
                .short('i')
                .index(1),
        )
        .arg(
            Arg::new("destination")
                .help("Path to a newly created text file")
                .takes_value(true)
                .required(true)
                .long("destination")
                .short('d')
                .index(2),
        )
        .arg(
            Arg::new("new_dimensions")
                .help("Resize source image to specified dimensions")
                .takes_value(true)
                .required(false)
                .long("new_dimensions")
                .short('r'),
        )
        .arg(
            Arg::new("charset")
                .help("Set a new character set to use")
                .takes_value(true)
                .required(false)
                .long("charset")
                .short('c')
                .default_value(" ░▒▓█"),
        )
        .get_matches();

    // check source path
    let source_image_path: &std::path::Path;
    match matches.value_of("image") {
        Some(path) => {
            source_image_path = std::path::Path::new(path);
        }
        None => {
            eprintln!("[ERROR] source image is not specified !");
            exit(1);
        }
    }
    if !source_image_path.exists() {
        eprintln!("[ERROR] \"{}\" does not exist", source_image_path.display());
        exit(1);
    } else if !source_image_path.is_file() {
        eprintln!("[ERROR] \"{}\" is not a file", source_image_path.display());
    }

    let mut source_image: image::GrayImage;
    match image::open(source_image_path) {
        Ok(img) => {
            source_image = img.to_luma8();
        }
        Err(e) => {
            eprintln!("[ERROR] error opening a source image: {}", e);
            exit(1);
        }
    }

    // check destination path
    let destination_file_path: &std::path::Path;
    match matches.value_of("destination") {
        Some(path) => {
            destination_file_path = std::path::Path::new(path);
        }
        None => {
            eprintln!("[ERROR] destination path is not specified !");
            exit(1);
        }
    }
    match destination_file_path.parent() {
        Some(path) => match std::fs::create_dir_all(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR] could not create \"{}\": {}", path.display(), e);
                exit(1);
            }
        },
        None => {}
    }

    let mut destination_file;
    match std::fs::File::create(destination_file_path) {
        Ok(file) => {
            destination_file = file;
        }
        Err(e) => {
            eprintln!("[ERROR] could not create destination file: {}", e);
            exit(1);
        }
    }

    // work with new dimensions if present
    match matches.value_of("new_dimensions") {
        Some(new_dimensions) => match new_dimensions.split_once('x') {
            Some((nw_str, nh_str)) => {
                let new_width: u32 = nw_str.parse::<u32>().unwrap();
                let new_height: u32 = nh_str.parse::<u32>().unwrap();

                source_image = image::imageops::resize(
                    &source_image,
                    new_width,
                    new_height,
                    image::imageops::FilterType::Lanczos3,
                );
            }
            None => {}
        },
        None => {}
    }

    // define a new character set if given
    match matches.value_of("charset") {
        Some(new_charset) => {
            charset.clear();
            for new_character in new_charset.chars() {
                charset.push(new_character);
            }
        }
        None => {}
    }

    // convert an image to a character map
    let character_map = charify(&charset, &source_image);
    for y in 0..source_image.height() {
        for x in 0..source_image.width() {
            match write!(
                destination_file,
                "{}",
                character_map[(y * source_image.width() + x) as usize] as char
            ) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[ERROR] error writing to destination file: {}", e)
                }
            }
        }
        match write!(destination_file, "{}", "\n") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR] error writing to destination file: {}", e)
            }
        }
    }
}
