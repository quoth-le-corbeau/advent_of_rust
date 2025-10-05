mod aoc_2020;

fn twenty_twenty () {
    println!("-------------");
    println!("2020");
    println!("-------------");
    let day_1_p1 = aoc_2020::day_1::solution::part_one();
    println!("Day 1 Part 1: {}", day_1_p1);
    let day_1_p2 = aoc_2020::day_1::solution::part_two();
    println!("Day 1 Part 2: {}", day_1_p2);

    println!("-------------");

}

fn main() {
  twenty_twenty();
}
