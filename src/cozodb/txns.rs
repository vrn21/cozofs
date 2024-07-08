use crate::cozodb::scripts::*;
use crate::utils::types;
use crate::{DirectoryFS, FileFS};
use cozo::*;

pub struct CozoDB {
    db: DbInstance,
}

impl CozoDB {
    pub fn new() -> Self {
        let cozoDB = Self {
            db: DbInstance::new("mem", "", Default::default()).unwrap(),
        };
        cozoDB.run(create_folder_file_scehma());
        cozoDB
    }

    pub fn run(&self, script: &str) -> Result<NamedRows, String> {
        self.db
            .run_script(script, Default::default(), ScriptMutability::Mutable)
            .map_err(|e| {
                println!("{:?}", e);
                println!(" \n \n While executing command: {}", script);
                e.to_string()
            })
    }

    pub fn insert_file(&self, file: FileFS) {
        let script = insert_file_script(&file);
        self.run(&script);
    }

    pub fn insert_dir(&self, dir: DirectoryFS) {
        let script = insert_directory_script(&dir);
        self.run(&script);
    }

    pub fn insert_many_files(&self, files: Vec<FileFS>) {
        for file in files {
            self.insert_file(file);
        }
    }
    pub fn insert_many_dirs(&self, dirs: Vec<DirectoryFS>) {
        for dir in dirs {
            self.insert_dir(dir);
        }
    }

    pub fn show_all(&self) {
        let result = self.run(&show_all_dir_files());
        print!("results \n {:#?} ", result);
    }

    pub fn get_entries_in_dir(&self, parent_uuid: String) {
        let result = self.run(&contents_of_dir(parent_uuid));
        print!("results \n {:#?} ", result);
    }

    pub fn get_uuid_from_path(&self, path: String) -> String {
        let result = self.run(&get_uuid_script(path));
        let ret = &result.unwrap();
        let ret = ret.rows.get(0).ok_or_else(|| "not found").unwrap().get(0);
        println!("results are:{:?}", ret.unwrap().to_string());
        ret.unwrap().to_string()
    }

    pub fn delete_dir_from_uuid(&self, uuid: &str) {
        self.run(&delete_dir_from_uuid_script(uuid));
        println!(
            "deleted from uuid:{} \n script: {}",
            uuid,
            &delete_dir_from_uuid_script(uuid)
        );
    }

    pub fn delete_dir_from_path(&self, path: String) {
        self.run(&delete_dir_from_path_script(path.clone()));
        println!("deleted directory {}", path);
    }

    pub fn update_path_dir(&self, new_path: String, uuid: String) {
        self.run(&update_path(uuid.clone(), new_path, "dir".to_string()));
    }
}
