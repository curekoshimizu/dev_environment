use std::fs;
use std::io;
use std::path::Path;

pub fn setup(target_dir: &Path) -> Result<(), io::Error> {
    if !target_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{:?} not found", target_dir),
        ));
    }

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
    use is_executable::IsExecutable;
    use tempfile::TempDir;

    #[test]
    fn tempdir_copy_test() {
        let tmp_path;
        {
            let tmp = TempDir::new().unwrap();
            tmp_path = tmp.path().to_path_buf();
            assert!(tmp_path.exists());

            setup(&tmp_path).unwrap();

            assert!(tmp_path.join("pyproject.toml").exists());
            assert!(!tmp_path.join("pyproject.toml").is_executable());
            assert!(tmp_path.join("lint.bash").exists());
            assert!(tmp_path.join("lint.bash").is_executable());
            assert!(tmp_path.join("setup.cfg").exists());
            assert!(!tmp_path.join("setup.cfg").is_executable());
        }
        assert!(!tmp_path.exists());
    }

    #[test]
    fn unknown_dir_test() {
        let tmp = TempDir::new().unwrap();
        let tmp_path = tmp.path().to_path_buf().join("unknown_dir");
        setup(&tmp_path).unwrap_err();
    }
}
