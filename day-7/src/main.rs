use std::collections::HashMap;

struct Manifold {
    width: usize,
    height: usize,
    grid: Vec<char>,
    splitter_hit_pos: Vec<usize>,
    beam_pos: Vec<usize>,
    unique_beams: HashMap<usize, usize>,
}

impl Manifold { 
    fn new(input: &str) -> Manifold {
        let lines = input.lines();
        let mut width = 0;
        let height = lines.clone().count();
        let mut grid: Vec<char> = Vec::new();
        for line in lines {
            if line.len() > width {
                width = line.len();
            }
            grid.extend(line.chars());
        }
        Manifold {
            width,
            height,
            grid,
            splitter_hit_pos: Vec::new(),
            beam_pos: Vec::new(),
            unique_beams: HashMap::new(),
        }
    }

    fn update_beam_positions(&mut self) {
        let mut new_beam_pos = Vec::new();

        for (i, c) in self.grid.iter().enumerate() {
            if i >= self.width * (self.height - 1) {
                break;
            }
            match c {
                '.' => {}
                '^' => {
                    if self.beam_pos.contains(&(i - self.width)) && !self.splitter_hit_pos.contains(&i) {
                        new_beam_pos.push(i - 1);
                        new_beam_pos.push(i + 1);
                        self.splitter_hit_pos.push(i);
                    }
                }
                'S' => {
                    new_beam_pos.push(i + self.width);
                }
                '|' => {
                    if self.grid[i + self.width] != '^' {
                        new_beam_pos.push(i + self.width);
                    }
                }
                _ => println!("Unknown"),
            }
        }

        self.beam_pos = new_beam_pos;
    }

    fn update_grid(&mut self) {
        for &pos in &self.beam_pos {
            if self.grid[pos] == '.' {
                self.grid[pos] = '|';
            }
        }
    }

    fn _print_grid(&self) {
        for (i, c) in self.grid.iter().enumerate() {
            print!("{}", c);
            if i % self.width == self.width - 1 {
                println!();
            }
        }
        println!("{}", "-".repeat(self.width));
    }
    fn p2(&mut self, current_line: usize) {
        // println!("Processing line {}", current_line);
        for i in current_line * self.width..current_line * self.width + self.width {
            // println!("Processing position {}", i);
            match self.grid[i] {
                '.' => {}
                '^' => {
                    let current_value = self.unique_beams.get(&(i % self.width)).cloned().unwrap_or(0);

                    if self.unique_beams.contains_key(&(i % self.width)) {
                        self.unique_beams.remove(&(i % self.width));
                    }

                    self.unique_beams
                        .entry((i - 1) % self.width)
                        .and_modify(|e| *e += current_value)
                        .or_insert(current_value);


                    self.unique_beams
                        .entry((i + 1) % self.width)
                        .and_modify(|e| *e += current_value)
                        .or_insert(current_value);
                    
                }
                'S' => {
                    self.unique_beams
                        .entry(i % self.width)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                _ => println!("Unknown"),
            }
        }
    }
}


fn main() {
    println!("Hello, AoC Day 7!");
    println!("-------------------");

    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    // Part 1
    let mut manifold = Manifold::new(input);
    for _ in 0..manifold.height {
        manifold.update_beam_positions();
        manifold.update_grid();
        // manifold.print_grid();
    }
    println!("Part 1 - Total: {}", manifold.splitter_hit_pos.len());

    // Part 2
    let mut manifold = Manifold::new(input);
    for l in 0..manifold.height {
        manifold.p2(l);
    }

    let total_beams: usize = manifold.unique_beams.values().sum();
    println!("Part 2 - Total: {}", total_beams);
}