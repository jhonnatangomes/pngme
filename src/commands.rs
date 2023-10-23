use std::{fs, path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

fn open_png_file(file_path: &PathBuf) -> Result<Png> {
    let file = fs::read(file_path)?;
    TryFrom::try_from(file.as_ref())
}

pub fn encode(
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> Result<String> {
    let mut png = open_png_file(&file_path)?;
    let chunk_type: ChunkType = ChunkType::from_str(chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);
    if let Some(output_file) = output_file {
        fs::write(output_file, png.as_bytes())?;
    } else {
        fs::write(file_path, png.as_bytes())?;
    }
    Ok("Message succesfully encoded".into())
}

pub fn decode(file_path: PathBuf, chunk_type: String) -> Result<String> {
    let png = open_png_file(&file_path)?;
    match png.chunk_by_type(&chunk_type) {
        Some(chunk) => Ok(format!("{}", chunk.data_as_string()?)),
        None => Err("Chunk not found".into()),
    }
}

pub fn remove(file_path: PathBuf, chunk_type: String) -> Result<String> {
    let mut png = open_png_file(&file_path)?;
    png.remove_chunk(&chunk_type)?;
    fs::write(file_path, png.as_bytes())?;
    Ok("Chunk successfully removed".into())
}

pub fn print(file_path: PathBuf) -> Result<String> {
    let png = open_png_file(&file_path)?;
    let mut messages = String::new();
    for chunk in png.chunks() {
        if let Ok(message) = chunk.data_as_string() {
            messages.push_str(format!("{}: {}\n", chunk.chunk_type(), message).as_str());
        }
    }
    Ok(messages)
}
