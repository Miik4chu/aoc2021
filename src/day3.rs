pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x: &str| x.chars().map(|f: char| f.to_digit(2).unwrap()).collect())
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let numbers: Vec<Vec<u32>> = parse_input(input);
    let mut gamma_bits: Vec<u32> = Vec::new();
    let mut epsilon_bits: Vec<u32> = Vec::new();

    let length = numbers.first().unwrap().len();

    for i in 0..length {
        let (zero_count, one_count) = get_count(&numbers, i);
        gamma_bits.push((zero_count > one_count) as u32);
        epsilon_bits.push((zero_count < one_count) as u32);
    }

    convert_bits_to_int(gamma_bits) * convert_bits_to_int(epsilon_bits)
}

pub fn part2(input: &str) -> u32 {
    let numbers: Vec<Vec<u32>> = parse_input(input);
    let length = numbers.first().unwrap().len();

    let mut oxygen_ratings: Vec<Vec<u32>> = numbers.clone();
    let mut co2_ratings: Vec<Vec<u32>> = numbers;

    for i in 0..length {
        let (zero_count, one_count) = get_count(&oxygen_ratings, i);
        if oxygen_ratings.len() > 1 {
            oxygen_ratings.retain(|x: &Vec<u32>| -> bool {
                match x.get(i).unwrap() {
                    0 => zero_count > one_count,
                    1 => one_count >= zero_count,
                    _ => panic!("invalid value"),
                }
            });
        }

        let (zero_count, one_count) = get_count(&co2_ratings, i);
        if co2_ratings.len() > 1 {
            co2_ratings.retain(|x: &Vec<u32>| -> bool {
                match x.get(i).unwrap() {
                    0 => zero_count <= one_count,
                    1 => one_count < zero_count,
                    _ => panic!("invalid value"),
                }
            });
        }
    }

    let oxygen_rating: Vec<u32> = oxygen_ratings.first().unwrap().to_vec();
    let co2_rating: Vec<u32> = co2_ratings.first().unwrap().to_vec();

    convert_bits_to_int(oxygen_rating) * convert_bits_to_int(co2_rating)
}

pub fn get_count(input: &[Vec<u32>], index: usize) -> (usize, usize) {
    let zero_count: usize = input
        .to_owned()
        .into_iter()
        .filter(|line| *line.get(index).unwrap() == 0)
        .count();
    let one_count: usize = input.len() - zero_count;

    (zero_count, one_count)
}

pub fn convert_bits_to_int(bits: Vec<u32>) -> u32 {
    let bits_str: Vec<String> = bits.iter().map(|x: &u32| x.to_string()).collect();

    u32::from_str_radix(bits_str.concat().as_str(), 2).unwrap()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    use super::*;

    #[test]
    fn solve_day_3() {
        assert_eq!(part1(INPUT), 198);
        assert_eq!(part2(INPUT), 230);
    }
}
