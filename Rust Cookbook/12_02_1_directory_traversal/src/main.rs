use std::{env, fs};
use std::io;
use std::path::{Path, PathBuf};
use same_file::is_same_file;
use std::collections::HashMap;
use walkdir::{DirEntry, WalkDir};
use glob::glob;

#[allow(unused_must_use)]
fn main() -> Result<(), Box <dyn std::error::Error>> {
  files_modified_last_24_hour()?;
  find_loops();
  duplicate_files();
  find_files_given_predicate();

  WalkDir::new(".")
    .into_iter()
    .filter_entry(|e| is_not_hidden(e))
    .filter_map(|v| v.ok())
    .for_each(|x| println!("{}", x.path().display()));

  total_size_path();
  find_all("**/*.png");

  Ok(())
}

fn files_modified_last_24_hour() -> Result<(), Box <dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

fn find_loops() -> Result<(), Box <dyn std::error::Error>> {
    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );

    Ok(())
}

fn contains_loop<P: AsRef<Path>>(path: P) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path = path.as_ref();
    let mut path_buf = path.to_path_buf();
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
        } else if let Some(looped_paths) = contains_loop(&path_buf)? {
            return Ok(Some(looped_paths));
        }
    }
    return Ok(None);
}

fn duplicate_files() -> () {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }
}

fn find_files_given_predicate() -> Result<(), Box <dyn std::error::Error>> {
    
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}


fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || !s.starts_with("."))
         .unwrap_or(false)
}

fn total_size_path() -> () {
    let total_size = WalkDir::new(".")
    .min_depth(1)
    .max_depth(3)
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.metadata().ok())
    .filter(|metadata| metadata.is_file())
    .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}

fn find_all(filter : &str) -> Result<(), Box <dyn std::error::Error>> {
    for entry in glob(filter)? {
        println!("{}", entry?.display());
    }

    Ok(())
}