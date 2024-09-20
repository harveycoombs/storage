pub mod files {
    use std::fs;
    use std::path::Path;

    pub fn list_directory_files(path: &str) -> Vec<String> {
        if !directory_exists(path) {
            return Vec::new();
        }
        
        return fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect();
    }

    pub fn directory_exists(path: &str) -> bool {
        let path = Path::new(path);
        return path.exists() && path.is_dir();
    }

    pub fn create_directory(path: &str) -> bool {
        return match fs::create_dir_all(path) {
            Ok(_) => true,
            Err(_) => false,
        };
    }

    pub fn delete_directory(path: &str) -> bool {
        return match fs::remove_dir_all(path) {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}