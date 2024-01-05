use std::collections::HashMap;
use std::default::Default;
use std::env::args;
use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
pub struct NameOnly {
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
pub struct NameAndNumberData {
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

#[derive(Debug, Default)]
struct ScoreStruct {
    running_total: i32,
    score_count: i32,
    missed_test_count: i32,
}

impl ScoreStruct {
    pub fn add_score(&mut self, new_score: i32) {
        self.running_total += new_score;
        self.score_count += 1;
    }

    pub fn add_missed_test(&mut self) {
        self.missed_test_count += 1;
    }

    pub fn running_total(&self) -> i32 {
        self.running_total
    }

    pub fn score_count(&self) -> i32 {
        self.score_count
    }

    pub fn missed_test_count(&self) -> i32 {
        self.missed_test_count
    }
}

fn test_word(test_count: i32) -> &'static str {
    if test_count == 0 {
        "test"
    } else {
        "tests"
    }
}

impl Display for ScoreStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, with a total score of {}. They missed {} {}",
            self.score_count,
            test_word(self.score_count),
            self.running_total,
            self.missed_test_count,
            test_word(self.missed_test_count)
        )
    }
}

#[derive(Debug)]
pub enum Line {
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

    let parsed_lines = file_to_lines(file_name)?;

    let mut test_scores_map: HashMap<String, ScoreStruct> = HashMap::new();

    for line in parsed_lines {
        match line {
            Line::NameOnly(line) => test_scores_map
                .entry(line.name)
                .or_default()
                .add_missed_test(),
            Line::NameAndNumber(line) => test_scores_map
                .entry(line.name)
                .or_default()
                .add_score(line.number),
        }
    }

    println!("{:?}", test_scores_map);

    Ok(())
}
