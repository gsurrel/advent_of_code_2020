use anyhow::Result;

#[derive(Debug)]
pub struct Line<'a> {
    pub letter: char,
    pub min: u8,
    pub max: u8,
    pub pass: &'a str,
}

fn main() -> Result<()> {
    let raw_input = std::fs::read_to_string("input/day2")?;
    let mut input: Vec<Line> = Vec::new();
    for line in raw_input.lines() {
        let letter;
        let min;
        let max;
        let pass;
        let items: Vec<_> = line.split_ascii_whitespace().collect();

        let pos = items[0].find('-').unwrap();
        min = items[0][0..pos].parse()?;
        max = items[0][pos + 1..].parse()?;
        letter = items[1].chars().nth(0).unwrap();
        pass = items[2];

        let line = Line {
            letter,
            min,
            max,
            pass,
        };

        input.push(line);
    }

    println!("{}", number_of_valid_passwords(&input));
    println!("{}", number_of_valid_passwords_new_policy(&input));

    Ok(())
}

pub fn number_of_valid_passwords(input: &Vec<Line>) -> i32 {
    let mut valid = 0;
    for line in input {
        let mut count = 0;
        for char in line.pass.chars() {
            if char == line.letter {
                count += 1;
            }
        }
        if count >= line.min && count <= line.max {
            valid += 1;
        }
    }

    valid
}

pub fn number_of_valid_passwords_new_policy(input: &Vec<Line>) -> i32 {
    let mut valid = 0;
    for line in input {
        let mut count = 0;
        if line.pass.chars().nth(line.min as usize - 1).unwrap_or('!') == line.letter {
            count += 1;
        }
        if line.pass.chars().nth(line.max as usize - 1).unwrap_or('!') == line.letter {
            count += 1;
        }
        if count == 1 {
            valid += 1;
        }
    }

    valid
}
