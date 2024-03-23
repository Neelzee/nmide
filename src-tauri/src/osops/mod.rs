use eyre::{Context, Result};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Reads from the file into a buffer
pub fn read_file(file: &File) -> Result<String> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .wrap_err("Failed reading from file into buffer")?;
    Ok(buffer)
}

// Writes content onto file
pub fn write_to_file(file: &File, content: String) -> Result<()> {
    let mut writer = BufWriter::new(file);
    writer
        .write_all(content.as_bytes())
        .wrap_err("Failed writing content into file")?;
    Ok(())
}

fn visit_dirs_recursive(
    dir: &Path,
    depth: usize,
    paths: &mut Vec<PathBuf>,
) -> Result<(), std::io::Error> {
    if depth == 0 {
        return Ok(());
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            paths.push(path.clone());

            if path.is_dir() {
                visit_dirs_recursive(&path, depth - 1, paths)?;
            }
        }
    }

    Ok(())
}

/// Returns a list of all the paths in the given directory
///
/// Recursively checks for until level = 0
pub fn get_paths(path: &Path, level: usize) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    visit_dirs_recursive(path, level, &mut paths)?;
    Ok(paths)
}
