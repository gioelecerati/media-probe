const VIDEO_EXTENSIONS: &'static [&'static str] =
    &["mp4", "mkv", "avi", "mov", "wmv", "flv", "webm"];

pub fn walk_in_folder(path: &String) -> Vec<String> {
    // Look for video media files into folder
    let mut files: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path().to_string_lossy().to_string();
            crate::debug!("Found file: {}", file_name);
            let file_extension = entry.path().extension();

            match file_extension {
                Some(extension) => {
                    let extension = extension.to_string_lossy().to_string();
                    if VIDEO_EXTENSIONS.contains(&extension.as_str()) {
                        crate::debug!("Found video file: {}", file_name);
                        files.push(file_name);
                        paths.push(file_path);
                    }
                }
                None => {
                    crate::debug!("File {} has no extension", file_name);
                }
            }
        }
    }
    paths
}
