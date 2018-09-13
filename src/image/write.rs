use image::buffer::*;
use png;
use png::HasParameters;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug)]
pub enum WriteImageFileErr {
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

pub fn save_image<'a>(file_name: &'a str, buffer: &ImageBuffer) -> Result<(), WriteImageFileErr> {
    let path = Path::new(file_name);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, buffer.imgx as u32, buffer.imgy as u32);

    encoder.set(png::ColorType::RGB).set(buffer.bit_depth);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&buffer.buffer)?;

    Ok(())
}
