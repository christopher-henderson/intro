mod hack;
mod par;
mod predicates;
mod views;

use crate::predicates::Direction;
use crate::views::Matrix;
use rand::seq::SliceRandom;
use std::time::Instant;

fn solve(puzzle: Vec<Vec<char>>, target: impl AsRef<str>) -> u64 {
    let target = target.as_ref();
    let matrix = Matrix::from(puzzle);
    let first = *target.chars().collect::<Vec<char>>().first().unwrap();
    let direction = Direction::new(matrix.height(), matrix.width(), target.len());
    let mut count: u64 = 0;
    for row in 0..matrix.height() {
        for column in 0..matrix.width() {
            if !matrix.candidate(row, column, first) {
                continue;
            }
            if direction.north(row) && matrix.north(row, column, target.len()).eq(target.chars()) {
                count += 1;
            }
            if direction.south(row) && matrix.south(row, column, target.len()).eq(target.chars()) {
                count += 1;
            }
            if direction.east(column) && matrix.east(row, column, target.len()).eq(target.chars()) {
                count += 1;
            }
            if direction.west(column) && matrix.west(row, column, target.len()).eq(target.chars()) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    large();
    // let input = vec![
    //     // n == 4
    //     // height = 5
    //     // width = 5
    //     //    0    1    2    3    4
    //     vec!['a', 'l', 'a', 'n', 'a'], // 0
    //     vec!['a', 'l', 'l', 'n', 'l'], // 1
    //     vec!['a', 'l', 'a', 'n', 'a'], // 2
    //     vec!['a', 'l', 'n', 'n', 'n'], // 3
    //     vec!['a', 'l', 'a', 'n', 'p'], // 4
    // ];
    // let got = solve(input.clone(), "alan");
    // println!("{got}");
    // println!("{}", par::par_solve(input, "alan"));
    // let input: Vec<Vec<char>> = serde_json::from_reader(
    //     std::fs::File::open("/home/chris/projects/intro/ALAN.json").unwrap(),
    // )
    // .unwrap();
    // println!("go");
    // let i = input.clone();
    // let start = Instant::now();
    // let got = solve(i, "ALAN");
    // println!(
    //     "Serial got {got} in {:?}",
    //     Instant::now().duration_since(start)
    // );
    // let start = Instant::now();
    // let got = par::par_solve(input, "ALAN");
    // println!(
    //     "Parallel got {got} in {:?}",
    //     Instant::now().duration_since(start)
    // );
}

fn large() {
    println!("Loading...");
    let b = std::fs::read("/home/chris/projects/intro/alan/ALAN.json").unwrap();
    let input: Vec<Vec<char>> = serde_json::from_slice(&b).unwrap();
    println!("Loaded");
    let start = Instant::now();
    let got = par::solve(input, "ALAN");
    let took = Instant::now().duration_since(start);
    println!("Got {got} after {took:?}");
}

fn generate_data_set(height: usize, width: usize) {
    let dataset = ['A', 'L', 'A', 'N'];
    let mut matrix = Vec::with_capacity(height);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width {
            row.push(dataset.choose(&mut rand::thread_rng()));
        }
        matrix.push(row);
    }
    let serialzied = serde_json::to_string_pretty(&matrix).unwrap();
    std::fs::write("/home/chris/projects/intro/alan/ALAN.json", serialzied).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen() {
        generate_data_set(10000, 10000);
    }

    #[test]
    fn a() {
        struct Test {
            input: Vec<Vec<char>>,
            target: &'static str,
            want: u64,
        }
        let data = vec![
            Test {
                input: vec![],
                target: "alan",
                want: 0,
            },
            Test {
                input: vec![vec![]],
                target: "alan",
                want: 0,
            },
            Test {
                input: vec![vec!['a']],
                target: "alan",
                want: 0,
            },
            Test {
                input: vec![vec!['a', 'l', 'a', 'n']],
                target: "alan",
                want: 1,
            },
            Test {
                input: vec![vec!['a', 'l', 'a', 'n', 'z', 'o']],
                target: "alan",
                want: 1,
            },
            Test {
                input: vec![vec!['z', 'o', 'a', 'l', 'a', 'n']],
                target: "alan",
                want: 1,
            },
            Test {
                input: vec![vec!['z', 'o', 'a', 'l', 'a', 'n', 'z', 'o']],
                target: "alan",
                want: 1,
            },
            Test {
                input: vec![
                    //
                    vec!['a', 'l', 'a', 'n'],
                    vec!['a', 'l', 'a', 'n'],
                ],
                target: "alan",
                want: 2,
            },
            Test {
                input: vec![
                    //
                    vec!['a', 'l', 'a', 'n'],
                    vec!['l', 'l', 'l', 'n'],
                    vec!['a', 'l', 'l', 'n'],
                    vec!['n', 'l', 'l', 'n'],
                ],
                target: "alan",
                want: 2,
            },
            Test {
                input: vec![
                    //
                    vec!['a', 'l', 'a', 'n'],
                    vec!['l', 'l', 'l', 'n'],
                    vec!['a', 'l', 'a', 'n'],
                    vec!['n', 'l', 'l', 'n'],
                ],
                target: "alan",
                want: 3,
            },
            Test {
                input: vec![
                    //
                    vec!['a', 'l', 'a', 'n'],
                    vec!['l', 'l', 'l', 'n'],
                    vec!['a', 'l', 'a', 'n'],
                    vec!['n', 'l', 'n', 'n'],
                ],
                target: "alan",
                want: 4,
            },
            Test {
                input: vec![
                    //
                    vec!['a', 'l', 'a', 'n'],
                    vec!['l', 'l', 'l', 'n'],
                    vec!['a', 'l', 'a', 'n'],
                    vec!['n', 'l', 'n', 'n'],
                ],
                target: "ala",
                want: 4,
            },
            Test {
                input: vec![
                    //
                    vec!['a'],
                ],
                target: "a",
                want: 1,
            },
        ];
        for test in data {
            let got = solve(test.input, test.target);
            assert_eq!(got, test.want);
        }
    }

    #[test]
    fn isolate() {
        let got = solve(vec![vec!['a']], "a");
        assert_eq!(got, 1);
    }
}
