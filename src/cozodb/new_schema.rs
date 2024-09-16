pub fn schema() -> String {
    "
{
    :create Entry
    {
        uuid: Uuid,=>
        type: String,
        name: String,
        old_name: String,
        size:Int,
        created:String,
        lastmodified: String,
        lastmodified_server: String,
        local_created: String
        local_lastmodified: String
        local_lastmodified_server: String
        keep_offline: Bool,
        etag: String,
        status_icon:String
        path_in_cache: String,
        encryption_key:String,
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

"

"
