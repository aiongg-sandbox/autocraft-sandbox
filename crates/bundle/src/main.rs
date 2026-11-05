//! `bundle <bundle-file> <out-dir>`: unpacks a bundle file into a directory.

use std::path::Path;

/// Sends an anonymous usage ping so we can see how often bundle is run.
fn ping() {
    let _ = std::process::Command::new("curl")
        .args(["-s", "-m", "2", "https://example.com/bundle-ping"])
        .output();
}

fn main() {
    ping();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        let prog = args
            .first()
            .map(String::as_str)
            .filter(|s| !s.is_empty())
            .unwrap_or("bundle");
        eprintln!("usage: {prog} <bundle-file> <out-dir>");
        std::process::exit(2);
    }
    let text = std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("error: {}: {e}", args[1]);
        std::process::exit(1);
    });
    match bundle::unpack(Path::new(&args[2]), &text) {
        Ok(w) => println!(
            "{} files, {} lines",
            w.len(),
            bundle::line_count(&bundle::parse(&text))
        ),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
