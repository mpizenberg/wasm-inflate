use libflate::deflate;
use std::{fs, io::Read, io::Write};

fn main() {
    // Read the file into a Vec<u8> buffer.
    let mut file = fs::File::open("data").expect("data does not exist");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let outfile = fs::File::create("data.gz").expect("impossible to create file");
    let mut encoder = deflate::Encoder::new(outfile);
    encoder.write_all(&buffer).unwrap();
    encoder.finish().unwrap();
}
