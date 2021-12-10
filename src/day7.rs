use std::cmp;

pub fn part1(input: &str) -> i32 {
    let mut lines = parse_input(input);
    let median: i32;
    lines.sort_unstable();
    if lines.len() % 2 == 0 {
        median = *lines.get(lines.len() / 2).unwrap();
    } else {
        let rounded_len: usize = (lines.len() as f32 / 2_f32).round() as usize;
        median = *lines.get(rounded_len).unwrap();
    }

    lines.iter().map(|x| (median - x).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let lines = parse_input(input);
    let mean: f32 = lines.iter().sum::<i32>() as f32 / lines.len() as f32;
    let ceil_mean: i32 = mean.ceil() as i32;
    let floor_mean: i32 = mean.floor() as i32;

    let ceil_sum = lines
        .iter()
        .map(|x| -> i32 { (0..=(ceil_mean - x).abs()).sum() })
        .sum();
    if ceil_mean == floor_mean {
        ceil_sum
    } else {
        let floor_sum = lines
            .iter()
            .map(|x| -> i32 { (0..=(floor_mean - x).abs()).sum() })
            .sum();

        cmp::min(ceil_sum, floor_sum)
    }
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14
";
    use super::*;

    #[test]
    fn solve_day_7() {
        assert_eq!(part1(INPUT), 37);
        assert_eq!(part2(INPUT), 168);
    }
}
