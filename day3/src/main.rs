use std::{collections::HashSet, hash, slice::Chunks};

fn main() {
    let lines = include_str!("input.txt");

    part1(lines);
    part2(lines);

}

fn part2(lines: &str) {
    for chunk in lines.lines().collect::<Vec<&str>>().chunks(3) {
        println!("{:?}", chunk)
    }

}


fn part1(lines: &str) {
    let mut test: Vec<HashSet<char>> = Vec::new();

    
    for line in lines.lines() {
        let mid = line.len() / 2;
        let (firstCompartment, secondCompartment) = line.split_at(mid);

        let mut hashSet:HashSet<char> = HashSet::new();

        for (i, c) in firstCompartment.chars().enumerate() {
            if (secondCompartment.contains(c)) {
                hashSet.insert(c);
            }
        }
        test.push(hashSet);
    }

    // Create function to convert char to ascii

    let mut count : u64 = 0;
    for i in test {
        for j in i {
            let mut ascii = j as u64;

            if ascii >= 65 && ascii <= 90 {
                ascii -= 38
            } else if ascii >= 97 && ascii <= 122 {
                ascii -= 96
            }

            count += ascii;
        }
    }

    println!("{}", count);
}