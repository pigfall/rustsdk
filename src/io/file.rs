use std::fs;
use std::io;
use std::io::Read;

pub fn read_file(filepath: String)->Result<Vec<u8>,io::Error>{
    let mut file = fs::File::open(filepath)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;
    return Ok(bytes)
}
