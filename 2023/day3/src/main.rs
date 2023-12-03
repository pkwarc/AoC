use std::error::Error;
use std::{ops::Range, vec};

fn main() {
    let input = include_str!("../input_test.txt");
    println!("{}", part1(input).unwrap());
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    value: char,
}

#[derive(Debug)]
struct Number {
    x: Range<usize>,
    y: usize,
    value: i32,
}

const fn decr_to_zero(v: usize) -> usize {
    if v - 1 == 0 {
        0
    } else {
        v - 1
    }
}

fn is_adjacent(num: &Number, sign: &Symbol) -> bool {
    let adjs = vec![
        (sign.x + 1, sign.y),
        (decr_to_zero(sign.x), sign.y),
        (sign.x, decr_to_zero(sign.y)),
        (sign.x, sign.y + 1),
        (decr_to_zero(sign.x), sign.y + 1),
        (sign.x + 1, sign.y + 1),
        (decr_to_zero(sign.x), decr_to_zero(sign.y)),
        (sign.x + 1, decr_to_zero(sign.y)),
    ];
    for (x, y) in adjs {
        if num.x.contains(&x) && num.y == y {
            return true;
        }
    }
    false
}

fn parse_line(l: &str, line_index: usize) -> Result<(Vec<Number>, Vec<Symbol>), Box<dyn Error>> {
    let mut x: Option<usize> = None;
    let mut nums = vec![];
    let mut signs = vec![];
    for (i, c) in l.chars().enumerate() {
        if c.is_ascii_digit() {
            if x.is_none() {
                x = Some(i);
            }
        } else {
            if let Some(start) = x {
                let p = Number {
                    x: Range { start, end: i },
                    y: line_index,
                    value: l[start..i].parse::<i32>()?,
                };
                x = None;
                nums.push(p);
            }
            if c != '.' {
                signs.push(Symbol {
                    x: i,
                    y: line_index,
                    value: c,
                });
            }
        }
        if i == l.len() - 1 {
            if let Some(start) = x {
                let p = Number {
                    x: Range {
                        start,
                        end: l.len(),
                    },
                    y: line_index,
                    value: l[start..l.len()].parse::<i32>()?,
                };
                x = None;
                nums.push(p);
            }
        }
    }
    Ok((nums, signs))
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut nums = vec![];
    let mut signs = vec![];
    for (line_index, l) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        let (line_nums, line_signs) = parse_line(l, line_index)?;
        nums.extend(line_nums);
        signs.extend(line_signs);
    }
    let sum = nums
        .iter()
        .filter(|num| {
            for sign in &signs {
                if is_adjacent(num, sign) {
                    return true;
                }
            }
            false
        })
        .map(|p| p.value)
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut nums = vec![];
    let mut signs = vec![];
    for (line_index, l) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        let (line_nums, line_signs) = parse_line(l, line_index)?;
        nums.extend(line_nums);
        signs.extend(line_signs);
    }
    let sum = signs
        .iter()
        .map(|sign| {
            let mut gears = vec![];
            for num in &nums {
                if is_adjacent(num, sign) && sign.value == '*' {
                    gears.push(num);
                }
            }
            if gears.len() == 2 {
                return gears[0].value * gears[1].value;
            }
            0
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(&input).unwrap(), 525119)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(&input).unwrap(), 76504829)
    }
}
