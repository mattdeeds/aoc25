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
    part2(&points);
}

fn part1(points: &Vec<(i64, i64)>) {
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


fn part2(points: &Vec<(i64, i64)>) {
    let mut max_area = 0;
    for (x, y) in points {
        // println!("Checking point: {}, {}", x, y);
        for (x2, y2) in points.iter() {
            if (x2, y2) == (x, y) {
                continue;
            }
            // println!("  Against point: {}, {}", x2, y2);
            let area = ((x - x2).abs() + 1) * ((y - y2).abs() + 1);

            let mut all_inside = true;
            for px in (*x).min(*x2)..=(*x).max(*x2) {
                for py in (*y).min(*y2)..=(*y).max(*y2) {
                    if !is_point_in_polygon(points, (px, py)) {
                        all_inside = false;
                        break;
                    }
                }
                if !all_inside {
                    break;
                }
            }

            if all_inside && area > max_area {
                // println!("    New max area found: {} (points: ({}, {}) to ({}, {}))", area, x, y, x2, y2);
                max_area = area;
            }
        }
    }
    println!("Max area: {}", max_area);
}