use std::fs;
use std::io::Read;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub point: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeMP {
    pub title: String,
    pub questions: Vec<Question>,
}

pub fn import(filename: &str) -> Result<TeMP, String> {

    match fs::File::open(filename) {
        Ok(mut file) => {
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);

            match serde_json::from_str::<TeMP>(&buffer) {
                Ok(temp) => {
                    Ok(temp)
                }, Err(_) => {
                    Err("Decode failed.".into())
                }
            }

        }, 
        Err(_) => {
            Err("File cannot be created.".into())
        }
    }

}