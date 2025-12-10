use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

struct State {
    indicators: Vec<bool>,
    buttons: HashMap<usize, Vec<usize>>,
    joltage: Vec<usize>,
}

impl State {
    fn new() -> Self {
        State {
            indicators: Vec::new(),
            buttons: HashMap::new(),
            joltage: Vec::new(),
        }
    }
}

fn main() {
    println!("Hello, AoC Day 10!");
    println!("-------------------");

    let input = include_str!("../test.txt");
    // let input = include_str!("../input.txt");

    // let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut p1_total = 0;
    let mut p2_total = 0;

    for line_str in input.lines() {
        let mut state = State::new();
        
        // First, extract the joltage section (between curly braces)
        if let Some(joltage_start) = line_str.find('{') {
            if let Some(joltage_end) = line_str.find('}') {
                let joltage_str = &line_str[joltage_start + 1..joltage_end];
                state.joltage = joltage_str
                    .split(',')
                    .filter_map(|s| s.trim().parse::<usize>().ok())
                    .collect();
            }
        }
        
        let line: Vec<char> = line_str.chars().collect();
        let mut in_indicator = false;
        let mut in_button = false;
        let mut button_index = 0;

        for c in line {
            match c {
                '[' => {
                    // Handle opening bracket
                    in_indicator = true;
                }
                ']' => {
                    // Handle closing bracket
                    in_indicator = false;
                }
                '.' => {
                    // Handle dot
                    if in_indicator {
                        state.indicators.push(false);
                    }
                }
                '#' => {
                    // Handle hash
                    if in_indicator {
                        state.indicators.push(true);
                    }
                }
                '(' => {
                    // Handle opening parenthesis
                    in_button = true;
                    state.buttons.insert(button_index, Vec::new());
                }
                ')' => {
                    // Handle closing parenthesis
                    in_button = false;
                    button_index += 1;
                }
                d if d.is_digit(10) => {
                    // Handle digit - only for buttons
                    if in_button {
                        let digit = d.to_digit(10).unwrap() as usize;
                        state.buttons.get_mut(&button_index).unwrap().push(digit);
                    }
                }
                _ => {}
            }
        }

        // Part 1: Find minimum button presses to match indicators
        let mut min_presses: usize = usize::MAX;

        // button values represent indexes to indicators
        // indicator starts with all off (false)
        // pressing a button toggles the indicators at the given indexes
        // goal is to match the state.indicators with the minimum number of button presses
        // brute force all combinations of button presses to find the minimum presses

        let num_buttons = state.buttons.len();
        let num_indicators = state.indicators.len();

        // Since pressing a button twice cancels out, we only need to check
        // whether each button is pressed 0 or 1 times
        // This gives us 2^num_buttons combinations to check
        let total_combinations = 1 << num_buttons; // 2^num_buttons

        for combination in 0..total_combinations {
            // Create a working state starting with all indicators off
            let mut current_indicators = vec![false; num_indicators];
            let mut presses = 0;

            // Check each button in this combination
            for button_idx in 0..num_buttons {
                // Check if this button is pressed in the current combination
                if (combination & (1 << button_idx)) != 0 {
                    presses += 1;

                    // Toggle the indicators for this button
                    if let Some(indicator_indexes) = state.buttons.get(&button_idx) {
                        for &indicator_idx in indicator_indexes {
                            if indicator_idx < num_indicators {
                                current_indicators[indicator_idx] =
                                    !current_indicators[indicator_idx];
                            }
                        }
                    }
                }
            }

            // Check if this combination matches the target state
            if current_indicators == state.indicators {
                if presses < min_presses {
                    min_presses = presses;
                }
            }
        }

        if min_presses == usize::MAX {
            println!("No solution found!");
        } else {
            p1_total += min_presses;
        }

        // Part 2: Find minimum button presses to match joltage (PARALLEL VERSION)
        // TAKES TOO LONG ON LARGE INPUTS
        
        // Recursive helper function for parallel search
        fn try_combinations_parallel(
            button_idx: usize,
            num_buttons: usize,
            current_presses: &mut Vec<usize>,
            current_joltage: &mut Vec<usize>,
            buttons: &HashMap<usize, Vec<usize>>,
            target_joltage: &[usize],
            min_presses: &AtomicUsize,
            current_total: usize,
        ) {
            // Prune if current total already exceeds minimum found
            let current_min = min_presses.load(Ordering::Relaxed);
            if current_total >= current_min {
                return;
            }
            
            if button_idx == num_buttons {
                // Check if we've matched the target
                if current_joltage == target_joltage {
                    // Update min_presses atomically
                    min_presses.fetch_min(current_total, Ordering::Relaxed);
                }
                return;
            }
            
            // Calculate useful bounds for this button
            if let Some(joltage_indexes) = buttons.get(&button_idx) {
                // Calculate max presses needed for this button based on remaining deficit
                let mut max_useful_presses = 0;
                for &jolt_idx in joltage_indexes {
                    if jolt_idx < target_joltage.len() {
                        let deficit = target_joltage[jolt_idx].saturating_sub(current_joltage[jolt_idx]);
                        max_useful_presses = max_useful_presses.max(deficit);
                    }
                }
                
                // Try different press counts for this button
                for press_count in 0..=max_useful_presses {
                    // Early termination with atomic read
                    let current_min = min_presses.load(Ordering::Relaxed);
                    if current_total + press_count >= current_min {
                        break;
                    }
                    
                    current_presses[button_idx] = press_count;
                    
                    // Apply the presses
                    for &jolt_idx in joltage_indexes {
                        if jolt_idx < current_joltage.len() {
                            current_joltage[jolt_idx] += press_count;
                        }
                    }
                    
                    try_combinations_parallel(
                        button_idx + 1,
                        num_buttons,
                        current_presses,
                        current_joltage,
                        buttons,
                        target_joltage,
                        min_presses,
                        current_total + press_count,
                    );
                    
                    // Undo the presses (backtrack)
                    for &jolt_idx in joltage_indexes {
                        if jolt_idx < current_joltage.len() {
                            current_joltage[jolt_idx] -= press_count;
                        }
                    }
                }
            } else {
                // No button at this index, skip to next
                try_combinations_parallel(
                    button_idx + 1,
                    num_buttons,
                    current_presses,
                    current_joltage,
                    buttons,
                    target_joltage,
                    min_presses,
                    current_total,
                );
            }
        }
        
        // Use AtomicUsize for thread-safe minimum tracking
        let min_presses_atomic = AtomicUsize::new(usize::MAX);
        
        // Parallelize at the first button level
        if num_buttons > 0 && !state.joltage.is_empty() {
            if let Some(joltage_indexes) = state.buttons.get(&0) {
                // Calculate max useful presses for first button
                let mut max_useful_presses_0 = 0;
                for &jolt_idx in joltage_indexes {
                    if jolt_idx < state.joltage.len() {
                        max_useful_presses_0 = max_useful_presses_0.max(state.joltage[jolt_idx]);
                    }
                }
                
                // Try different press counts for button 0 in parallel
                (0..=max_useful_presses_0).into_par_iter().for_each(|press_count_0| {
                    let mut current_presses = vec![0; num_buttons];
                    let mut current_joltage = vec![0; state.joltage.len()];
                    
                    current_presses[0] = press_count_0;
                    
                    // Apply button 0 presses
                    for &jolt_idx in joltage_indexes {
                        if jolt_idx < current_joltage.len() {
                            current_joltage[jolt_idx] += press_count_0;
                        }
                    }
                    
                    // Recursively solve for remaining buttons
                    try_combinations_parallel(
                        1,
                        num_buttons,
                        &mut current_presses,
                        &mut current_joltage,
                        &state.buttons,
                        &state.joltage,
                        &min_presses_atomic,
                        press_count_0,
                    );
                });
            }
        }
        
        let min_presses_p2 = min_presses_atomic.load(Ordering::Relaxed);
        
        if min_presses_p2 == usize::MAX {
            println!("No solution found for Part 2!");
        } else {
            // println!("Part 2 minimum presses: {}", min_presses_p2);
            p2_total += min_presses_p2;
        }

    }
    println!("Part 1 total: {}", p1_total);
    println!("-------------------");
    println!("Part 2 total: {}", p2_total)
}
