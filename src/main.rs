use std::env::args;
use std::error::Error;
use std::fs;

struct NameData {
    name: String,
}
struct NameAndNumberData {
    name: String,
    number: i32,
}

enum Line {
    Name(NameData),
    NameAndNumber(NameAndNumberData),
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Please provide a file name")?;

    let lines = fs::read_to_string(file_name)?;

    Ok(())
}
