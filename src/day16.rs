use std::collections::VecDeque;
use std::fs;

use itertools::Itertools;

pub(crate) fn day16() {
    let input = fs::read_to_string("input/day16/input.txt").unwrap();
    let moves: Vec<&str> = input.trim().split(",").collect();
    let mut ans = dance("abcdefghijklmnop", &moves);
    println!("{}", ans);
    for i in 1..1000000000 {
        ans = dance(ans.as_str(), &moves);
        if i % 10000 == 0 {
            println!("{}", i);
        }
    }
    println!("{}", ans);
}

enum DanceMove {
    S(usize),
    X(usize, usize),
    P(char, char),
}

fn parse(input: &str) -> Vec<DanceMove> {
    input.trim().split(",")
        .map(|m| match m.chars().nth(0).unwrap() {
            's' => {
                let i = m[1..m.len()].parse::<usize>().unwrap();
                DanceMove::S(i)
            }
            'x' => {
                let (from, to) = m[1..m.len()].split_once("/").unwrap();
                DanceMove::X(from.parse().unwrap(), to.parse().unwrap())
            }
            'p' => {
                let first = m.chars().nth(1).unwrap();
                let second = m.chars().nth(3).unwrap();
                DanceMove::P(first, second)
            }
            c => panic!("Unknown move {}", c)
        })
        .collect()
}

fn dance(programs: &str, moves: &Vec<&str>) -> String {
    let mut programs: VecDeque<char> = programs.chars().collect();
    for m in moves {
        match m.chars().nth(0).unwrap() {
            's' => {
                let i = m[1..m.len()].parse::<i32>().unwrap();
                for _ in 0..i {
                    let temp = programs.pop_back().unwrap();
                    programs.push_front(temp);
                }
            }
            'x' => {
                let (from, to) = m[1..m.len()].split_once("/").unwrap();
                programs.swap(from.parse().unwrap(), to.parse().unwrap());
            }
            'p' => {
                let first = m.chars().nth(1).unwrap();
                let second = m.chars().nth(3).unwrap();
                let pos_first = programs.iter().position(|c| *c == first).unwrap();
                let pos_second = programs.iter().position(|c| *c == second).unwrap();
                programs.swap(pos_first, pos_second);
            }
            c => panic!("Unknown move {}", c)
        }
    }

    programs.iter().join("")
}

#[cfg(test)]
mod day16_tests {
    use std::fs;

    use crate::day16::dance;

    #[test]
    fn test_works() {
        let input = fs::read_to_string("input/day16/test.txt").unwrap();
        let moves: Vec<&str> = input.trim().split(",").collect();
        assert_eq!("baedc", dance("abcde", &moves));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/day16/input.txt").unwrap();
        let moves: Vec<&str> = input.trim().split(",").collect();
        assert_eq!("ebjpfdgmihonackl", dance("abcdefghijklmnop", &moves));
    }
}
