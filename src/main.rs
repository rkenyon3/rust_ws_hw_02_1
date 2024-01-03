use std::env::args;
use std::error::Error;

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

    Ok(())
}
