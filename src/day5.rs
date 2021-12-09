use std::{collections::HashMap, hash::Hash, num::ParseIntError, ops::Neg, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let v: Vec<Point> = input
            .split(" -> ")
            .map(|f| {
                let pos: Vec<i32> = f.split(',').map(|x| x.parse().unwrap()).collect();

                Point {
                    x: pos[0],
                    y: pos[1],
                }
            })
            .collect();

        Ok(Line { a: v[0], b: v[1] })
    }
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn dx(&self) -> i32 {
        self.b.x - self.a.x
    }

    fn dy(&self) -> i32 {
        self.b.y - self.a.y
    }

    fn is_diagonal(&self) -> bool {
        self.dx().abs() == self.dy().abs()
    }

    fn draw(&self, with_diagonal: bool) -> Vec<Point> {
        let mut v = vec![];

        if self.is_horizontal() || self.is_vertical() {
            for mut x in 0..self.dx().abs() + 1 {
                for mut y in 0..self.dy().abs() + 1 {
                    if self.dy().is_negative() {
                        y = y.neg();
                    }
                    if self.dx().is_negative() {
                        x = x.neg();
                    }

                    let p = Point {
                        x: self.a.x + x,
                        y: self.a.y + y,
                    };
                    v.push(p);
                }
            }
        } else if with_diagonal && self.is_diagonal() {
            for mut x in 0..self.dx().abs() + 1 {
                let mut y = x;
                if self.dy().is_negative() {
                    y = y.neg();
                }

                if self.dx().is_negative() {
                    x = x.neg();
                }

                let p = Point {
                    x: self.a.x + x,
                    y: self.a.y + y,
                };
                v.push(p);
            }
        }

        v
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    let mut map: HashMap<Point, i32> = HashMap::with_capacity(lines.len());

    for line in lines {
        for point in line.draw(false) {
            if map.get(&point).is_none() {
                map.insert(point, 1);
            } else {
                map.insert(point, map.get(&point).unwrap() + 1);
            }
        }
    }

    map.values().filter(|v| **v >= 2).count()
}

pub fn part2(input: &str) -> usize {
    let lines = parse_input(input);
    let mut map: HashMap<Point, i32> = HashMap::with_capacity(lines.len());

    for line in lines {
        for point in line.draw(true) {
            if map.get(&point).is_none() {
                map.insert(point, 1);
            } else {
                map.insert(point, map.get(&point).unwrap() + 1);
            }
        }
    }

    map.values().filter(|v| **v >= 2).count()
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
pub mod tests {
    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    use super::*;

    #[test]
    fn solve_day_5() {
        assert_eq!(part1(INPUT), 5);
        assert_eq!(part2(INPUT), 12);
    }
}
