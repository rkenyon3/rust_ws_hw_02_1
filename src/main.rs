use std::env::args;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct NameData {
    name: String,
}

impl NameData{
    pub fn new(name: &str) -> Self{
        Self { name:name.to_owned() }
    }
}

#[derive(Debug)]
struct NameAndNumberData {
    name: String,
    number: i32,
}

impl NameAndNumberData {
    pub fn new(name: &str, number: i32) -> Self{
        Self{
            name:name.to_owned(), 
            number
        }
    }
}

#[derive(Debug)]
enum Line {
    NameOnly(NameData),
    NameAndNumber(NameAndNumberData),
}

impl TryFrom<&str> for Line{
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains(":") {
            let parts: Vec<&str> = value.split(":").collect();
            let name = parts.get(0).;
            let number: i32 = parts.nth(1).parse()?;

            NameAndNumberData::new(name, number)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let file_name = args().nth(1).ok_or("Please provide a file name")?;

    // let lines = fs::read_to_string(file_name)?;



    Ok(())
}
