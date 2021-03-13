use std::fs;
use std::io;
use std::path::Path;

pub fn setup(target_dir: &Path) -> Result<(), io::Error> {
    let dir = fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("assets"))?;

    for path in dir {
        let p = path?.path();
        let file_name = p.file_name().unwrap();
        let new_file = target_dir.join(file_name);
        fs::copy(p, &new_file)?;
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    extern crate tempdir;
    use tempdir::TempDir;

    #[test]
    fn tempdir_copy_test() {
        let tmp_path = TempDir::new("").unwrap().into_path();

        setup(&tmp_path).unwrap();

        assert!(tmp_path.join("pyproject.toml").exists());
        assert!(tmp_path.join("lint.bash").exists());
        assert!(tmp_path.join("setup.cfg").exists());
    }

    #[test]
    fn unknown_dir_test() {
        let tmp_path = TempDir::new("").unwrap().into_path().join("unknown_dir");

        setup(&tmp_path).unwrap_err();
    }
}
