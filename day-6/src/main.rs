fn main() {
    println!("Hello, AoC Day 6!");
    println!("-------------------");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    part1(input);
    part2(input);
}

enum Operation {
    Add,
    Multiply,
    None
}

fn part2(input: &str) {
    // Placeholder for part 2 implementation
    let lines = input.lines().collect::<Vec<_>>();
    let rows = lines.len();
    let mut iters = 0;

    let mut merged: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        iters = chars.len();
        merged.push(chars);
    }

    let mut to_sum: Vec<usize> = Vec::new();
    let mut current_operation = Operation::None;
    let mut last_operation = Operation::None;
    let mut current_count = 0;
    let mut do_op = false;

    for _ in 0..iters {
        let mut row_total: usize = 0;

        let last_row = merged[rows - 1].clone();
        let mut last = ' ';
        for (i, c) in last_row.iter().enumerate() {
            match c {
                '*' => {
                    last = '*';
                }
                '+' => {
                    last = '+';
                }
                _ => {
                    merged[rows - 1][i] = last;
                }
            }
        }

        // pop last element from each row
        let mut stack: Vec<char> = Vec::new();
        for row in &mut merged {
            let val = row.pop().unwrap();
            stack.push(val);
        }

        let mut operand: Vec<char> = Vec::new();
        for c in &mut stack {
            match c {
                '*' => {
                    let operand_str: String = operand.iter().collect();
                    let num = operand_str.parse::<usize>();
                    match num {
                        Ok(num) => {
                            current_operation = Operation::Multiply;
                            row_total = num;
                        }
                        Err(_) => {
                            last_operation = current_operation;
                            current_operation = Operation::Multiply;
                            do_op = true;
                        }
                    }
                }
                '+' => {
                    let operand_str: String = operand.iter().collect();
                    let num = operand_str.parse::<usize>();
                    match num {
                        Ok(num) => {
                            current_operation = Operation::Add;
                            row_total = row_total + num;
                        }
                        Err(_) => {
                            last_operation = current_operation;
                            current_operation = Operation::Add;
                            do_op = true;
                        }
                    }
                }
                ' ' => {}
                _ => {
                    operand.push(*c);
                    do_op = false;
                }
            }
        }

        if !do_op {
            to_sum.push(row_total);
            current_count += 1;
        }

        if do_op {
            match last_operation {
                Operation::Add => {
                    let sum: usize = to_sum.iter().skip(to_sum.len() - current_count).sum();
                    to_sum.truncate(to_sum.len() - current_count);
                    to_sum.push(sum);
                }
                Operation::Multiply => {
                    let product: usize = to_sum.iter().skip(to_sum.len() - current_count).product();
                    to_sum.truncate(to_sum.len() - current_count);
                    to_sum.push(product);
                }
                Operation::None => {} 
            }
            current_count = 0;
        }
    }

    // final operation
    match current_operation {
        Operation::Add => {
            let sum: usize = to_sum.iter().skip(to_sum.len() - current_count).sum();
            to_sum.truncate(to_sum.len() - current_count);
            to_sum.push(sum);
        }
        Operation::Multiply => {
            let product: usize = to_sum.iter().skip(to_sum.len() - current_count).product();
            to_sum.truncate(to_sum.len() - current_count);
            to_sum.push(product);
        }
        Operation::None => {}
    }

    println!("Part 2 Total: {:?}", to_sum.iter().sum::<usize>());
}

fn part1(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let rows = lines.len();
    let columns = lines[0].split_whitespace().collect::<Vec<_>>().len();

    let mut merged: Vec<String> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        for part in parts {
            merged.push(part.to_string());
        }
    }

    let mut p1_total = 0;

    for i in 0..columns {
        let mut stack: Vec<usize> = Vec::new();
        for j in 0..rows {
            let val = merged[i + j * columns].clone();
            let num = val.parse::<usize>();
            match num {
                Ok(num) => {
                    stack.push(num);
                }
                Err(e) => match val.as_str() {
                    "*" => {
                        let product: usize = stack.iter().product();
                        stack.clear();
                        stack.push(product);
                        p1_total += product;
                    }
                    "+" => {
                        let sum: usize = stack.iter().sum();
                        stack.clear();
                        stack.push(sum);
                        p1_total += sum;
                    }
                    _ => {
                        println!("Error parsing value: {}", e);
                    }
                },
            }
        }
    }
    println!("Part 1 Total: {}", p1_total);
}
