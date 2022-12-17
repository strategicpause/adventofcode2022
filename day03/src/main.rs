use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(157, part_a("sample.txt"));
    assert_eq!(70, part_b("sample.txt"));

    println!("Part A: {}", part_a("input.txt"));
    println!("Part B: {}", part_b("input.txt"));

    Ok(())
}

fn part_a(file: &str) -> u32 {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|line| line.unwrap())
        .map(|rucksacks| {
            let rucksack_size = rucksacks.len() / 2;

            let (left, right) = rucksacks.split_at(rucksack_size);
            let unique_supplies: HashSet<_> = left.bytes().collect();

            right.bytes()
                .find(|c| unique_supplies.contains(&c))
                .map_or(0, |c| if c.is_ascii_lowercase() { c - 96 } else { c - 38 }) as u32
        })
        .sum::<u32>()
}

fn part_b(file: &str) -> u32 {
    let input_file = File::open(file).unwrap(); // Returns File
    let reader = BufReader::new(input_file); // Returns BufReader<&File>
    reader.lines() // Returns Lines<Result<String, std::io::Error>>
        .map(|line| line.unwrap()) // Returns String (panics otherwise)
        .collect::<Vec<String>>() // Returns Vec<String>
        .chunks(3) // Returns Vec<Vec<String>>
        .map(|rucksacks| {
            let first: HashSet<u8> = rucksacks[0].bytes().collect();
            let second: HashSet<u8> = rucksacks[1].bytes().collect();
            let third: HashSet<u8> = rucksacks[2].bytes().collect();

            first.intersection(&second)
                .find(|c| third.contains(c))
                .map_or(0, |c| if c.is_ascii_lowercase() { c - 96 } else { c - 38 }) as u32
        })
        .sum::<u32>()
}