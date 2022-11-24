use std::fs::{metadata, remove_dir_all, remove_file};
use std::path::PathBuf;

pub mod test_1;

struct TestEnv {
    used_paths: Vec<PathBuf>
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        // Remove files/directories in reversed order since
        // symlinks evaluate original ones instead when
        // they are evaluated like using Path.exists() method.
        // We cannot check if symlinks which are invalid in any way does really exists.
        // However, fs::remove_file method still works.
        for value in self.used_paths.iter().rev() {
            if value.exists() {
                let metadata = match metadata(value) {
                    Ok(value) => value,
                    Err(error) => {
                        dbg!(error);
                        continue;
                    }
                };

                if metadata.is_dir() {
                    match remove_dir_all(value) {
                        Ok(_) => {},
                        Err(error) => {
                            dbg!(error);
                            continue;
                        }
                    };
                } else {
                    match remove_file(value) {
                        Ok(_) => {},
                        Err(error) => {
                            dbg!(error);
                            continue;
                        }
                    };
                }

                println!("\"{}\" is safely deleted after tests.", value.to_string_lossy());
            }
        }
    }
}
