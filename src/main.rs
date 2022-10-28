use std::env;
use std::fs;


use wasmparser::{Result, Parser, Chunk, SectionReader, Payload::*};
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>>  {
    let args: Vec<String> = env::args().collect();

    let wasm_file = &args[1];

    println!("Testing file {:?}", wasm_file);
    let wasm_bytes_vec = fs::read(wasm_file)?;
    let mut wasm_bytes = &wasm_bytes_vec[..];

    objdump_headers(wasm_bytes).unwrap();

    Ok(())
}


fn objdump_headers(mut wasm: &[u8]) -> Result<()> {
    let mut parser = Parser::new(0);
    loop {
        let payload = match parser.parse(wasm, true)? {
            Chunk::Parsed { consumed, payload } => {
                wasm = &wasm[consumed..];
                payload
            }
            // this state isn't possible with `eof = true`
            Chunk::NeedMoreData(_) => unreachable!(),
        };
        match payload {
            MemorySection(mut s) => {
                let m = s.read().unwrap();
                if m.memory64 {
                    println!("Is 64!")
                } else {
                    println!("Nope")
                }
            },
            End(_) => break,
            _ => {}
        }
    }
    Ok(())
}

fn print_range(section: &str, range: &Range<usize>) {
    println!("{:>40}: {:#010x} - {:#010x}", section, range.start, range.end);
}