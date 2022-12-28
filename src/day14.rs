use std::fs;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use crate::day10::knot_hash;

pub(crate) fn day14() {
    let input = fs::read_to_string("input/day14/input.txt").unwrap();
    println!("{}", count_used(input.trim()));
    // println!("{}", part_b);
}

fn count_used(input: &str) -> usize {
    (0..128).par_bridge().map(|i| {
        let hash = knot_hash(format!("{}-{}", input, i).as_str());
        hash.chars().into_iter().tuples()
            .map(|(a, b)| u8::from_str_radix(format!("{}{}", a, b).as_str(), 16).unwrap())
            .map(|mut num| {
                let mut ones = 0;
                for _ in 0..8 {
                    if num & 1 == 1 { ones += 1; }
                    num = num >> 1;
                }
                ones
            })
            .sum::<usize>()
    }).sum()
}

#[cfg(test)]
mod day14_tests {
    use std::fs;
    use crate::day14::count_used;

    #[test]
    fn test_works() {
        assert_eq!(8108, count_used("flqrgnkx"));
    }

    #[test]
    fn input_works() {
        let input = fs::read_to_string("input/day14/input.txt").unwrap();
        assert_eq!(8250, count_used(input.trim()));
    }
}
