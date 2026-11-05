//! `bundle <file> <dir>`: unpacks a bundle file into a directory.

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        let prog = args.first().map(String::as_str).unwrap_or("bundle");
        eprintln!("usage: {prog} <file> <dir>");
        std::process::exit(2);
    }
    let text = std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("{}: {e}", args[1]);
        std::process::exit(1);
    });
    match bundle::unpack(Path::new(&args[2]), &text) {
        Ok(w) => println!("{} files, {} lines", w.len(), bundle::line_count(&bundle::parse(&text))),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
