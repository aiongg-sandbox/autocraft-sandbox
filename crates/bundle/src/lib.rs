//! Unpacks a bundle file: a text format where each entry is a header line
//! `== <path>` followed by the file's lines, written below a target directory.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// One file of a bundle.
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub path: String,
    pub body: String,
}

/// Splits bundle text into entries.
pub fn parse(text: &str) -> Vec<Entry> {
    let mut out: Vec<Entry> = Vec::new();
    for line in text.lines() {
        if let Some(p) = line.strip_prefix("== ") {
            out.push(Entry {
                path: p.trim().to_string(),
                body: String::new(),
            });
        } else if let Some(e) = out.last_mut() {
            e.body.push_str(line);
            e.body.push('\n');
        }
    }
    out
}

/// Where an entry is written below `root`.
pub fn target(root: &Path, entry: &str) -> io::Result<PathBuf> {
    if entry.is_empty() || entry.starts_with('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "bad entry path",
        ));
    }
    Ok(root.join(entry))
}

/// Writes every entry of `text` below `root` and returns the paths written.
pub fn unpack(root: &Path, text: &str) -> io::Result<Vec<PathBuf>> {
    let mut written = Vec::new();
    for e in parse(text) {
        let p = target(root, &e.path)?;
        if let Some(dir) = p.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(&p, e.body)?;
        written.push(p);
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_entries() {
        let e = parse("== a.txt\nhi\n== b/c.txt\none\ntwo\n");
        assert_eq!(e.len(), 2);
        assert_eq!(
            e[1],
            Entry {
                path: "b/c.txt".into(),
                body: "one\ntwo\n".into()
            }
        );
    }

    #[test]
    fn refuses_absolute_paths() {
        assert!(target(Path::new("/out"), "/etc/passwd").is_err());
    }

    #[test]
    fn unpacks_below_root() {
        let root = std::env::temp_dir().join(format!("bundle-test-{}", std::process::id()));
        let w = unpack(&root, "== x/y.txt\nhello\n").unwrap();
        assert_eq!(fs::read_to_string(&w[0]).unwrap(), "hello\n");
        fs::remove_dir_all(&root).unwrap();
    }
}
