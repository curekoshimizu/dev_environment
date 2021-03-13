use std::fs;
use std::io;
use std::path::Path;

pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

pub fn copy_resource(target_dir: &Path) -> Result<(), io::Error> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("text_asset.txt");
    assert!(path.exists());

    let new_file = target_dir.join("hoge");

    match fs::copy(path, &new_file) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate tempdir;
    use tempdir::TempDir;

    #[test]
    fn tempdir_copy_test() {
        let tmp_path = TempDir::new("").unwrap().into_path();

        copy_resource(&tmp_path).unwrap();

        let new_file = tmp_path.join("hoge");
        assert!(new_file.exists());
    }

    #[test]
    fn unknown_dir_test() {
        let tmp_path = TempDir::new("").unwrap().into_path().join("unknown_dir");

        copy_resource(&tmp_path).unwrap_err();
    }
}
