use anyhow::Result;

fn main() {
    let input = load("input/day4").unwrap();
    println!("{}", validate_part1(&input));
    println!("{}", validate_part2(&input));
}

fn load(input: &str) -> Result<String> {
    Ok(std::fs::read_to_string(input)?)
}

#[test]
fn test_load() {
    assert_eq!(load("input/day4tdd").unwrap().lines().count(), 13);
}

fn validate_part1(input: &str) -> usize {
    let input = input.replace('\n', " ").replace("  ", "\n");
    input
        .lines()
        .filter(|line| validate_passport_part1(line))
        .count()
}

fn validate_passport_part1(line: &str) -> bool {
    let fields = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid"*/
    ];
    fields.iter().filter(|field| line.contains(*field)).count() == fields.len()
}

#[test]
fn test_part1() {
    let input = load("input/day4tdd").unwrap();
    assert_eq!(validate_part1(&input), 2);
}

/// Stolen from https://old.reddit.com/r/rust/comments/k6fgrs/advent_of_code_2020_day_4/gelwfuz/
/// as I find it readable and efficient!
fn validate_part2(line: &str) -> usize {
    line.split_terminator("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|p| p.split(':'))
                .filter_map(|mut pair| {
                    let in_range = |v: &str, a, b| Some((a..=b).contains(&v.parse::<u32>().ok()?));
                    let (key, val) = (pair.next()?, pair.next()?);
                    Some(match (key, val.len()) {
                        ("byr", 4) if in_range(val, 1920, 2002)? => 0,
                        ("iyr", 4) if in_range(val, 2010, 2020)? => 1,
                        ("eyr", 4) if in_range(val, 2020, 2030)? => 2,
                        ("hgt", _)
                            if val
                                .strip_suffix("cm")
                                .and_then(|h| in_range(h, 150, 193))
                                .or_else(|| {
                                    val.strip_suffix("in").and_then(|h| in_range(h, 59, 76))
                                })? =>
                        {
                            3
                        }
                        ("hcl", 7)
                            if val
                                .strip_prefix('#')?
                                .chars()
                                .all(|c| matches!(c,'0'..='9'|'a'..='f')) =>
                        {
                            4
                        }
                        ("pid", 9) if val.chars().all(|c| c.is_ascii_digit()) => 5,
                        ("ecl", 3)
                            if matches!(
                                val,
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                            ) =>
                        {
                            6
                        }
                        _ => return None,
                    })
                })
                .fold(0, |acc, index| acc | (1 << index))
        })
        .filter(|&bitset| bitset == 0b111_1111)
        .count()
}
