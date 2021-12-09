use std::collections::HashSet;

fn input_data() -> Vec<&'static str> {
    // Replace these height map values with a different set of numbers
    vec![
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ]
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<i32>> {
    // Create numerical height map representation from input lines
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|number_char| number_char.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_local_minimum(x: i32, neighbors: Vec<i32>) -> bool {
    // Check if all four neighbors have higher values
    neighbors.iter().cloned().all(|height| height > x)
}

fn get_neighbor_coordinate_in_direction(
    (x, y): (usize, usize),
    direction: &Direction,
    x_border: usize,
    y_border: usize,
) -> Option<(usize, usize)> {
    // Get the next neighboring coordinate in the given direction if available
    let x = match (x, &direction) {
        (0, Direction::Left) => None,
        (x, Direction::Left) => Some(x - 1),
        (x, Direction::Right) if x == x_border => None,
        (x, Direction::Right) => Some(x + 1),
        (x, _) => Some(x),
    };
    let y = match (y, &direction) {
        (0, Direction::Up) => None,
        (y, Direction::Up) => Some(y - 1),
        (y, Direction::Down) if y == y_border => None,
        (y, Direction::Down) => Some(y + 1),
        (y, _) => Some(y),
    };
    if x.and(y).is_some() {
        Some((x.unwrap(), y.unwrap()))
    } else {
        None
    }
}

fn get_neighbor_coordinates(
    coordinate: (usize, usize),
    x_border: usize,
    y_border: usize,
) -> Vec<(usize, usize)> {
    // Get the two - four neighboring coordinates
    vec![
        get_neighbor_coordinate_in_direction(coordinate, &Direction::Up, x_border, y_border),
        get_neighbor_coordinate_in_direction(coordinate, &Direction::Down, x_border, y_border),
        get_neighbor_coordinate_in_direction(coordinate, &Direction::Left, x_border, y_border),
        get_neighbor_coordinate_in_direction(coordinate, &Direction::Right, x_border, y_border),
    ]
    .iter()
    .filter(|neighbor| neighbor.is_some())
    .map(|neighbor| neighbor.unwrap())
    .collect()
}

fn find_local_minima(
    height_map: &Vec<Vec<i32>>,
    x_border: usize,
    y_border: usize,
) -> Vec<(usize, usize)> {
    // Search the map for local minima and collect their coordinates
    let mut local_minima = Vec::new();
    for y in 0..=y_border {
        for x in 0..=x_border {
            let neighbor_heights = get_neighbor_coordinates((x, y), x_border, y_border)
                .iter()
                .cloned()
                .map(|(x, y)| height_map[y][x])
                .collect();

            if is_local_minimum(height_map[y][x], neighbor_heights) {
                local_minima.push((x, y));
            }
        }
    }
    local_minima
}

// Puzzle 1 //

fn sum_coordinate_heights(coordinates: &Vec<(usize, usize)>, height_map: &Vec<Vec<i32>>) -> i32 {
    // Get the heights+1 of a list of coordinates and calculate their sum
    coordinates
        .iter()
        .cloned()
        .map(|(x, y)| height_map[y][x] + 1)
        .sum()
}

// Puzzle 2 //

fn get_basin_size(
    (x, y): (usize, usize),
    height_map: &Vec<Vec<i32>>,
    x_border: usize,
    y_border: usize,
) -> usize {
    // Extend basin from minimum iteratively until the basin edges (fields with a 9) are reached
    // Store each basin field in a hash set to prevent duplicate extensions and keep count of size
    let mut basin_fields = HashSet::<(usize, usize)>::new();
    basin_fields.insert((x, y));
    let mut basin_borders_reached = false;

    while !basin_borders_reached {
        basin_borders_reached = true;

        for (x, y) in basin_fields.clone() {
            let unchecked_neighbors = get_neighbor_coordinates((x, y), x_border, y_border)
                .iter()
                .cloned()
                .filter(|(x, y)| !(height_map[*y][*x] == 9 || basin_fields.contains(&(*x, *y))))
                .collect::<HashSet<(usize, usize)>>();

            if unchecked_neighbors.len() > 0 {
                basin_borders_reached = false;
                basin_fields = basin_fields.union(&unchecked_neighbors).cloned().collect();
            }
        }
    }
    basin_fields.len()
}

fn get_three_largest_basin_sizes(
    local_minima: &Vec<(usize, usize)>,
    height_map: &Vec<Vec<i32>>,
    x_border: usize,
    y_border: usize,
) -> Vec<usize> {
    // Get the basin size starting at each local optimum
    // Return the sizes of the three largest
    let mut basin_sizes = local_minima
        .iter()
        .cloned()
        .map(|basin_minimum| get_basin_size(basin_minimum, height_map, x_border, y_border))
        .collect::<Vec<usize>>();

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0..3].to_vec()
}

fn main() {
    let height_map = parse_input(input_data());
    let x_border = height_map[0].len() - 1;
    let y_border = height_map.len() - 1;
    let local_minima = find_local_minima(&height_map, x_border, y_border);

    // Solution for puzzle 1
    let local_minima_height_sum = sum_coordinate_heights(&local_minima, &height_map);
    println!(
        "Sum of the heights of the local minima: {}",
        local_minima_height_sum
    );

    // Solution for puzzle 2
    let basin_sizes = get_three_largest_basin_sizes(&local_minima, &height_map, x_border, y_border);
    println!(
        "Product of the three largest basin sizes: {}",
        basin_sizes.iter().product::<usize>()
    );
}
