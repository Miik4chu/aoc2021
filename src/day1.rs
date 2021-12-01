pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|depth| depth.parse::<u32>().ok())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let depths: Vec<u32> = parse_input(input);
    depths.windows(2).filter(|x| x[0] < x[1]).count()
}

pub fn part2(input: &str) -> usize {
    let depths: Vec<u32> = parse_input(input);
    depths
        .windows(4)
        .filter(|x| (x[0] + x[1] + x[2]) < (x[1] + x[2] + x[3]))
        .count()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    use super::{part1, part2};

    #[test]
    fn solve_day_1() {
        assert_eq!(part1(INPUT), 7);
        assert_eq!(part2(INPUT), 5);
    }
}
