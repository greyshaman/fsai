use std::fs;
use std::fs::{Metadata, FileType};


fn main() -> std::io::Result<()> {
    // 1. read current directory
    let dir_path = ".";

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            // let mut metadata: Metadata;
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let f_type = if metadata.file_type().is_dir() {
                        "Directory"
                    } else if metadata.file_type().is_file() {
                        "File"
                    } else {
                        "SymLink"
                    };
                    println!(
                        "{} {:?} permissions: {:?}", 
                        f_type,
                        entry.path(), 
                        metadata.permissions()
                    );
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }

    // for entry in fs::read_dir(".")? {
    //     let dir_entry = entry?;
    //     // 2. show file list
    //     println!("current directory: {:?}", dir_entry.path());
    //     println!("\\----> metadata: {:?}", dir_entry.metadata());
    // }
    Ok(())
}

// fn get_directory() {}
