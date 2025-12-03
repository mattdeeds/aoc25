fn main() {
    println!("Hello, AoC Day 2!");
    println!("-------------------");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum: usize = 0;

    // Part 1
    time_it!({
        for range in &ranges {
            let split_range: Vec<&str> = range.split('-').collect();
            let start: usize = split_range[0].parse().unwrap();
            let end: usize = split_range[1].parse().unwrap();
            
            for num in start..=end {
                let num_s = num.to_string();
                let num_len = num_s.len();
                if num_len % 2 == 0 {
                    let (first, second) = num_s.split_at(num_len/2);
                    if first == second {
                        sum = sum + num;
                    }
                }
            }
            
        }
        println!("Part 1 total: {}", sum);
        sum = 0;
    });

    println!("-------------------");

    // Part 2
    time_it!({
        for range in &ranges {
            let split_range: Vec<&str> = range.split('-').collect();
            let start: usize = split_range[0].parse().unwrap();
            let end: usize = split_range[1].parse().unwrap();

            for num in start..=end {
                let num_s = num.to_string();
                let num_len = num_s.len();

                for i in 1..=num_len/2 {
                    let chunks: Vec<&str> = num_s.as_bytes()
                        .chunks(i)
                        .map(|c| std::str::from_utf8(c).unwrap())
                        .collect();

                    if chunks.iter().any(|c| c.len() != i) {
                        continue;
                    }

                    if chunks.windows(2).all(|w| w[0] == w[1]) {
                        sum += num;
                        break;
                    }
                }
            }
            
        }
        println!("Part 2 total: {}", sum);
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