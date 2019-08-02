use inflate::{DeflateDecoder, DeflateDecoderBuf, InflateStream, InflateWriter};
use libflate::deflate;
use miniz_oxide;
use std::io::{Read, Write};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmInflate {
    file_buffer: Vec<u8>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl WasmInflate {
    pub fn new() -> WasmInflate {
        WasmInflate {
            file_buffer: Vec::new(),
        }
    }

    pub fn allocate(&mut self, length: usize) {
        self.file_buffer = vec![0; length];
    }

    pub fn memory_pos(&self) -> *const u8 {
        self.file_buffer.as_ptr()
    }

    pub fn inflate_bytes(&mut self) -> u8 {
        let _output = inflate::inflate_bytes(self.file_buffer.as_slice()).expect("inflate_bytes");
        42
    }

    pub fn deflate_decoder(&mut self) -> u8 {
        let mut output = Vec::new();
        let mut decoder = DeflateDecoder::new(self.file_buffer.as_slice());
        decoder.read_to_end(&mut output).expect("error");
        42
    }

    pub fn deflate_decoder_buf(&mut self) -> u8 {
        let mut output = Vec::new();
        let mut decoder = DeflateDecoderBuf::new(self.file_buffer.as_slice());
        decoder.read_to_end(&mut output).expect("error");
        42
    }

    pub fn inflate_stream(&mut self) -> u8 {
        let mut output = Vec::new();
        let mut inflate_stream = InflateStream::new();
        let buffer_slice = self.file_buffer.as_slice();
        let mut n = 0;
        while n < buffer_slice.len() {
            if let Ok((nb_read, res)) = inflate_stream.update(buffer_slice) {
                n += nb_read;
                output.extend(res.iter().cloned());
            } else {
                break;
            }
        }
        42
    }

    pub fn inflate_writer(&mut self) -> u8 {
        let mut writer = InflateWriter::new(Vec::new());
        writer.write(self.file_buffer.as_slice()).expect("write");
        let _decoded = writer.finish().expect("finish");
        42
    }

    pub fn libflate(&mut self) -> u8 {
        let mut output = Vec::new();
        let mut decoder = deflate::Decoder::new(self.file_buffer.as_slice());
        decoder.read_to_end(&mut output).expect("error");
        42
    }

    pub fn miniz_oxide(&mut self) -> u8 {
        let _output_7 =
            miniz_oxide::inflate::decompress_to_vec(self.file_buffer.as_slice()).expect("7");
        42
    }
}
