use crate::bingo_game::BingoGame;

pub mod bingo_board;
pub mod bingo_game;

pub fn day_04(input: &Vec<String>) {
    let mut bingo = BingoGame::new(input);

    println!("Day 04");
    println!("\tPart 1: {}", part_one(&mut bingo));
    bingo.reset_boards();
    println!("\tPart 2: {}", part_two(&mut bingo));
}

pub fn part_one(bingo: &mut BingoGame) -> usize {
    match bingo.play_to_win() {
        Some(score) => score,
        None => 0,
    }
}

pub fn part_two(bingo: &mut BingoGame) -> usize {
    match bingo.play_to_lose() {
        Some(score) => score,
        None => 0,
    }
}
