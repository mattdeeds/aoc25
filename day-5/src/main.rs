fn main() {
    println!("Hello, AoC Day 5!");
    println!("-------------------");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let sections: Vec<&str> = input.split("\n\n").collect();
    let fresh: Vec<&str> = sections[0].split('\n').collect();
    let ingredients: Vec<&str> = sections[1].split('\n').collect();

    let mut p1_total = 0;
    let mut added: Vec<usize> = Vec::new();

    // Part 1
    for range in &fresh {
        let parts: Vec<&str> = range.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        for id in ingredients.clone() {
            let id_value: usize = id.parse().unwrap();
            if id_value >= start && id_value <= end && !added.contains(&id_value) {
                p1_total += 1;
                added.push(id_value);
            }
        }
    }
    println!("Part 1 Total: {}", p1_total);
    println!("----");

    // Part 2
    time_it!({
        // Parse ranges into tuples
        let mut ranges: Vec<(usize, usize)> = fresh.iter().map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: usize = parts[0].parse().unwrap();
            let end: usize = parts[1].parse().unwrap();
            (start, end)
        }).collect();

        // Sort ranges by start value
        ranges.sort_by_key(|k| k.0);

        // Merge overlapping ranges
        let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
        let mut current_start = ranges[0].0;
        let mut current_end = ranges[0].1;

        for &(start, end) in &ranges[1..] {
            if start <= current_end {
                current_end = current_end.max(end);
            } else {
                merged_ranges.push((current_start, current_end));
                current_start = start;
                current_end = end;
            }
        }
        merged_ranges.push((current_start, current_end));

        // Count unique IDs in merged ranges
        let mut total = 0;
        for range in &merged_ranges {
            total += range.1 - range.0 + 1;
        }

        println!("Part 2 Total: {}", total);
        println!("----");
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