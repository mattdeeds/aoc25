use std::collections::HashMap;
use z3::ast::{Ast, Int};

// BINDGEN_EXTRA_CLANG_ARGS="-I/opt/homebrew/include" RUSTFLAGS="-L/opt/homebrew/lib" cargo build --release 2>&1 && ./target/release/day-10

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

    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

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

        let num_buttons = state.buttons.len();
        let num_indicators = state.indicators.len();
        let total_combinations = 1 << num_buttons;

        for combination in 0..total_combinations {
            let mut current_indicators = vec![false; num_indicators];
            let mut presses = 0;

            for button_idx in 0..num_buttons {
                if (combination & (1 << button_idx)) != 0 {
                    presses += 1;

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

        // Part 2: Find minimum button presses to match joltage using z3 solver
        if !state.joltage.is_empty() && num_buttons > 0 {
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let optimizer = z3::Optimize::new(&ctx);
            
            let mut button_vars = Vec::new();
            for i in 0..num_buttons {
                let var = Int::new_const(&ctx, format!("button_{}", i));
                optimizer.assert(&var.ge(&Int::from_i64(&ctx, 0)));
                button_vars.push(var);
            }
            
            for (counter_idx, &target) in state.joltage.iter().enumerate() {
                let mut counter_expr = Int::from_i64(&ctx, 0);
                
                for (button_idx, joltage_indexes) in &state.buttons {
                    if joltage_indexes.contains(&counter_idx) {
                        counter_expr = counter_expr + &button_vars[*button_idx];
                    }
                }
                
                optimizer.assert(&counter_expr._eq(&Int::from_i64(&ctx, target as i64)));
            }
            
            let mut total_presses = Int::from_i64(&ctx, 0);
            for var in &button_vars {
                total_presses = total_presses + var;
            }
            optimizer.minimize(&total_presses);
            
            // Solve
            if optimizer.check(&[]) == z3::SatResult::Sat {
                if let Some(model) = optimizer.get_model() {
                    let min_presses = model.eval(&total_presses, true).unwrap().as_i64().unwrap();
                    p2_total += min_presses as usize;
                }
            } else {
                println!("No solution found for Part 2!");
            }
        }

    }
    println!("Part 1 total: {}", p1_total);
    println!("-------------------");
    println!("Part 2 total: {}", p2_total)
}
