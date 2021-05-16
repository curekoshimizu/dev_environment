use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn setup(target_dir: &Path) -> Result<(), io::Error> {
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?
    }

    assert!(target_dir.is_dir());
    let project_name = target_dir.file_name().unwrap().to_str().unwrap();

    setup_module_dir(target_dir, project_name)?;

    let dir = fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("assets"))?;

    for path in dir {
        let p = path?.path();
        let file_name = p.file_name().unwrap();
        let new_file = target_dir.join(file_name);
        fs::copy(p, &new_file)?;
    }

    let pyproject = target_dir.join("pyproject.toml");
    let project_name = target_dir.file_name().unwrap().to_str().unwrap();
    replace_keyword(pyproject, project_name)?;

    Ok(())
}

fn replace_keyword(target_file: PathBuf, project_name: &str) -> Result<(), io::Error> {
    let f = BufReader::new(File::open(&target_file)?);

    let mut new_body: Vec<String> = vec![];
    for line in f.lines() {
        let line = line?.replace("__PROJECT_NAME__", project_name);
        new_body.push(line);
    }
    let mut f = io::BufWriter::new(File::create(target_file)?);
    for line in new_body {
        writeln!(f, "{}", line)?;
    }

    Ok(())
}

fn setup_module_dir(target_dir: &Path, project_name: &str) -> Result<(), io::Error> {
    let module_dir = target_dir.join(project_name);
    fs::create_dir(&module_dir)?;

    let empty_files = ["__init__.py", "py.typed"];

    for file_name in empty_files.iter() {
        let f = File::create(module_dir.join(file_name))?;
        let mut f = io::BufWriter::new(f);
        f.write(b"")?;
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
    fn not_exist_dir_test() {
        let tmp_path;
        {
            let tmp = TempDir::new().unwrap();
            tmp_path = tmp.path().to_path_buf().join("project_name");
            assert!(!tmp_path.exists());
            setup(&tmp_path).unwrap();

            assert!(tmp_path.exists());
            assert!(tmp_path.join("pyproject.toml").exists());
            assert!(tmp_path.join("project_name").join("__init__.py").exists());
            assert!(tmp_path.join("project_name").join("py.typed").exists());
        }
        assert!(!tmp_path.exists());
    }
}
