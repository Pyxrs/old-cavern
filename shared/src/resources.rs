use std::{io::{Write, BufReader, Read, Error}, fs::{File, self, ReadDir, DirEntry}, result, path::PathBuf};
use serde::Serialize;

pub type Result<T> = result::Result<T, (String, Error)>;

fn read_raw(path: &str, dir: &PathBuf) -> Result<File> {
    helpful_result(path, File::open(dir))
}

pub fn read(path: &str) -> Result<File> {
    let dir = std::env::current_dir().unwrap().join(path);
    read_raw(path, &dir)
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>> {
    let mut contents = Vec::new();
    helpful_result(path, BufReader::new(read(path)?).read_to_end(&mut contents))?;
    Ok(contents)
}

pub fn read_string(path: &str) -> Result<String> {
    let mut contents = String::new();
    helpful_result(path, BufReader::new(read(path)?).read_to_string(&mut contents))?;
    Ok(contents)
}

pub fn read_dir(path: &str) -> Result<ReadDir> {
    let dir = std::env::current_dir().unwrap().join(path);
    helpful_result(path, fs::read_dir(dir))
}

pub fn read_dir_entry_bytes(entry: &DirEntry) -> Result<Vec<u8>> {
    let dir = entry.path();
    let path = dir.to_str().unwrap();
    
    let mut contents = Vec::new();
    helpful_result(path, BufReader::new(read_raw(path, &dir)?).read_to_end(&mut contents))?;
    Ok(contents)
}

pub fn read_dir_entry_string(entry: &DirEntry) -> Result<String> {
    let dir = entry.path();
    let path = dir.to_str().unwrap();
    
    let mut contents = String::new();
    helpful_result(path, BufReader::new(read_raw(path, &dir)?).read_to_string(&mut contents))?;
    Ok(contents)
}

pub fn save(path: &str, data: &[u8]) -> Result<()> {
    let dir = std::env::current_dir().unwrap().join(path);
    let mut file = helpful_result(path, File::create(dir))?;
    helpful_result(path, file.write_all(data))?;
    Ok(())
}

pub fn save_toml<T>(path: &str, data: T) -> Result<()> where T: Serialize {
    let data = toml::to_vec(&data).unwrap();
    let dir = std::env::current_dir().unwrap().join(path);
    let mut file = helpful_result(path, File::create(dir))?;
    helpful_result(path, file.write_all(&data))?;
    Ok(())
}

fn helpful_result<T>(path: &str, result: std::io::Result<T>) -> Result<T> {
    match result {
        Ok(t) => Result::Ok(t),
        Err(e) => Result::Err((std::env::current_dir().unwrap().join(path).to_str().unwrap().to_string(), e)),
    }
}