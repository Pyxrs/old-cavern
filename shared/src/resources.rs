use std::{io::{Write, BufReader, Read, Error}, fs::{File, self, ReadDir}, result};
use serde::Serialize;

pub type Result<T> = result::Result<T, (String, Error)>;

pub fn load(path: &str) -> Result<File> {
    let dir = std::env::current_dir().unwrap().join(path);
    helpful_result(path, File::open(dir))
}

pub fn load_bytes(path: &str) -> Result<Vec<u8>> {
    let mut contents = Vec::new();
    helpful_result(path, BufReader::new(load(path)?).read_to_end(&mut contents))?;
    Ok(contents)
}

pub fn load_dir(path: &str) -> Result<ReadDir> {
    let dir = std::env::current_dir().unwrap().join(path);
    helpful_result(path, fs::read_dir(dir))
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