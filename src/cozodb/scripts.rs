use std::fmt::format;

use crate::{DirectoryFS, FileFS};

pub fn create_folder_file_scehma() -> &'static str {
    "
        {
            :create Directory
            {
                uuid: String, =>
                name: String,
                parent: String,
                path: String,
                entry:String
            }
        }

        {
            :create File
            {
                uuid: String, =>
                name: String,
                parent: String,
                path: String,
                entry:String
            }
        }

    "
}

pub fn insert_file_script(file: &FileFS) -> String {
    format!(
        "?[uuid,name,parent,path,entry] <-[['{uuid}','{name}','{parent}','{path}','{entry}']]",
        uuid = file.uuid,
        name = file.name,
        parent = file.parent,
        path = file.path,
        entry = file.entry
    ) + ":put File {uuid,name,parent,path,entry}"
}

pub fn insert_directory_script(dir: &DirectoryFS) -> String {
    format!(
        "?[uuid,name,parent,path,entry] <-[['{uuid}','{name}','{parent}','{path}','{entry}']]",
        uuid = dir.uuid,
        name = dir.name,
        parent = dir.parent,
        path = dir.path,
        entry = dir.entry
    ) + ":put Directory{uuid,name,parent,path,entry}"
}

pub fn show_all_dir_files() -> String {
    "
        files[uuid,name,parent,path,entry] := *File{uuid,name,parent,path,entry}
        folders[uuid,name,parent,path,entry] := *Directory{uuid,name,parent,path,entry}
        ?[uuid,name,parent,path,entry] := files[uuid,name,parent,path,entry] or folders[uuid,name,parent,path,entry]
    "
    .to_string()
}

pub fn contents_of_dir(parent_uuid: String) -> String {
    let mut result = "".to_string();
    result.push_str(
        "
        files[uuid,name,parent,path,entry] := *File{uuid,name,parent
        ,path,entry},parent='",
    );
    result.push_str(parent_uuid.as_str());
    result.push_str(
        "'
        folders[uuid,name,parent,path,entry] := *Directory{uuid,name,parent,path,entry},parent='",
    );
    result.push_str(parent_uuid.as_str());

    result.push_str("'
        ?[uuid,name,parent,path,entry] := files[uuid,name,parent,path,entry] or folders[uuid,name,parent,path,entry]
    ");
    result
}

pub fn get_uuid_script(path: String) -> String {
    let mut result = "?[uuid] := *File{uuid,path} or *Directory{uuid,path}, path='".to_string();
    result.push_str(&path);
    result.push_str("'");
    result
}

pub fn delete_dir_from_uuid_script(uuid: &str) -> String {
    format!(
        "
        ?[uuid, name, parent, path,entry] := *Directory[uuid, name, parent, path,entry],uuid = {}
        :delete Directory{{uuid, name, parent, path,entry}}
    ",
        uuid
    )
}

pub fn delete_dir_from_path_script(path: String) -> String {
    format!(
        "
            ?[uuid, name, parent, path,entry] := *Directory[uuid, name, parent, path,entry],path = '{}'
            :delete Directory{{uuid, name, parent, path,entry}}
        ",path
    )
}

pub fn update_path(uuid: String, new_path: String, entry: String) -> String {
    format!(
        "
            ?[uuid, path] := uuid = {}, *Directory{{uuid}}, path='{}'
            :update Directory {{uuid,path}}
        ",
        uuid, new_path
    )
}
// "
// ?[id,Age] := id = 1, *person{id}, Age = 29
// :update person {id,Age}
// "
// pub fn update_name(uuid: &str) -> String {}
