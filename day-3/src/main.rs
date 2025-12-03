fn main() {
    println!("Hello, AoC Day 3!");
    println!("-------------------");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let banks: Vec<&str> = input.split('\n').collect();
    let mut sum: usize = 0;

    // Part 1
    time_it!({
        for bank in &banks {
            let batteries: Vec<char> = bank.chars().collect();
            let mut highest = 0;
            let mut highest_index = 0;
            let bank_len = batteries.len();
            let mut index = 0;
            for battery in &batteries {
                let num: usize = battery.to_digit(10).unwrap() as usize;
                if num > highest && index < bank_len - 1 {
                    highest = num;
                    highest_index = index;
                }
                index += 1;
            }

            let mut second_highest = 0;
            for i in highest_index + 1..bank_len {
                let num: usize = batteries[i].to_digit(10).unwrap() as usize;
                if num > second_highest {
                    second_highest = num;
                }
            }
            let combined = format!("{}{}", highest, second_highest);
            let combined_num: usize = combined.parse().unwrap();
            // println!("{}", combined_num);
            sum += combined_num;
        }
        println!("Part 1 Final Sum: {}", sum);
        sum = 0;
    });

    // Part 2
    time_it!({
        for bank in banks {
            let batteries: Vec<char> = bank.chars().collect();
            let battery_max = 12;
            let mut sorted = Vec::new();

            let mut highest = 0;
            let mut highest_index = 0;
            let bank_len = batteries.len();
    
            // Find Joltage Value
            for b in 0..bank_len-12 {
                let num: usize = batteries[b].to_digit(10).unwrap() as usize;
                if num > highest {
                    highest = num;
                    highest_index = b;
                }
            }
            sorted.push(highest);
            
            let mut remaining_battery_count = bank_len - highest_index - 1;
            let mut selected_battery_count = sorted.len();

            while selected_battery_count < battery_max && remaining_battery_count > battery_max - selected_battery_count {
                let mut next_highest = 0;
                let mut next_highest_index = 0;
                for i in highest_index + 1..bank_len {
                    let num: usize = batteries[i].to_digit(10).unwrap() as usize;
                    if num > next_highest && i <= bank_len - (battery_max - selected_battery_count) {
                        next_highest = num;
                        next_highest_index = i;
                    }
                }

                sorted.push(next_highest);
                highest_index = next_highest_index;
                remaining_battery_count = bank_len - highest_index - 1;
                selected_battery_count = sorted.len();

                if remaining_battery_count == battery_max - selected_battery_count {
                    for j in highest_index + 1..bank_len {
                        let num: usize = batteries[j].to_digit(10).unwrap() as usize;
                        sorted.push(num);
                    }
                    break;
                }
            }
            
            let combined: String = sorted.iter().map(|n| n.to_string()).collect();
            let combined_num: usize = combined.parse().unwrap();
            sum += combined_num;
        }
        println!("Part 2 Final Sum: {}", sum);
    });
}

#[macro_export]
macro_rules! time_it {
    ($code:block) => {{
        use std::time::Instant;
        let start = Instant::now();
        $code
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);
    }};
}