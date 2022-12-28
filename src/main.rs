use std::collections::BinaryHeap;

fn main() {
    let input: &str = include_str!("../input.txt");
    println!("Part 1 Solution: {}", part1(input));
    println!("Part 2 Solution: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut max: u64 = 0;
    for group in input.split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            let value: u64 = line.parse().unwrap();
            sum += value;
        }
        if sum > max {
            max = sum
        }
    }
    max
}

fn part2(input: &str) -> u64 {
    let mut heap = BinaryHeap::<u64>::new();
    let mut current_group: u64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            heap.push(current_group);
            current_group = 0;
            continue;
        }
        current_group += line.parse::<u64>().unwrap();
    }

    let mut solution: u64 = 0;
    for _ in 0..3 {
        let val = heap.pop().unwrap();
        solution += val;
    }
    solution
}
