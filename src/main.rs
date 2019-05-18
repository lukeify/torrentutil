use std::env;
use std::path::Path;
use std::fs::{metadata, Metadata};
use std::error::Error;
use std::ffi::OsStr;

/// Main function. Grab the argument passed in by the user. This argument either refers to a
/// directory on disk, or a file on disk.
fn main() {
    let directory_or_file_arg: Option<String> = env::args().skip(1).last();

    if let Some(x) = directory_or_file_arg {
        // Convert the string argument to a Path object.
        let path = Path::new(x);
        // Retrieve the filesystem metadata.
        let md: Result<Metadata, Error> = metadata(path);
        // Unwrap the result and continue processing if the argument was a valid directory
        // or file.
        match md {
            Ok(x) => {
                if x.is_dir() {
                    process_dir(path);
                } else if x.is_file() {
                    process_file(path);
                }
            }
            Err(e) => {
                println!("Given argument is not a file or directory.");
                return;
            }
        }

    } else {
        println!("No directory or file was given.");
        return;
    }
}

fn process_dir(path: &Path) {

}

fn process_file(path: &Path) {
    match file_name = path.file_name() {
        Some(x) => split_on_periods(x),
        None    => return
    }
}

fn split_on_periods(file_name_as_osstr: &OsStr) {
    if let str = file_name_as_osstr.to_str() {

    }
}

fn is_movie_or_episode() -> MovieOrEpisodeEnum {
    
}

fn rename_file() {

}