use std::fs;
use std::io;
use std::path::Path;

pub fn add_one(x: i32) -> i32 {
    return x + 1;
}

pub fn copy_resource(target_dir: &Path) -> Result<u64, io::Error> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("text_asset.txt");

    let new_file = target_dir.join("hoge");

    fs::copy(path, &new_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate tempdir;
    use tempdir::TempDir;

    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }

    #[test]
    fn tempdir_copy_test() {
        let tmp_dir = TempDir::new("").unwrap();
        let tmp_path = tmp_dir.into_path();
        let new_file = tmp_path.join("hoge");

        copy_resource(&tmp_path).unwrap();

        //
        //
        // let content = fs::read_to_string(&new_file).unwrap();
        // println!(">>>> result <<<< {}", content);
    }
}
