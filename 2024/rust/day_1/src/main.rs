use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let input = read_lines("input.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for i in input {
        let line: Vec<&str> = i.split_whitespace().collect();
        println!("{:?}", line);
        let (l, r): (i32, i32) = (line[0].parse().unwrap(), line[1].parse().unwrap());
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();
    let mut sum = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        sum += (l-r).abs();
    }
    println!("Part 1: {}", sum);
}
