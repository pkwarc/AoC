use std::collections::hash_set::HashSet;
use std::error::Error;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input).unwrap());
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Card {
    winning: usize,
    count: usize,
}

fn parse_have_winning_line(l: &str) -> Result<(HashSet<i32>, HashSet<i32>)> {
    let (_, nums) = l.split_once(':').ok_or("failed to split at :")?;
    let (left, right) = nums.split_once('|').ok_or("failed to split at :")?;
    let (left, right) = (left.trim(), right.trim());
    let mut have = HashSet::new();
    for n in left.split(' ') {
        if n.trim().is_empty() {
            continue;
        }
        have.insert(n.trim().parse::<i32>()?);
    }
    let mut winning = HashSet::new();
    for n in right.split(' ') {
        if n.trim().is_empty() {
            continue;
        }
        let n = n.trim().parse::<i32>()?;
        winning.insert(n);
    }
    Ok((have, winning))
}

fn part1(input: &str) -> Result<i32> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_have_winning_line)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|(have, winning)| {
            let mut result = 0;
            for w in winning {
                if have.contains(w) {
                    result = if result >= 1 { result * 2 } else { 1 }
                }
            }
            Ok(result)
        })
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    let mut cards = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_have_winning_line)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|(have, winning)| {
            let mut result = 0;
            for w in winning {
                if have.contains(w) {
                    result += 1;
                }
            }
            Card {
                winning: result,
                count: 1,
            }
        })
        .collect::<Vec<Card>>();

    for i in 0..cards.len() {
        for _ in 0..cards[i].count {
            for j in i + 1..i + 1 + cards[i].winning {
                cards[j].count += 1;
            }
        }
    }

    let mut total = 0;
    for card in &cards {
        total += card.count;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(&input).unwrap(), 25010)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(&input).unwrap(), 9924412)
    }
}
