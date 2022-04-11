use std::{
    fs,
    io::{self, BufReader, Read},
    path::Path,
};

use flate2::read::GzDecoder;

pub mod access;

pub const NUM_FILES: usize = 79;
pub const HEADER: &str = "timestamp,user_id,pixel_color,coordinate\n";

pub const FILE_ORDER: [usize; NUM_FILES] = [
    10, 6, 9, 14, 15, 17, 13, 11, 7, 16, 12, 18, 24, 20, 26, 19, 21, 22, 23, 25, 27, 28, 35, 30,
    31, 32, 33, 34, 39, 42, 36, 37, 38, 43, 44, 45, 46, 48, 29, 51, 47, 54, 3, 2, 41, 0, 1, 55, 56,
    4, 50, 57, 59, 53, 40, 49, 62, 52, 65, 61, 69, 8, 63, 64, 60, 66, 67, 5, 74, 68, 75, 76, 77,
    70, 71, 72, 73, 58, 78,
];

pub fn open_dataset(path: impl AsRef<Path>) -> io::Result<BufReader<impl Read>> {
    let dataset = fs::File::open(path)?;

    let mut reader = GzDecoder::new(dataset);

    // Remove header from file.
    let mut buffer = vec![0u8; HEADER.len()];
    reader.read_exact(&mut buffer[0..HEADER.len()])?;

    // Buffer the reader so we can iterate over the contained lines
    Ok(BufReader::new(reader))
}

pub fn create_file_name(i: usize) -> String {
    format!("2022_place_canvas_history-0000000000{:0>2}.csv.gzip", i)
}
