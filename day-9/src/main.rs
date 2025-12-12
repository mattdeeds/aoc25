fn main() {
    println!("Hello, AoC Day 9!");
    println!("-------------------");

    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    part1(&points);
    println!("-------------------");
    part2(&points); // cargo run --release  3926.01s user 21.96s system 99% cpu 1:06:06.16 total (~ 1 hour)
}

fn part1(points: &Vec<(i64, i64)>) -> i64 {
    let mut max_area = 0;
    for (x, y) in points {
        for (x2, y2) in points.iter() {
            let area = ((x - x2).abs() + 1) * ((y - y2).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Max area: {}", max_area);
    max_area
}

fn is_point_in_polygon(poly: &[(i64, i64)], point: (i64, i64)) -> bool {
    let (px, py) = point;
    let mut inside = false;

    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];

        // Check if point is exactly on a vertex
        if (px, py) == (x1, y1) || (px, py) == (x2, y2) {
            return true;
        }

        // Check if point is on edge (collinear + within bounds)
        let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
        if cross == 0 {
            // Check bounding box
            if px >= x1.min(x2) && px <= x1.max(x2)
                && py >= y1.min(y2) && py <= y1.max(y2)
            {
                return true;
            }
        }

        // Ray crossing
        let intersects = ((y1 > py) != (y2 > py))
            && px < (x2 - x1) * (py - y1) / (y2 - y1) + x1;

        if intersects {
            inside = !inside;
        }
    }

    inside
}


fn part2(points: &Vec<(i64, i64)>) -> i64 {
    // Calculate bounding box of polygon for quick rejection
    let (min_x, max_x, min_y, max_y) = points.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
        }
    );

    let mut max_area = 0;
    for (x, y) in points {
        for (x2, y2) in points.iter() {
            if (x2, y2) == (x, y) {
                continue;
            }
            
            let rect_min_x = (*x).min(*x2);
            let rect_max_x = (*x).max(*x2);
            let rect_min_y = (*y).min(*y2);
            let rect_max_y = (*y).max(*y2);
            
            // Quick rejection: if rectangle extends beyond polygon bounds
            if rect_min_x < min_x || rect_max_x > max_x || 
               rect_min_y < min_y || rect_max_y > max_y {
                continue;
            }
            
            let area = (rect_max_x - rect_min_x + 1) * (rect_max_y - rect_min_y + 1);
            
            // Skip if this can't be better than current max (pruning)
            if area <= max_area {
                continue;
            }

            let mut all_inside = true;
            
            // Check top and bottom edges
            for px in rect_min_x..=rect_max_x {
                if !is_point_in_polygon(points, (px, rect_min_y)) ||
                   !is_point_in_polygon(points, (px, rect_max_y)) {
                    all_inside = false;
                    break;
                }
            }
            
            if !all_inside { continue; }
            
            // Check left and right edges (excluding corners already checked)
            for py in (rect_min_y + 1)..rect_max_y {
                if !is_point_in_polygon(points, (rect_min_x, py)) ||
                   !is_point_in_polygon(points, (rect_max_x, py)) {
                    all_inside = false;
                    break;
                }
            }
            
            if !all_inside { continue; }
            
            let width = rect_max_x - rect_min_x + 1;
            let height = rect_max_y - rect_min_y + 1;
            
            if width > 2 && height > 2 {
                let x_step = 1.max(width / 50) as usize;
                let y_step = 1.max(height / 50) as usize;
                
                'check_interior: for px in ((rect_min_x + 1)..rect_max_x).step_by(x_step) {
                    for py in ((rect_min_y + 1)..rect_max_y).step_by(y_step) {
                        if !is_point_in_polygon(points, (px, py)) {
                            all_inside = false;
                            break 'check_interior;
                        }
                    }
                }
            }

            if all_inside && area > max_area {
                max_area = area;
            }
        }
    }
    println!("Max area: {}", max_area);
    max_area
}
