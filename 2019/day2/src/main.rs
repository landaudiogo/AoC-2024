use std::{fs::File, io::BufReader};

mod part1;
mod part2;

fn main() {
    part1::run(BufReader::new(File::open("input").unwrap()));
}
