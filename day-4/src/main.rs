fn main() {
    println!("Hello, AoC Day 4!");
    println!("-------------------");

    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");

    let rows: Vec<&str> = input.split('\n').collect();
    let row_size = rows[0].len();
    let mut rolls: Vec<char> = rows.iter().flat_map(|&row| row.chars()).collect();

    // Part 1
    let (p1_total, _) = window(&rolls, row_size);
    println!("Part 1 Total: {}", p1_total);

    // Part 2
    let mut sum = 0;
    let mut con = true;
    while con {
        let (total, indices) = window(&rolls, row_size);
        for i in indices {
            if rolls[i] == '@' {
                rolls[i] = '.';
            }
        }
        if total == 0 {
            con = false;
        }
        sum += total;
    }    
    println!("Part 2 Total: {}", sum);
    
}

fn window(rolls: &[char], row_size: usize) -> (usize, Vec<usize>) {
    let mut total= 0;
    let mut valid_indices: Vec<usize> = Vec::new();
    for i in 0..rolls.len() {
        let roll = rolls[i];
        let mut count = 0;
        match roll {
            '.' => {
                continue;
            },
            '@' => {
                // get left roll
                if i % row_size != 0 && i > 0 {
                    if rolls[i-1] == '@' {
                        count += 1;
                    }
                }
                // get right roll
                if i % row_size != row_size - 1 && i < rolls.len() - 1 {
                    if rolls[i+1] == '@' {
                        count += 1;
                    }
                }
                // get 3 above rolls
                if i % row_size != 0 && i >= row_size {
                    if rolls[i-row_size-1] == '@' {
                        count += 1;
                    }
                }
                if i >= row_size {
                    if rolls[i-row_size] == '@' {
                        count += 1;
                    }
                }
                if i % row_size != row_size - 1 && i >= row_size {
                    if rolls[i-row_size+1] == '@' {
                        count += 1;
                    }
                }
                // get 3 below rolls
                if i % row_size != 0 && i + row_size - 1 < rolls.len() {
                    if rolls[i+row_size-1] == '@' {
                        count += 1;
                    }
                }
                if i + row_size < rolls.len() {
                    if rolls[i+row_size] == '@' {
                        count += 1;
                    }
                }
                if i % row_size != row_size - 1 && i + row_size + 1 < rolls.len() {
                    if rolls[i+row_size+1] == '@' {
                        count += 1;
                    }
                }
            },
            _ => break
        }
        if count < 4 {
            total += 1;
            valid_indices.push(i);
        }
    }
    (total, valid_indices)
}