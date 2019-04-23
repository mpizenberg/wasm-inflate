use inflate::DeflateDecoder;
use libflate::deflate;
use std::io::Read;
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

    pub fn inflate(&mut self) -> u8 {
        let mut output = Vec::new();
        // let mut deflate_decoder = DeflateDecoder::new(self.file_buffer.as_slice());
        let mut deflate_decoder = deflate::Decoder::new(self.file_buffer.as_slice());
        deflate_decoder.read_to_end(&mut output).expect("error");
        42
    }
}
