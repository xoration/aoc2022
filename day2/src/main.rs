/*
A Rock
B Paper
C Scissors

X Rock      1
Y Paper     2
Z Scissors  3

win 6 draw 3 loss 0

X need to lose
Y need to draw
Z need to win
*/

use std::{collections::HashMap, str::Lines};



fn main() {
    let text = include_str!("input.txt");
    let lines = text.lines();

    part1(lines.clone());
    part2(lines.clone());
}

fn part2(lines: Lines){
    let mut points= 0;
    let handPoints = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    let rulesWin = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B'),
    ]);

    let rulesLoss = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A'),
    ]);

    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let result = line.chars().nth(2).unwrap();

        match result {
            'X' => {
                let lossHand = rulesWin.get(&opponent).unwrap();
                points += *handPoints.get(lossHand).unwrap();
            }, 
            'Z' => {
                let winHand = rulesLoss.get(&opponent).unwrap();
                points += *handPoints.get(winHand).unwrap();
                points += 6;
            },
            'Y' => {
                points += *handPoints.get(&opponent).unwrap();
                points += 3;
            },
            _ => {}
        } 

     
    }

    println!("Points: {}", points);
}


fn part1(lines: Lines)
{
    
    let mut points = 0;

    let handPoints = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);

    let rulesWinners = HashMap::from([
        ('X', 'C'),
        ('Y', 'A'),
        ('Z', 'B'),
    ]);


    let rulesDraw = HashMap::from([
        ('X', 'A'),
        ('Y', 'B'),
        ('Z', 'C'),
    ]);

    

    for line in lines {
        let opponent = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        if *rulesDraw.get(&me).unwrap() == opponent {
            points += 3;
            points += handPoints.get(&me).unwrap();
            continue;
        }

        let loser = rulesWinners.get(&me).unwrap();

        if opponent == *loser {
            points += 6;
            points += handPoints.get(&me).unwrap();
            continue;
        }
        
        points += handPoints.get(&me).unwrap();
    }

    println!("Points: {}", points);
}
