use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(2, part_a("sample.txt"));
    assert_eq!(4, part_b("sample.txt"));

    println!("Part A: {}", part_a("input.txt"));
    println!("Part B: {}", part_b("input.txt"));

    Ok(())
}

fn part_a(file: &str) -> u32 {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|assignment_pairs| {
            let (left, right) = assignment_pairs.split_once(",").unwrap();
            let (a, b) = left.split_once("-").unwrap();
            let (c, d) = right.split_once("-").unwrap();

            (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap(), c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap())
        })
        .filter(|(a,b,c,d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count() as u32
}

fn part_b(file: &str) -> u32 {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|assignment_pairs| {
            let (left, right) = assignment_pairs.split_once(",").unwrap();
            let (a, b) = left.split_once("-").unwrap();
            let (c, d) = right.split_once("-").unwrap();

            (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap(), c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap())
        })
        .filter(|(a,b,c,d)| (a <= c && b >= c) || (a <= d && b >= d) || (a >= c && b <= d) || (a <= c && b >= d))
        .count() as u32
}
