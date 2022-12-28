fn main() {
    let mut max: u64 = 0;
    for group in include_str!("../input.txt").split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            let value: u64 = line.parse().unwrap();
            sum += value;
        }
        if sum > max {
            max = sum
        }
    }
    println!("{max}")
}
