use std::num::ParseIntError;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part2(input).unwrap());
}

fn part1(input: &str) -> Result<i32, ParseIntError> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut num = String::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    num.push(c);
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    num.push(c);
                    break;
                }
            }
            num
        })
        .map(|num| num.parse::<i32>())
        .sum()
}

const DIGITS_LETTERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn starts_with_digit(s: &str) -> Option<char> {
    for (i, d) in DIGITS_LETTERS.iter().enumerate() {
        if s.len() < d.len() {
            continue;
        }
        if &s[..d.len()] == *d {
            return char::from_digit(i as u32, 10);
        }
    }
    None
}

fn part2(input: &str) -> Result<i32, ParseIntError> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut num = String::new();
            for (i, c) in line.chars().enumerate() {
                if let Some(digit) = starts_with_digit(&line[i..]) {
                    num.push(digit);
                    break;
                }
                if c.is_ascii_digit() {
                    num.push(c);
                    break;
                }
            }
            for (i, c) in line.chars().rev().enumerate() {
                if let Some(digit) = starts_with_digit(&line[line.len() - i..]) {
                    num.push(digit);
                    break;
                }
                if c.is_ascii_digit() {
                    num.push(c);
                    break;
                }
            }
            num
        })
        .map(|num| num.parse::<i32>())
        .sum::<Result<i32, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(&input).unwrap(), 54561)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(&input).unwrap(), 54076)
    }
}
