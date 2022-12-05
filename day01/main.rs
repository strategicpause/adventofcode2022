use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut heap = BinaryHeap::new();

    let mut total_calories: i32 = 0;
    let mut highest_calories: i32 = 0;
    for line in reader.lines() {
        match line?.parse::<i32>() {
          Ok(calories) => total_calories += calories,
          Err(_) => {
            if total_calories > highest_calories {
                highest_calories = total_calories
            }
            heap.push(total_calories);
            total_calories = 0;
          },
        }
    }
    println!("Part A: {}", highest_calories);

    let top_three: i32 = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("Part B: {}", top_three);

    Ok(())
}


