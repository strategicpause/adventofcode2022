use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;


const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";

const RESP_ROCK: &str = "X";
const RESP_PAPER: &str = "Y";
const RESP_SCISSORS: &str = "Z";

const RESP_LOSE: &str = "X";
const RESP_DRAW: &str = "Y";

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

const WIN: i32 = 6;
const DRAW: i32 = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let win_map = HashMap::from([
        (OPP_ROCK, RESP_PAPER),
        (OPP_PAPER, RESP_SCISSORS),
        (OPP_SCISSORS, RESP_ROCK),
    ]);

    let draw_map = HashMap::from([
       (OPP_ROCK, RESP_ROCK),
       (OPP_PAPER, RESP_PAPER),
       (OPP_SCISSORS, RESP_SCISSORS),
   ]);

    let lose_map = HashMap::from([
          (OPP_ROCK, RESP_SCISSORS),
          (OPP_PAPER, RESP_ROCK),
          (OPP_SCISSORS, RESP_PAPER),
      ]);

    let scores = HashMap::from([
        (RESP_ROCK, SCORE_ROCK),
        (RESP_PAPER, SCORE_PAPER),
        (RESP_SCISSORS, SCORE_SCISSORS),
    ]);

    let mut part_a_total_score: i32 = 0;
    let mut part_b_total_score: i32 = 0;
    for l in reader.lines() {
        let line = l?;
        let round = line.split_whitespace().collect::<Vec<&str>>();
        let opponent = &round[0];
        let response = &round[1];
        // Chekck if draw
        if draw_map.get(opponent).unwrap() == response {
            part_a_total_score += DRAW;
        } else if win_map.get(opponent).unwrap() == response { // Win
            part_a_total_score += WIN;
        }

        let response_play: &str;
        if response == &RESP_LOSE {
            response_play = lose_map.get(opponent).unwrap();
        } else if response == &RESP_DRAW {
            response_play = draw_map.get(opponent).unwrap();
            part_b_total_score += DRAW;
        } else { // Win
            response_play = win_map.get(opponent).unwrap();
            part_b_total_score += WIN;
        }
        part_a_total_score += scores.get(response).unwrap();
        part_b_total_score += scores.get(&response_play).unwrap();
    }
    println!("Part A: {}", part_a_total_score);
    println!("Part B: {}", part_b_total_score);

    Ok(())
}
