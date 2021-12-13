use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Signal {
    input: Vec<String>,
    output: Vec<String>,
}

pub fn part1(input: &str) -> usize {
    let signals = parse_input(input);

    signals
        .iter()
        .map(|signal| -> usize {
            signal
                .output
                .iter()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    // 0 = 6, 1 = 2, 2 = 5, 3 = 5, 4 = 4, 5 = 5, 6 = 6, 7 = 2, 8 = 7, 9 = 6
    let input = parse_input(input);
    input
        .iter()
        .map(|signal| -> u32 {
            // unique segments
            let one = find_digit_by_len(&signal.input, 2);
            let four = find_digit_by_len(&signal.input, 4);
            let seven = find_digit_by_len(&signal.input, 3);
            let eight = find_digit_by_len(&signal.input, 7);

            // segment length 6
            let six = find_digit(&signal.input, 6, &one, 1, None);
            let nine = find_digit(&signal.input, 6, &four, 4, None);
            let zero = find_digit(&signal.input, 6, &four, 3, Some(&six));

            // segment length 5
            let three = find_digit(&signal.input, 5, &one, 2, None);
            let two = find_digit(&signal.input, 5, &four, 2, None);
            let five = find_digit(&signal.input, 5, &six, 5, None);

            let mut mappings: HashMap<Vec<char>, u32> = HashMap::new();
            mappings.insert(zero, 0);
            mappings.insert(one, 1);
            mappings.insert(two, 2);
            mappings.insert(three, 3);
            mappings.insert(four, 4);
            mappings.insert(five, 5);
            mappings.insert(six, 6);
            mappings.insert(seven, 7);
            mappings.insert(eight, 8);
            mappings.insert(nine, 9);

            let output: Vec<u32> = signal
                .output
                .iter()
                .map(|x| -> u32 {
                    let mut digit: Vec<char> = x.chars().collect();
                    digit.sort_unstable();

                    *mappings.get(&digit).unwrap()
                })
                .collect();

            String::from_iter(output.iter().map(|x| x.to_string()))
                .parse()
                .unwrap()
        })
        .sum()
}

pub fn str_to_hashset(str: &str) -> HashSet<char> {
    let mut s: Vec<char> = str.chars().collect();
    s.sort_unstable();

    HashSet::from_iter(s)
}

pub fn find_digit_by_len(input: &[String], len: usize) -> Vec<char> {
    let mut d: Vec<char> = input
        .iter()
        .find(|x| x.len() == len)
        .unwrap()
        .chars()
        .collect();
    d.sort_unstable();

    d
}

pub fn find_digit(
    input: &[String],
    len: usize,
    mask: &[char],
    count: usize,
    exclude: Option<&[char]>,
) -> Vec<char> {
    let digit = input
        .iter()
        .filter(|x| x.len() == len)
        .map(|x| str_to_hashset(x))
        .find(|x| {
            let mut cond = true;
            if let Some(value) = exclude {
                let h: HashSet<char> = HashSet::from_iter(value.iter().copied());
                cond = &h != x;
            }

            cond && x
                .intersection(&HashSet::from_iter(mask.iter().copied()))
                .count()
                == count
        })
        .unwrap();

    let mut v: Vec<char> = digit.iter().copied().collect();
    v.sort_unstable();

    v
}

pub fn parse_input(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|line| {
            let mut l = line
                .split('|')
                .map(|l| l.split_whitespace().map(|x| x.to_string()).collect());
            Signal {
                input: l.next().unwrap(),
                output: l.next().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    use super::*;

    #[test]
    fn solve_part1() {
        assert_eq!(part1(INPUT), 26);
    }

    #[test]
    fn solve_part2_exemple() {
        assert_eq!(part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
    }

    #[test]
    fn solve_part2_exemple2() {
        assert_eq!(part2("cg fadegbc ecfadb acdbeg abgfe dcegfb gcad bceag debca bgc | ceafbd gfedcb cabedf dbace"), 6065);
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(INPUT), 61229);
    }
}
