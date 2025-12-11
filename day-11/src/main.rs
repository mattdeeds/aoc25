use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, AoC Day 11!");
    println!("-------------------");

    // let input = include_str!("../test.txt");
    // let input = include_str!("../test2.txt");
    let input = include_str!("../input.txt");

    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() >= 2 {
            let key = parts[0].trim_end_matches(':').to_string();
            let values = parts[1..].iter().map(|s| s.to_string()).collect();
            paths.insert(key, values);
        }
    }

    part1(&paths);
    part2(&paths);
}

fn part1(paths: &HashMap<String, Vec<String>>) {
    let count = count_paths_segment(
        paths,
        "you",
        "out",
        &mut HashSet::new(),
        &mut HashMap::new(),
    );
    println!("Part 1: {}", count);
}

fn part2(paths: &HashMap<String, Vec<String>>) {
    let count1 = count_paths_through(paths, "svr", "out", &["dac", "fft"]);
    let count2 = count_paths_through(paths, "svr", "out", &["fft", "dac"]);
    let total = count1 + count2;
    println!("Part 2: {}", total);
}

fn count_paths_through(
    paths: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    through: &[&str],
) -> usize {
    let mut current = start;
    let mut total = 1;

    for &next_required in through {
        let segment_count = count_paths_segment(
            paths,
            current,
            next_required,
            &mut HashSet::new(),
            &mut HashMap::new(),
        );
        if segment_count == 0 {
            // No paths for this segment
            return 0;
        }
        total *= segment_count;
        current = next_required;
    }

    let final_count = count_paths_segment(
        paths,
        current,
        end,
        &mut HashSet::new(),
        &mut HashMap::new(),
    );
    total * final_count
}

fn count_paths_segment(
    paths: &HashMap<String, Vec<String>>,
    current: &str,
    end: &str,
    visited: &mut HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if current == end {
        return 1;
    }

    if !visited.contains(current) {
        if let Some(&count) = memo.get(current) {
            return count;
        }
    }

    visited.insert(current.to_string());
    let mut total = 0;

    if let Some(neighbors) = paths.get(current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                total += count_paths_segment(paths, neighbor, end, visited, memo);
            }
        }
    }

    visited.remove(current);

    memo.insert(current.to_string(), total);
    total
}
