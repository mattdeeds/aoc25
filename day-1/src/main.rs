fn main() {
    println!("Hello, AoC Day 1!");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let moves: Vec<&str> = input.split('\n').collect();
    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for a_move in moves {
        let mut chars = a_move.chars();
        let dir = chars.next().unwrap();
        let steps: String = chars.collect();

        let value: i32 = match dir {
            'R' => {
                steps.parse::<i32>().unwrap()
            },
            'L' => {
                -steps.parse::<i32>().unwrap()
            }
            _ => {
                panic!("ERROR: Invalid Direction!")
            },
        };

        println!("Moving: {}", value);
        for _ in 0..value.abs() {
            if value > 0 {
                // Clockwise
                if dial == 99 {
                    dial = 0;
                } else {
                    dial += 1;
                }
            } else {
                // Counter-Clockwise
                if dial == 0 {
                    dial = 99;
                } else {
                    dial -= 1;
                }
            }
            if dial == 0 {
                zero_count += 1;
            }
        }
        
        println!("Result: {}", dial);
    } 
    println!("==========================");
    println!("Final Count: {}", zero_count)
}