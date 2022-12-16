use std::{
    fs::File,
    io::{self, BufRead}, collections::BinaryHeap,
};

pub fn calculate_highest_calories(path: &str) -> String {
    let lines = io::BufReader::new(File::open(path).expect("Could not open file")).lines();

    let mut max: u32 = 0;
    let mut current_count = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            if current_count > max {
                max = current_count;
            }
            current_count = 0;
        } else {
            current_count += line.parse::<u32>().unwrap()
        }
    }

    max.to_string()
}

pub fn calculate_highest_3_calories(path: &str) -> String {
    let lines = io::BufReader::new(File::open(path).expect("Could not open file")).lines();

    let mut heap = BinaryHeap::new();
    let mut current_count = 0;
    for line in lines.flatten() {
        if line.is_empty() {
            heap.push(current_count);
            current_count = 0;
        } else {
            current_count += line.parse::<u32>().unwrap()
        }
    }

    let mut max_3 = Vec::new();
    for _ in 0..3 {
        max_3.push(heap.pop().unwrap())
    }
    max_3.iter().sum::<u32>().to_string()
}