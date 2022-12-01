use std::fs;

fn main() {
    
    let input = fs::read_to_string("W:\\Workspace\\Code\\aoc2022\\day1\\src\\input.txt").expect("Unable to read file");
    
    let mut list: Vec<i32> = Vec::new();

    let mut weight: i32 = 0;

    for line in input.lines() {

        if (!line.is_empty()) {
            weight += line.parse::<i32>().unwrap();
        } else {
            list.push(weight);
            weight=0;
        }
    }

    println!("{:?}", list);

    list.sort_by(|a, b| b.cmp(a));
    
    println!("{}", list[0]);
    
    let sum = list[0] + list[1] + list[2];
    println!("{}", sum);

}
