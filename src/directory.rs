use std::fs;

pub struct Directory {
    pub path: String,
    pub extensions: Vec<String>,
}

impl Directory {
    pub fn get_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if self
                            .extensions
                            .contains(&extension.to_str().unwrap().to_string())
                        {
                            files.append(&mut vec![path.to_str().unwrap().to_string()]);
                        }
                    }
                } else if path.is_dir() {
                    let sub_files = Directory {
                        path: path.to_str().unwrap().to_string(),
                        extensions: self.extensions.clone(),
                    }
                        .get_files();
                    files.extend(sub_files);
                }
            }
        }
        files
    }
}
