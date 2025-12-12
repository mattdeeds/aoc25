use std::collections::HashMap;

fn main() {
    println!("Hello, AoC Day 12!");
    println!("-------------------");

    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    let mut shapes:HashMap<usize, Vec<bool>>  = HashMap::new();
    let mut dimensions: Vec<((usize, usize), Vec<usize>)> = Vec::new();

    for block in input.split("\n\n") {
        if block.contains("#") || block.contains(".") {
            let mut lines = block.lines();
            let index_line = lines.next().unwrap();
            let index: usize = index_line.trim_end_matches(':').parse().unwrap();

            let mut shape: Vec<bool> = Vec::new();
            for line in lines {
                for ch in line.chars() {
                    match ch {
                        '#' => shape.push(true),
                        '.' => shape.push(false),
                        _ => panic!("Unexpected character in shape"),
                    }
                }
            }
            shapes.insert(index, shape);
        } else {
            // parse dimensions line
            // let parts: Vec<&str> = block.split(':').collect();
            let lines = block.lines().collect::<Vec<&str>>();
            for line in &lines {
                println!("Dimension line: {}", line);
                let parts: Vec<&str> = line.split(':').collect();
                println!("Parts: {:?}", parts);
                let dim_part = parts[0].trim();
                let size_parts: Vec<&str> = dim_part.split('x').collect();
                let width: usize = size_parts[0].parse().unwrap();
                let height: usize = size_parts[1].parse().unwrap();
                println!("Width: {}, Height: {}", width, height);
                let shape_indices: Vec<usize> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                dimensions.push(((width, height), shape_indices))
            }
        }
    }
    
    println!("Parsed shapes: {:?}", shapes);
    println!("Parsed dimensions: {:?}", dimensions);

    let mut good_dimensions = 0;

    for ((width, height), shape_count) in &dimensions {
        let area = width * height;
        println!("Dimension ({}x{}) has area {}", width, height, area);
        println!("Shape counts: {:?}", shape_count);

        // we actually only need to find the total shape area and confirm that it fits within the dimension area
        let mut full_shape_area: usize = 0;
        for shape in shape_count {
            if *shape > 0 {
                full_shape_area += 9 * shape; // all shapes are 3x3, so area is 9
            }
        }
        println!("Total shape area: {}", full_shape_area);

        if full_shape_area <= area {
            good_dimensions += 1;
        }
        
    } 
    println!("Number of good dimensions: {}", good_dimensions);
}
