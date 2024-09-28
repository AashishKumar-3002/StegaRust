use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn print(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    if !file_exists(path)? {
        return Err(Box::from(format!("File not found -: {}", path)));
    }

    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice())?;

    let chunk_types: Vec<String> = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect();
    
    Ok(chunk_types) // Return the chunk types
}



pub fn encode(path: &str, chunk_type: &str, message: &str) -> Result<bool, Box<dyn std::error::Error>> {

    if !file_exists(path)? {
        return Err(Box::from(format!("File not found -: {}", path)));
    }

    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();

    let i_end = png
        .remove_chunk("IEND")
        .map_err(|_| {
        Box::<dyn std::error::Error>::from("Unable to remove end chunk")
    })?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type).map_err(|_| {
            Box::<dyn std::error::Error>::from("Invalid chunk type")
        })?,
        message.as_bytes().into(),
    ));
    png.append_chunk(i_end);

    let write_path = std::path::Path::new(path);
    fs::write(write_path, png.as_bytes())?;
    println!("Message encoded!");
    Ok(true)
}

pub fn decode(path: &str, chunk_type: &str) -> Result<String, Box<dyn std::error::Error>> {

    if !file_exists(path)? {
        return Err(Box::from(format!("File not found -: {}", path)));
    }

    let buffer = get_bytes_from_path(path);
    let png = Png::try_from(buffer.as_slice()).unwrap();

    let target = png.chunk_by_type(chunk_type).ok_or_else(|| {
        Box::<dyn std::error::Error>::from(format!("No chunk found with type -: {}", chunk_type))
    })?;

    println!("Message is: {}", target.data_as_string().unwrap());

    Ok(target.data_as_string().unwrap_or_else(|_| "Could not convert data to string".to_string()))
}

pub fn remove(path: &str, chunk_type: &str) -> Result<bool, Box<dyn std::error::Error>> {

    if !file_exists(path)? {
        return Err(Box::from(format!("File not found -: {}", path)));
    }

    let buffer = get_bytes_from_path(path);
    let mut png = Png::try_from(buffer.as_slice()).unwrap();

    png.remove_chunk(chunk_type).map_err(|_| {
        Box::<dyn std::error::Error>::from(format!("Unable to remove chunk -: {}", chunk_type))
    })?;

    let write_path = std::path::Path::new(path);
    fs::write(write_path, png.as_bytes())?;
    println!("Chunk removed!");

    Ok(true)
}
fn get_bytes_from_path(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("no file found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("buffer overflow");
    buffer
}

pub fn file_exists(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(Box::new(e))
            }
        }
    }
}
