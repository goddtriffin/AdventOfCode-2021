use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
use utils::io::lines_from_file;

fn main() {
    day_01(&lines_from_file("d1.txt").unwrap());
    day_02(&lines_from_file("d2.txt").unwrap());
    day_03(&lines_from_file("d3.txt").unwrap());
    day_04(&lines_from_file("d4.txt").unwrap());
}
