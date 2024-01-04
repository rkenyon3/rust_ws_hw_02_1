use std::env::args;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct NameOnly {
    name: String,
}

impl NameOnly {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug)]
struct NameAndNumberData {
    name: String,
    number: i32,
}

impl NameAndNumberData {
    pub fn new(name: &str, number: i32) -> Self {
        Self {
            name: name.to_owned(),
            number,
        }
    }
}

#[derive(Debug)]
enum Line {
    NameOnly(NameOnly),
    NameAndNumber(NameAndNumberData),
}

impl TryFrom<&str> for Line {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains(':') {
            let parts: Vec<&str> = value.split(':').collect();
            let name = parts[0];
            let number: i32 = parts[1].trim().parse()?;
            let name_and_num = NameAndNumberData::new(name, number);
            Ok(Line::NameAndNumber(name_and_num))
        } else {
            Ok(Line::NameOnly(NameOnly::new(value)))
        }
    }
}

fn file_to_lines(file_name: String) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(file_name)?;

    text.lines().map(Line::try_from).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Please provide a file name")?;

    let that_vector = file_to_lines(file_name);

    println!("{:?}", that_vector);

    Ok(())
}
