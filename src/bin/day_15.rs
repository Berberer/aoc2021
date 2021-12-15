use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn input_data() -> Vec<&'static str> {
    vec![
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ]
}

fn parse_risk_level_map(input_data: Vec<&str>) -> (Vec<Vec<u32>>, usize, usize) {
    // Get a numerical risk level map from the input lines as well as the map dimensions
    let y_size = input_data.len();
    let x_size = input_data[0].len();
    let risk_level_map = input_data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    (risk_level_map, x_size, y_size)
}

fn extend_map(map: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize, usize) {
    let original_y_size = map.len();
    let extended_y_size = original_y_size * 5;
    let original_x_size = map[0].len();
    let extended_x_size = original_x_size * 5;
    let mut extended_map = vec![vec![0; extended_x_size]; extended_y_size];
    for y in 0..extended_y_size {
        for x in 0..extended_x_size {
            let risk_level = (map[y % original_y_size][x % original_x_size] as u32
                + y as u32 / original_y_size as u32
                + x as u32 / original_x_size as u32
                - 1)
                % 9
                + 1;
            extended_map[y][x] = risk_level;
        }
    }

    (extended_map, extended_x_size, extended_y_size)
}

fn generate_neighbors(point: (usize, usize), x_size: usize, y_size: usize) -> Vec<(usize, usize)> {
    // Get all valid neighbor coordinates for a point coordinate
    let (x, y) = point;
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < x_size - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < y_size - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

// Struct for elements of the search priority queue of Dijkstra algorithm
#[derive(Copy, Clone, Eq, PartialEq)]
struct QueueElement {
    cost: u32,
    coordinates: (usize, usize),
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_path_to_target(map: &Vec<Vec<u32>>, x_size: usize, y_size: usize) -> u32 {
    // Dijkstra algorithm to find shortest paths from the start field (0, 0)
    let mut costs = HashMap::new();
    let mut open_fields = HashSet::new();
    let mut queue = BinaryHeap::new();
    for x in 0..x_size {
        for y in 0..y_size {
            if y == 0 && x == 0 {
                continue;
            }
            costs.insert((x, y), None);
            open_fields.insert((x, y));
        }
    }
    costs.insert((0, 0), Some(0));
    queue.push(QueueElement {
        cost: 0,
        coordinates: (0, 0),
    });

    while let Some(QueueElement { cost, coordinates }) = queue.pop() {
        let neighbors = generate_neighbors(coordinates, x_size, y_size);
        for neighbor in neighbors {
            if open_fields.contains(&neighbor) {
                open_fields.remove(&neighbor);
                let neighbor_cost = cost + map[neighbor.1][neighbor.0];
                let update = if let Some(current_cost) = costs[&neighbor] {
                    current_cost > cost
                } else {
                    true
                };
                if update {
                    queue.push(QueueElement {
                        cost: neighbor_cost,
                        coordinates: neighbor,
                    });
                    costs.insert(neighbor, Some(neighbor_cost));
                }
            }
        }
    }

    costs[&(x_size - 1, y_size - 1)].unwrap()
}

fn main() {
    let (risk_level_map, x_size, y_size) = parse_risk_level_map(input_data());

    // Solution for puzzle 1
    let cost_to_goal = find_min_path_to_target(&risk_level_map, x_size, y_size);
    println!("Minimum risk level to reach the goal is {}", cost_to_goal);

    // Solution for puzzle 2
    let (risk_level_map, x_size, y_size) = extend_map(risk_level_map);
    let cost_to_goal = find_min_path_to_target(&risk_level_map, x_size, y_size);
    println!(
        "Minimum risk level to reach the goal on the extended map is {}",
        cost_to_goal
    );
}
