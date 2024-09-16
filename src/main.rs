// use cozo::*;
mod utils;
use txns::CozoDB;
use utils::types::*;
mod cozodb;
use cozodb::*;
use uuid::Uuid;
use walkdir::WalkDir;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("took default value");
        "/Users/vrn21/Developer/DummyDrive".to_string()
    });
    let parent = std::env::args().nth(2).unwrap_or_else(|| {
        println!("took default value");
        "/Users/vrn21/Developer/DummyDrive/child".to_string()
    });

    let mut all_folders: Vec<DirectoryFS> = Vec::new();
    let mut all_files: Vec<FileFS> = Vec::new();

    for entry in WalkDir::new(&path)
        .follow_links(false)
        .follow_root_links(true)
    {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    all_folders.push(DirectoryFS {
                        uuid: Uuid::new_v4().to_string(),
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry.path().display().to_string(),
                        entry: "Directory".to_string(),
                        parent: entry
                            .path()
                            .parent()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|| "no parent".to_string()),
                    });
                } else if entry.file_type().is_file() {
                    all_files.push(FileFS {
                        uuid: Uuid::new_v4().to_string(),
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry.path().display().to_string(),
                        entry: "File".to_string(),
                        parent: entry
                            .path()
                            .parent()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|| "no parent".to_string()),
                    });
                }
            }
            Err(error) => {
                println!("Read dir_entry error: {}", error);
            }
        }
    }
    let db = CozoDB::new();
    db.insert_many_dirs(all_folders);
    db.insert_many_files(all_files);
    db.show_all();
    db.get_entries_in_dir(parent.clone());
    let uuid =
        db.get_uuid_from_path("/Users/vrn21/Developer/DummyDrive/child/child_of_child".to_string());
    //db.delete_dir_from_uuid(&uuid);
    db.get_entries_in_dir(parent.clone());
    //db.delete_dir_from_path("/Users/vrn21/Developer/DummyDrive/child/child_of_child".to_string());
    // db.update_path_dir("updated path".to_string(), uuid);
    db.rename_file("newer name".to_string(), uuid);
    db.get_entries_in_dir(parent.clone());
    // db.show_all();
}
