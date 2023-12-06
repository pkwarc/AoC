use std::error::Error;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(input).unwrap());
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn get_line<'a>(mut iter: impl std::iter::Iterator<Item = &'a str>) -> Result<&'a str> {
    let line = iter.next().ok_or("no line found")?;
    let (_, line) = line.split_once(':').ok_or("cannot splitline with :")?;
    Ok(line)
}

fn part1(input: &str) -> Result<usize> {
    let mut iter = input.lines();
    let mut times = vec![];
    for val in get_line(&mut iter)?.split_whitespace() {
        times.push(val.parse::<usize>()?);
    }
    let mut distances = vec![];
    for val in get_line(&mut iter)?.split_whitespace() {
        distances.push(val.parse::<usize>()?);
    }

    let mut ways = vec![];
    for (i, &t) in times.iter().enumerate() {
        let mut win_moves = vec![];
        for push_time in 0..t {
            let speed = push_time;
            let distance = (t - push_time) * speed;
            if distance > distances[i] {
                win_moves.push(distance);
            }
        }
        ways.push(win_moves.len());
    }
    Ok(ways.iter().product())
}

fn part2(input: &str) -> Result<usize> {
    let mut iter = input.lines();
    let mut time = String::new();
    for val in get_line(&mut iter)?.split_whitespace() {
        time.push_str(val);
    }
    let time = time.parse::<usize>()?;
    let mut distance = String::new();
    for val in get_line(&mut iter)?.split_whitespace() {
        distance.push_str(val);
    }
    let distance = distance.parse::<usize>()?;

    let mut ways = 0usize;
    for push_time in 0..time {
        let speed = push_time;
        let d = (time - push_time) * speed;
        if d > distance {
            ways += 1;
        }
    }
    Ok(ways)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(&input).unwrap(), 140220)
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(&input).unwrap(), 39570185)
    }
}
