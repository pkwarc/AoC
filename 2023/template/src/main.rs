fn main() {
    let input = include_str!("../input.txt");
    println!("{}", part1(&input));
}

fn part1(input: &str) -> String {
    String::new()
}

fn part2(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(&input), "")
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(&input), "")
    }
}
