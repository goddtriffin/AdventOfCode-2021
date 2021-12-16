use day_01::day_one;
use day_02::day_two;
use day_03::day_three;
use utils::io::lines_from_file;

fn main() {
    day_one(&lines_from_file("d1.txt").unwrap());
    day_two(&lines_from_file("d2.txt").unwrap());
    day_three(&lines_from_file("d3.txt").unwrap());
}
