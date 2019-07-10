use inflate::{DeflateDecoder, DeflateDecoderBuf, InflateStream, InflateWriter};
use libflate::deflate;
use std::time::SystemTime;
use std::{fs, io::Read, io::Write};

fn main() {
    let mut file = fs::File::open("data.gz").expect("File::open");
    let mut buffer = Vec::new();

    // Read the file into a Vec<u8> buffer.
    file.read_to_end(&mut buffer).unwrap();

    // 1. inflate_bytes (2.8s native, 20.6s wasm)
    let now = SystemTime::now();
    let _output_1 = inflate::inflate_bytes(buffer.as_slice()).expect("inflate_bytes");
    println!("Elapsed (inflate_bytes): {:?}", now.elapsed());

    // 2. DeflateDecoder (0.64s native, 4.6s wasm)
    let now = SystemTime::now();
    let mut output_2 = Vec::new();
    let mut deflate_decoder = DeflateDecoder::new(buffer.as_slice());
    deflate_decoder.read_to_end(&mut output_2).expect("2");
    println!("Elapsed (DeflateDecoder): {:?}", now.elapsed());

    // 3. DeflateDecoderBuf (3.0s native)
    let now = SystemTime::now();
    let mut output_3 = Vec::new();
    let mut deflate_decoder_buf = DeflateDecoderBuf::new(buffer.as_slice());
    deflate_decoder_buf.read_to_end(&mut output_3).expect("3");
    println!("Elapsed (DeflateDecoderBuf): {:?}", now.elapsed());

    // 4. InflateStream (2.8s native)
    let now = SystemTime::now();
    let mut output_4 = Vec::new();
    let mut inflate_stream = InflateStream::new();
    let buffer_slice = buffer.as_slice();
    let mut n = 0;
    while n < buffer_slice.len() {
        if let Ok((nb_read, res)) = inflate_stream.update(buffer_slice) {
            n += nb_read;
            output_4.extend(res.iter().cloned());
        } else {
            break;
        }
    }
    println!("Elapsed (InflateStream): {:?}", now.elapsed());

    // 5. InflateWriter (2.9s native)
    let now = SystemTime::now();
    let output_5 = Vec::new();
    let mut inflate_writer = InflateWriter::new(output_5);
    inflate_writer.write(buffer_slice).expect("write");
    let _decoded_5 = inflate_writer.finish().expect("finish");
    println!("Elapsed (InflateWriter): {:?}", now.elapsed());

    // 6. libflate (2.2s native, 4.8s wasm)
    let now = SystemTime::now();
    let mut output_6 = Vec::new();
    let mut libflate_decoder = deflate::Decoder::new(buffer.as_slice());
    libflate_decoder.read_to_end(&mut output_6).expect("6");
    println!("Elapsed (libflate): {:?}", now.elapsed());
}
