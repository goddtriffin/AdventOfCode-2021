use day_one::day_one;
use day_three::day_three;
use day_two::day_two;
use utils::lines_from_file;

fn main() {
    day_one(&lines_from_file("d1.txt").unwrap());
    day_two(&lines_from_file("d2.txt").unwrap());
    day_three(&lines_from_file("d3.txt").unwrap());
}
