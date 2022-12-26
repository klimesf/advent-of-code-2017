extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
use crate::day01::day01;

mod day01;
mod toolbox;

macro_rules! measure {
    ($s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}", format!("Elapsed: {:?}", timer.elapsed()).italic().dimmed());
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!();
    println!("{}", format!("Advent of Code").red());
    println!("{}", format!("        //2017").red());
    println!();

    if args.contains(&"all".to_string()) || args.contains(&"day01".to_string()) || args.contains(&"latest".to_string()) {
        println!("{}", format!("--- day01:").underline().green());
        measure!(day01());
    }
}
