use std::env::args;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct NameOnly {
    name: String,
}

impl NameOnly{
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
    NameOnly(NameOnly),
    NameAndNumber(NameAndNumberData),
}

impl TryFrom<&str> for NameAndNumberData{
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
    
        let parts: Vec<&str> = value.split(":").collect();
        let name = parts[0];
        let number: i32 = parts[1].trim().parse()?;

        Ok(NameAndNumberData::new(name, number))
        
    }
}

impl TryFrom<&str> for NameOnly{
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
    
        Ok(NameOnly::new(value))
        
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let file_name = args().nth(1).ok_or("Please provide a file name")?;

    // let lines = fs::read_to_string(file_name)?;



    Ok(())
}
