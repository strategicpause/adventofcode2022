use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(7, part_a("sample.txt"));
    assert_eq!(19, part_b("sample.txt"));

    println!("Part A: {}", part_a("input.txt"));
    println!("Part B: {}", part_b("input.txt"));

    Ok(())
}

fn part_a(file: &str) -> u32 {
    get_marker(file, 4)
}

fn part_b(file: &str) -> u32 {
    get_marker(file, 14)
}


fn get_marker(file: &str, marker_len: usize) -> u32 {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    let line = reader.lines().next().unwrap().unwrap();

    let mut char_vec: Vec<char> = vec!('\u{0000}'; marker_len);
    let mut it = 0;
    for c in line.chars() {
        if it >= marker_len && char_vec.iter().collect::<HashSet<&char>>().len() == marker_len {
            return it as u32;
        }
        char_vec[it % marker_len] = c;
        it = it + 1;
    }

    0
}
