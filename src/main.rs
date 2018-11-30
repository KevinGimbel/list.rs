extern crate chrono;
extern crate getopts;

// std libs
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::env;

// getops
use getopts::Options;

// chrono
use chrono::prelude::NaiveDateTime;

/// print_all_file_info
///
/// Prints all sorts of file information
/// similar to running `ls -l`
///
/// @param entry: &DirEntry
/// @return std::io::Result<()>
fn print_all_file_info(entry: &DirEntry) {
    let name = entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    // Grab the metadata from the file
    let meta = std::fs::metadata(&entry.path())
        .expect(&format!("Unable to get info about {:?}", &entry.path()));
    // get the permissions
    let perm = meta.permissions();
    // get the last miodified date in seconds, converted from u64 to i64
    let mod_date_secs = meta.modified()
        .unwrap()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    // get the last modified date
    let time = NaiveDateTime::from_timestamp(mod_date_secs, 0);
    let files_in_dir = get_file_count_in_dir(&entry.path());

    // Icon for file or directory
    let mut icon = "f";
    // Change to directory if it is a directory
    if meta.is_dir() {
        icon = "d";
    }

    println!(
        "{icon}\t{files_in}\t{size}\t{mode}\t{time}  {name}",
        icon = icon,
        files_in = files_in_dir,
        size = meta.len(),
        mode = perm.mode(),
        name = name,
        time = time,
    );
}

/// get_file_count_in_dir
/// "ls -l" shows a file count. A single file counts as "1" and inside directories
/// everything adds +1 to the count, including the default "." and ".." folders.
fn get_file_count_in_dir(dir: &Path) -> usize {
    // If it is no directory that it is a file which counts as "1".
    let mut res: usize = 1;

    if dir.is_dir() {
        // On *nix systems every directory
        // has a "." and ".." inside.
        // Since we set res to 1 above we add +1 here
        // to match the two folders
        res += 1;
        for _ in fs::read_dir(dir) {
            res += 1;
        }
    }

    res
}

/// print_file_info
/// Wrapper function for `print_all_file_info`
/// In the future this function should run the `ls` like code
/// or the `tree` like code.
fn print_file_info(entry: &DirEntry) {
    print_all_file_info(entry);
}

/// list_files_and_dirs
/// List all files and folders in current directory
fn list_files_and_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            print_file_info(&entry);
        }
    }
    Ok(())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [dir]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let _r = list_files_and_dirs(&Path::new("."));
}
