use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    let mut lanturns = VecDeque::from(lines);

    for _ in 0..80 {
        lanturns.rotate_left(1);
        if lanturns[8] != 0 {
            lanturns[6] += lanturns[8];
        }
    }

    lanturns.iter().sum()
}

pub fn part2(input: &str) -> usize {
    let lines = parse_input(input);
    let mut lanturns = VecDeque::from(lines);

    for _ in 0..256 {
        lanturns.rotate_left(1);
        if lanturns[8] != 0 {
            lanturns[6] += lanturns[8];
        }
    }

    lanturns.iter().sum()
}

pub fn parse_input(input: &str) -> [usize; 9] {
    let lines: Vec<usize> = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let mut count = [0; 9];
    count.fill(0);

    for line in lines {
        count[line] += 1;
    }

    count
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "3,4,3,1,2

";
    use super::*;

    #[test]
    fn solve_day_6() {
        assert_eq!(part1(INPUT), 5934);
        assert_eq!(part2(INPUT), 26984457539);
    }
}
