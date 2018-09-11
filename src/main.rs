extern crate png;

use std::fmt;
use std::error;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use png::HasParameters;

#[derive(Debug)]
enum WriteImageFileErr {
    File(io::Error),
    Encoding(png::EncodingError),
}

impl fmt::Display for WriteImageFileErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WriteImageFileErr::File(ref err) => write!(f, "File error: {}", err),
            WriteImageFileErr::Encoding(ref err) => write!(f, "Encoding error: {}", err),
        }
    }
}

impl error::Error for WriteImageFileErr {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WriteImageFileErr::File(ref err) => Some(err),
            WriteImageFileErr::Encoding(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for WriteImageFileErr {
    fn from(err: io::Error) -> WriteImageFileErr {
        WriteImageFileErr::File(err)
    }
}

impl From<png::EncodingError> for WriteImageFileErr {
    fn from(err: png::EncodingError) -> WriteImageFileErr {
        WriteImageFileErr::Encoding(err)
    }
}

///*
const BIT_DEPTH: png::BitDepth = png::BitDepth::Sixteen;
const BYTES_PER_COLOR: usize = 2;
//*/
/*
const BIT_DEPTH: png::BitDepth = png::BitDepth::Eight;
const BYTES_PER_COLOR: usize = 1;
*/

const COLORS_PER_PIXEL: usize = 3;
const BYTES_PER_PIXEL: usize = COLORS_PER_PIXEL * BYTES_PER_COLOR;
type ImageBuffer = Vec<u8>;

fn save_image(file_name: &String, buffer: &ImageBuffer, imgx: u32, imgy: u32) -> Result<(), WriteImageFileErr> {
    let path = Path::new(file_name);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, imgx, imgy);

    encoder.set(png::ColorType::RGB).set(BIT_DEPTH);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(buffer)?;

    Ok(())
}

fn main() {
    let imgx = 200;
    let imgy = 100;
    let buffer: ImageBuffer = vec![0; BYTES_PER_PIXEL * imgx * imgy];

    match save_image(&String::from("test.png"), &buffer, imgx as u32, imgy as u32) {
        Ok(()) => (),
        Err(err) => println!("error! {}", err),
    };
}