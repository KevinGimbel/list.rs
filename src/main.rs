use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

/**
 * get_file_depth
 * 
 * Calculate indention based on file path
 * @param path: String
 * @return usize
*/
fn get_file_depth(path: &String) -> usize {
    let mut i: usize = 0;
    let width: usize = 2;
    let sp = path.split('/');

    for _a in sp {
        i += width;
    }

    return i - width
}

/**
 * print_dir_name
 * 
 * Write a file or directory name to the console
 * @param dir: &DirEntry
*/
fn print_dir_name(dir: &DirEntry) {
    let filepath: String = format!("{:?}", dir.path());
    let filename: String = format!("{:?}", dir.path().file_name().unwrap());
    let indent: usize = get_file_depth(&filepath);
    let fill: String = std::iter::repeat("─").take(indent).collect::<String>();
    if dir.path().is_dir() {
        println!("{:ident$}└{icon}{path}","", icon=fill, path=filename, ident=indent);
    } else {
        println!("  ├{icon}{path}", icon=fill, path=filename);
    }
    // println!("{:ident$}{path}","", path=filepath, ident=indent);
}

/**
 * visit_dirs
 * 
 * Recursively traverse directories. Calls callback function on every file
 * @param dir: &Path
 * @param cb:  &Fn(&DirEntry)
 * @return io::Result<()>
*/
fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                print_dir_name(&entry);
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    visit_dirs(&Path::new("."), &print_dir_name);
}
