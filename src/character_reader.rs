use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;



fn read_file_to_string(filename: &str) -> String {
    let mut path = PathBuf::from("data");
    path.push(filename);
    path.set_extension("json");
    let mut file = File::open(&path)
                    .map_err(|err| format!("Couldn't open {}: {}", 
                                            path.display(), err.description()))
                    .unwrap();

    let mut s = String::new();

    file.read_to_string(&mut s)
        .map_err(|err| format!("Couldn't read {}: {}", 
                                path.display(), err.description()))
        .map(|_| s).unwrap()
}



use super::rustc_serialize::json;
use super::character::Character;



pub fn read_from(filename: &str) -> Result<Character, json::DecoderError> {
    json::decode::<Character>(&read_file_to_string(filename))
}
