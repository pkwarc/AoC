use std::error::Error;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input).unwrap());
}

#[derive(Debug)]
enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

#[derive(Debug)]
struct Game {
    id: i32,
    choices: Vec<Vec<Cube>>,
}

type Err = Box<dyn Error>;

fn parse_line(line: &str) -> Result<Game, Err> {
    let Some((game, choices)) = line.split_once(':') else {
        return Err("invalid input at ':'".into());
    };
    let Some((_, id)) = game.split_once(' ') else {
        return Err("invalid input at ' '".into());
    };
    let mut game = Game {
        id: id.parse::<i32>()?,
        choices: vec![],
    };
    for p in choices.split(';') {
        let mut ch = vec![];
        for pair in p.split(',') {
            let Some((val, color)) = pair.trim().split_once(' ') else {
                return Err("invalid pair".into());
            };
            let choice = match color.trim() {
                "red" => Cube::Red(val.parse::<i32>()?),
                "blue" => Cube::Blue(val.parse::<i32>()?),
                "green" => Cube::Green(val.parse::<i32>()?),
                _ => return Err("invalid color".into()),
            };
            ch.push(choice);
        }
        game.choices.push(ch);
    }
    Ok(game)
}

fn part1(input: &str) -> Result<i32, Err> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<Game>, Err>>()?
        .iter()
        .filter(|game| {
            println!("{game:?}");
            for choices in &game.choices {
                for c in choices {
                    let fits = match c {
                        Cube::Red(v) => *v <= 12,
                        Cube::Green(v) => *v <= 13,
                        Cube::Blue(v) => *v <= 14,
                    };
                    if !fits {
                        return false;
                    }
                }
            }
            true
        })
        .map(|game| Ok(game.id))
        .sum()
}

fn part2(input: &str) -> Result<i32, Err> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<Game>, Err>>()?
        .iter()
        .map(|game| {
            println!("{game:?}");
            let mut max_r = 0;
            let mut max_b = 0;
            let mut max_g = 0;
            for choices in &game.choices {
                let r = choices.iter().fold(
                    (Cube::Red(0), Cube::Blue(0), Cube::Green(0)),
                    |acc, choice| {
                        match choice {
                            Cube::Red(v) => {
                                if let Cube::Red(val) = acc.0 {
                                    return (Cube::Red(val + v), acc.1, acc.2);
                                }
                            }
                            Cube::Blue(v) => {
                                if let Cube::Blue(val) = acc.1 {
                                    return (acc.0, Cube::Blue(val + v), acc.2);
                                }
                            }
                            Cube::Green(v) => {
                                if let Cube::Green(val) = acc.2 {
                                    return (acc.0, acc.1, Cube::Green(v + val));
                                }
                            }
                        }
                        acc
                    },
                );
                if let Cube::Red(v) = r.0 {
                    if v > max_r {
                        max_r = v;
                    }
                }
                if let Cube::Blue(v) = r.1 {
                    if v > max_b {
                        max_b = v;
                    }
                }
                if let Cube::Green(v) = r.2 {
                    if v > max_g {
                        max_g = v;
                    }
                }
            }
            max_b * max_r * max_g
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(&input).unwrap(), 2486)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(&input).unwrap(), 87984)
    }
}
