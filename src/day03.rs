use anyhow::Result;

fn main() -> Result<()> {
    let raw_input = std::fs::read_to_string("input/day3")?;

    println!("{}", explore(&raw_input, (3, 1)));

    let product: i64 = vec![
        (1_usize, 1_usize),
        (3_usize, 1_usize),
        (5_usize, 1_usize),
        (7_usize, 1_usize),
        (1_usize, 2_usize),
    ]
    .iter()
    .map(|rule| explore(&raw_input, *rule))
    .product();

    println!("{}", product);

    Ok(())
}

fn explore(input: &String, rule: (usize, usize)) -> i64 {
    let mut x_pos = 0;
    let mut trees = 0;

    for line in input.lines().step_by(rule.1) {
        if line.chars().nth(x_pos).unwrap() == '#' {
            trees += 1;
        }
        x_pos += rule.0;
        if x_pos >= line.len() {
            x_pos -= line.len();
        }
    }

    trees
}
