use crate::bingo_game::BingoGame;

pub mod bingo_board;
pub mod bingo_game;

pub fn day_04(input: &Vec<String>) {
    println!("Day 04");
    println!("\tPart 1: {}", part_one(&input));
    println!("\tPart 2: {}", part_two(&input));
}

pub fn part_one(input: &Vec<String>) -> usize {
    let mut bingo = BingoGame::new(input);
    match bingo.play() {
        Some(score) => score,
        None => 0,
    }
}

pub fn part_two(_input: &Vec<String>) -> usize {
    2
}
