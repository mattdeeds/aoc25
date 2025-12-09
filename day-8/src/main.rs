#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

#[derive(Clone)]
struct Connection {
    a: Point,
    b: Point,
    distance: f64,
}

impl Connection {
    fn new(a: Point, b: Point, distance: f64) -> Self {
        Connection { a, b, distance }
    }
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, p: usize) -> usize {
        if self.parent[p] != p {
            self.parent[p] = self.find(self.parent[p]);
        }
        self.parent[p]
    }

    fn union(&mut self, p: usize, q: usize) -> bool {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return false;
        }

        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }

        self.count -= 1;
        true
    }
}

struct Space {
    points: Vec<Point>,
    connections: Vec<Connection>,
}

impl Space {
    fn new() -> Self {
        Space {
            points: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn calculate_distances(&mut self) {
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                let dist = self.points[i].distance(&self.points[j]);
                self.connections.push(Connection::new(
                    self.points[i].clone(),
                    self.points[j].clone(),
                    dist,
                ));
            }
        }
    }

    fn sort_distances(&mut self, limit: usize) {
        self.connections
            .select_nth_unstable_by(limit, |a, b| a.distance.partial_cmp(&b.distance).unwrap());
        self.connections.truncate(limit);
    }

    fn sort_all(&mut self) {
        self.connections
            .sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    }

    fn parse_input(&mut self, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        for line in &lines {
            let l: Vec<&str> = line.split(',').collect();
            let nums: Vec<u64> = l.iter().map(|s| s.parse().unwrap()).collect();
            self.points.push(Point::new(nums[0], nums[1], nums[2]));
        }
    }
}

fn p1(input: &str, limit: usize) {
    let mut space = Space::new();

    space.parse_input(input);
    space.calculate_distances();
    space.sort_distances(limit);

    let mut ds = DisjointSet::new(space.points.len());
    for connection in &space.connections {
        let a_index = space
            .points
            .iter()
            .position(|p| *p == connection.a)
            .unwrap();
        let b_index = space
            .points
            .iter()
            .position(|p| *p == connection.b)
            .unwrap();
        ds.union(a_index, b_index);
    }

    ds.size.sort_unstable_by(|a, b| b.cmp(a));
    let product: usize = ds.size.iter().take(3).product();
    println!("Part 1: {}", product);
}

fn p2(input: &str) {
    let mut space = Space::new();

    space.parse_input(input);
    space.calculate_distances();
    space.sort_all();

    let mut final_points: (Point, Point) = (Point::new(0, 0, 0), Point::new(0, 0, 0));
    let mut ds = DisjointSet::new(space.points.len());
    for connection in &space.connections {
        let a_index = space
            .points
            .iter()
            .position(|p| *p == connection.a)
            .unwrap();
        let b_index = space
            .points
            .iter()
            .position(|p| *p == connection.b)
            .unwrap();
        let u = ds.union(a_index, b_index);
        if ds.count == 1 && u {
            final_points = (connection.a.clone(), connection.b.clone());
            break;
        }
    }
    let product = final_points.0.x * final_points.1.x;
    println!("Part 2: {}", product);
}

fn main() {
    println!("Hello, AoC Day 8!");
    println!("-------------------");

    // let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    p1(input, 1000);
    p2(input);
}
