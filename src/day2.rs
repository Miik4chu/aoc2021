pub fn parse_input(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line: &str| {
            let data: Vec<&str> = line.split_whitespace().collect();
            (data[0], data[1].parse::<u32>().unwrap())
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let lines: Vec<(&str, u32)> = parse_input(input);

    for line in lines {
        match line {
            ("up", value) => y -= value,
            ("down", value) => y += value,
            ("forward", value) => x += value,
            _ => println!("Invalid line"),
        }
    }

    x * y
}

pub fn part2(input: &str) -> u32 {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut aim: u32 = 0;

    let lines: Vec<(&str, u32)> = parse_input(input);

    for line in lines {
        match line {
            ("up", value) => aim -= value,
            ("down", value) => aim += value,
            ("forward", value) => {
                x += value;
                y += value * aim;
            }
            _ => println!("Invalid line"),
        }
    }

    x * y
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    use super::{part1, part2};

    #[test]
    fn solve_day_2() {
        assert_eq!(part1(&INPUT), 150);
        assert_eq!(part2(&INPUT), 900);
    }
}
