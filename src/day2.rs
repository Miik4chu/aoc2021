pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let lines = parse_input(input).into_iter();
    for line in lines {
        let mut iter = line.split_whitespace();
        match iter.next().unwrap() {
            "up" => y -= iter.next().unwrap().parse::<i32>().unwrap(),
            "down" => y += iter.next().unwrap().parse::<i32>().unwrap(),
            "forward" => x += iter.next().unwrap().parse::<i32>().unwrap(),
            _ => {}
        }
    }

    x * y
}

pub fn part2(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    let lines = parse_input(input).into_iter();
    for line in lines {
        let mut iter = line.split_whitespace();
        match iter.next().unwrap() {
            "up" => aim -= iter.next().unwrap().parse::<i32>().unwrap(),
            "down" => aim += iter.next().unwrap().parse::<i32>().unwrap(),
            "forward" => {
                let n: i32 = iter.next().unwrap().parse::<i32>().unwrap();
                x += n;
                y += n * aim;
            }
            _ => {}
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
