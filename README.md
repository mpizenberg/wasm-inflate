# wasm-inflate

Minimal inflate example in WebAssembly (wasm).

Build with:

```
wasm-pack build --target web
```

Then serve `index.html` with your static server of choice.

## Reproduce the "Big slow-down in WebAssembly" #55 issue

I am referencing here [issue #55 in the inflate crate][issue].

[issue]: https://github.com/image-rs/inflate/issues/55

1. Download the file at https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-all-titles-in-ns0.gz for "somewhat" benchmarking.
2. Rename it data.gz and place it at the root of this repository.
3. Ungzip it into a data file which should be around 300MB.
4. Remove file data.gz and `cargo run --release --bin prepare` which will use libflate to deflate data into a new data.gz file of roughly 90MB.
5. `cargo run --release --bin rustnative` for benchmark with native compilation
6. `wasm-pack build --target web` will generate an wasm code.
7. Load index.html page with the server of your choice (`python -m http.server 8080`).
8. Open console in dev tools and use the file loader in the page to start wasm benchmark.

## Bench results

On my computer, with rustc 1.36.0, inflate 0.4.5, libflate 0.1.25, wasm-bindgen 0.2.47,
I have the following results with native rust compilation:

```
Elapsed (inflate_bytes): Ok(2.85300905s)
Elapsed (DeflateDecoder): Ok(3.021417847s)
Elapsed (DeflateDecoderBuf): Ok(2.992962828s)
Elapsed (InflateStream): Ok(2.565384845s)
Elapsed (InflateWriter): Ok(2.905209633s)
Elapsed (libflate): Ok(2.312502443s)
```

And in wasm with Firefox 67.0.4 I got:

```
inflate_bytes: 9991 milliseconds.
deflate_decoder: 9434 milliseconds.
deflate_decoder_buf: 9258 milliseconds.
inflate_stream: 7964 milliseconds.
inflate_writer: 8848 milliseconds.
libflate: 4902 milliseconds.
```
