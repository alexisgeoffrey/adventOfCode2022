mod day_1;

fn main() {
    println!("Day 1:");
    println!(
        "Highest calorie count: {}\nSum of highest three calorie counts: {}",
        day_1::calculate_highest_calories(&path(1)),
        day_1::calculate_highest_3_calories(&path(1))
    );
    println!();
}

fn path(day: u8) -> String {
    format!("src/day_{day}/day_{day}.txt")
}