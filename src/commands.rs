use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};
use std::{fs, path::PathBuf, str::FromStr};

pub fn encode(
    file_path: &PathBuf,
    chunk_type: &String,
    message: &String,
    output_file: &Option<PathBuf>,
) -> Result<String> {
    let image = fs::read(file_path)?;
    let mut png: Png = TryFrom::<&[u8]>::try_from(image.as_slice())?;
    let data: Vec<_> = message.bytes().collect();
    let chunk_type = ChunkType::from_str(chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);
    let path = match output_file {
        Some(file) => file,
        None => file_path,
    };
    fs::write(path, png.as_bytes())?;
    Ok("Message succesfully encoded".into())
}

pub fn decode(file_path: &PathBuf, chunk_type: &String) -> Result<String> {
    let image = fs::read(file_path)?;
    let png: Png = TryFrom::<&[u8]>::try_from(image.as_slice())?;
    match png.chunk_by_type(&chunk_type) {
        None => Err(format!("Chunk type {} not found in image", chunk_type).into()),
        Some(chunk) => chunk.data_as_string(),
    }
}

pub fn remove(file_path: &PathBuf, chunk_type: &String) -> Result<String> {
    let image = fs::read(file_path)?;
    let mut png: Png = TryFrom::<&[u8]>::try_from(image.as_slice())?;
    match png.chunk_by_type(&chunk_type) {
        None => return Err(format!("Chunk type {} not found in image", chunk_type).into()),
        Some(_) => {
            if let Err(e) = png.remove_chunk(&chunk_type) {
                return Err(e);
            }
        }
    };
    fs::write(file_path, png.as_bytes())?;
    Ok("Chunk succesfully removed".into())
}

pub fn print(file_path: &PathBuf) -> Result<String> {
    let image = fs::read(file_path)?;
    let png: Png = TryFrom::<&[u8]>::try_from(image.as_slice())?;
    Ok(png
        .chunks()
        .iter()
        .filter(|c| c.chunk_type().to_string() != "IHDR" && c.chunk_type().to_string() != "IEND")
        .filter(|c| c.data_as_string().is_ok())
        .map(|c| {
            format!(
                "Chunk type: {} \tChunk message: {}",
                c.chunk_type(),
                c.data_as_string().unwrap()
            )
        })
        .collect::<Vec<String>>()
        .join("\n"))
}
