use std::fmt::format;

use crate::{DirectoryFS, FileFS};

pub fn create_folder_file_scehma() -> &'static str {
    "
        {
            :create Directory
            {
                uuid: String,
                =>
                name: String,
                parent: String,
                path: String,
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
        "?[uuid,name,parent,path,entry] <-[['{}','{}','{}','{}','{}']]
        :put File {{uuid,name,parent,path,entry}}",
        file.uuid, file.name, file.parent, file.path, file.entry
    )
}

pub fn insert_directory_script(dir: &DirectoryFS) -> String {
    format!(
        "?[uuid,name,parent,path,entry] <-[['{}','{}','{}','{}','{}']]:put Directory{{uuid,name,parent,path,entry}}",
        dir.uuid, dir.name, dir.parent, dir.path, dir.entry
    )
}

pub fn show_all_dir_files() -> String {
    format!("
        files[uuid,name,parent,path,entry] := *File{{uuid,name,parent,path,entry}}
        folders[uuid,name,parent,path,entry] := *Directory{{uuid,name,parent,path,entry}}
        ?[uuid,name,parent,path,entry] := files[uuid,name,parent,path,entry] or folders[uuid,name,parent,path,entry]
    ")
}

pub fn contents_of_dir(parent_uuid: String) -> String {
    format!(
        "
        files[uuid,name,parent,path,entry] := *File{{uuid,name,parent,path,entry}},parent='{}'
        folders[uuid,name,parent,path,entry] := *Directory{{uuid,name,parent,path,entry}},parent='{}'
        ?[uuid,name,parent,path,entry] := files[uuid,name,parent,path,entry] or folders[uuid,name,parent,path,entry]
        ",
        parent_uuid, parent_uuid
    )
}

pub fn get_uuid_script(path: String) -> String {
    format!(
        "?[uuid] := *File{{uuid,path}} or *Directory{{uuid,path}}, path='{}'",
        path
    )
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

pub fn update_parent(uuid: String, new_parent: String, entry: String) -> String {
    format!(
        "
            ?[uuid, parent] := uuid = {}, *Directory{{uuid}}, parent='{}'
            :update Directory {{uuid,parent}}
        ",
        uuid, new_parent
    )
}

pub fn rename_file(uuid: String, new_name: String, entry: String) -> String {
    format!(
        "
        ?[uuid,name] := uuid={},*Directory{{uuid}},name='{}'
        :update Directory{{uuid,name}}
    ",
        uuid, new_name
    )
}
