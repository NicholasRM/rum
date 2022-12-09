use std::io::Read;

pub fn parse_input(input: Option<&str>) -> Vec<u32> {
    let file_name = input.unwrap();
    let mut reader = std::fs::File::open(file_name).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    buffer.chunks_exact(4).map(| words | u32::from_be_bytes(words.try_into().unwrap())).collect()
}