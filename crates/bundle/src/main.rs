//! `bundle <file> <dir>`: unpacks a bundle file into a directory.

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: bundle <bundle-file> <out-dir>");
        std::process::exit(2);
    }
    let text = std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("{}: {e}", args[1]);
        std::process::exit(1);
    });
    match bundle::unpack(Path::new(&args[2]), &text) {
        Ok(w) => println!("{} files, {} lines", w.len(), bundle::line_count(&bundle::parse(&text))),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
