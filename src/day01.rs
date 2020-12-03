use anyhow::Result;

fn main() -> Result<()> {
    let raw_input = std::fs::read_to_string("input/day1")?;
    let mut input: Vec<u32> = Vec::new();
    for line in raw_input.lines() {
        input.push(line.parse()?);
    }

    find_two(&input);
    find_three(&input);

    Ok(())
}

fn find_two(input: &Vec<u32>) {
    for (i, val1) in input.iter().enumerate() {
        for val2 in input[i + 1..].iter() {
            if val1 + val2 == 2020 {
                println!("{} {}: {}", val1, val2, val1 * val2);
                return;
            }
        }
    }
}

fn find_three(input: &Vec<u32>) {
    for (i, val1) in input.iter().enumerate() {
        for (j, val2) in input[i + 1..].iter().enumerate() {
            for val3 in input[i + j + 1..].iter() {
                if val1 + val2 + val3 == 2020 {
                    println!("{} {} {}: {}", val1, val2, val3, val1 * val2 * val3);
                    return;
                }
            }
        }
    }
}
