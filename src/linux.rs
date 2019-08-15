use std::io::prelude::*;
use std::path::Path;
use std::io::{ self, BufReader };
use std::fs::File;

type AppError = Box<std::error::Error>;
type AppResult<T> = Result<T,AppError>;

const ID_FILE: &str = "/etc/machine-id";

// https://unix.stackexchange.com/questions/395331/is-machine-id-a-uuid
pub fn get_machine_id() -> AppResult<String>{
    //const id_file = "/sys/class/dmi/id/board_serial";
    if Path::new(ID_FILE).exists(){
        let f = File::open(ID_FILE)?;
        let reader = BufReader::new(f);
        for line in reader.lines(){
            return Ok(line.unwrap());
        }
    }
    let err = io::Error::new(io::ErrorKind::Other, "Error: Unable to read /etc/machine-id");
    Err(AppError::from(err))
}