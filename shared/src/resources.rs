use std::{io::{Write, BufReader, Read, Error}, fs::{File, self, ReadDir, DirEntry}, result, path::{PathBuf, Path}};
use serde::Serialize;

pub type Result<T> = result::Result<T, (String, Error)>;

#[profiling::function]
fn read_raw(path: impl AsRef<Path>, dir: &PathBuf) -> Result<File> {
    helpful_result(path, File::open(dir))
}

#[profiling::function]
pub fn read(path: impl AsRef<Path>) -> Result<File> {
    let dir = std::env::current_dir().unwrap().join(&path);
    read_raw(path, &dir)
}

#[profiling::function]
pub fn read_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut contents = Vec::new();
    helpful_result(&path, BufReader::new(read(&path)?).read_to_end(&mut contents))?;
    Ok(contents)
}

#[profiling::function]
pub fn read_string(path: impl AsRef<Path>) -> Result<String> {
    let mut contents = String::new();
    helpful_result(&path, BufReader::new(read(&path)?).read_to_string(&mut contents))?;
    Ok(contents)
}

#[profiling::function]
pub fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir> {
    let dir = std::env::current_dir().unwrap().join(&path);
    helpful_result(path, fs::read_dir(dir))
}

#[profiling::function]
pub fn read_dir_entry_bytes(entry: &DirEntry, file_type: Option<&str>) -> Result<Vec<u8>> {
    let dir = entry.path();

    if let Some(file_type) = file_type {
        type_match(entry, file_type)?;
    }
    
    let path = dir.to_str().unwrap();
    let mut contents = Vec::new();
    helpful_result(path, BufReader::new(read_raw(path, &dir)?).read_to_end(&mut contents))?;
    Ok(contents)
}

#[profiling::function]
pub fn read_dir_entry_string(entry: &DirEntry, file_type: Option<&str>) -> Result<String> {
    let dir = entry.path();

    if let Some(file_type) = file_type {
        type_match(entry, file_type)?;
    }
    
    let path = dir.to_str().unwrap();
    let mut contents = String::new();
    helpful_result(path, BufReader::new(read_raw(path, &dir)?).read_to_string(&mut contents))?;
    Ok(contents)
}

#[profiling::function]
fn type_match(entry: &DirEntry, file_type: &str) -> Result<()> {
    if entry.file_name().to_str().unwrap().split_once(".").unwrap().1 != file_type {
        return Result::Err((
            format!(
                "File is not of the correct type {} at {}",
                file_type, 
                entry.path().to_str().unwrap().to_string()
            ),
            Error::last_os_error(),
        ));
    }
    Result::Ok(())
}

#[profiling::function]
pub fn save(path: &str, data: &[u8]) -> Result<()> {
    let dir = std::env::current_dir().unwrap().join(path);
    let mut file = helpful_result(path, File::create(dir))?;
    helpful_result(path, file.write_all(data))?;
    Ok(())
}

#[profiling::function]
pub fn save_toml<T>(path: impl AsRef<Path>, data: T) -> Result<()> where T: Serialize {
    let data = toml::to_string(&data).unwrap();
    let dir = std::env::current_dir().unwrap().join(&path);
    let mut file = helpful_result(&path, File::create(dir))?;
    helpful_result(path, file.write_all(&data.as_bytes()))?;
    Ok(())
}

#[profiling::function]
fn helpful_result<T>(path: impl AsRef<Path>, result: std::io::Result<T>) -> Result<T> {
    match result {
        Ok(t) => Result::Ok(t),
        Err(e) => Result::Err((std::env::current_dir().unwrap().join(path).to_str().unwrap().to_string(), e)),
    }
}