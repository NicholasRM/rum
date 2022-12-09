use std::io::Read;

/// Function to accept a path to a file and extract the information from the file into a `Vec<u32>`
/// This code was adapted from the `rumdump` lab and was provided by Noah Daniels.
/// 
/// This function will panic iff no file was entered, the file could not be opened,
/// or if an issue was encountered when reading the file to the end.
/// 
/// Returns a `Vec<u32>` representing all of the instructions in the file.
/// 
/// # Arguments:
/// * `input`: The first argument, provided from env::args.
pub fn parse_input(input: Option<&str>) -> Vec<u32> {
    let file_name = input.unwrap();
    let mut reader = std::fs::File::open(file_name).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    buffer.chunks_exact(4).map(| words | u32::from_be_bytes(words.try_into().unwrap())).collect()
}