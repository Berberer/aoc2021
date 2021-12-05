use std::cmp;

fn input_data() -> Vec<&'static str> {
    // Replace these line coordinates with different lines
    vec![
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ]
}

fn parse_line_coordinates(line_coordinates: Vec<&str>) -> Vec<((usize, usize), (usize, usize))> {
    // Gen numeric coordinates from textual representation
    line_coordinates
        .iter()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(line_start, line_end)| {
            (
                line_start.split_once(',').unwrap(),
                line_end.split_once(',').unwrap(),
            )
        })
        .map(|((line_start_x, line_start_y), (line_end_x, line_end_y))| {
            (
                (
                    line_start_x.parse::<usize>().unwrap(),
                    line_start_y.parse::<usize>().unwrap(),
                ),
                (
                    line_end_x.parse::<usize>().unwrap(),
                    line_end_y.parse::<usize>().unwrap(),
                ),
            )
        })
        .collect()
}

fn get_dimensions(line_coordinates: &Vec<((usize, usize), (usize, usize))>) -> (usize, usize) {
    // Get highest coordinates for the dimensions of the map
    line_coordinates.iter().fold(
        (0, 0),
        |(max_x, max_y), ((start_x, start_y), (end_x, end_y))| {
            (
                cmp::max(max_x, cmp::max(*start_x, *end_x)),
                cmp::max(max_y, cmp::max(*start_y, *end_y)),
            )
        },
    )
}

fn get_points_of_line(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Vec<(usize, usize)> {
    // Generate the discrete points of the whole line from the coordinates
    let mut points = Vec::new();
    let length = cmp::max(
        (end_x - start_x).abs() as i32,
        (end_y - start_y).abs() as i32,
    );
    let (x_d, y_d) = (
        (end_x - start_x).clamp(-1.0, 1.0),
        (end_y - start_y).clamp(-1.0, 1.0),
    );
    let mut x = start_x;
    let mut y = start_y;
    for _ in 0..=length {
        points.push((x as usize, y as usize));
        x = x + x_d;
        y = y + y_d;
    }
    points
}

fn fill_map(
    line_coordinates: &Vec<((usize, usize), (usize, usize))>,
    dimensions: &(usize, usize),
    draw_diagonal_lines: bool,
) -> Vec<Vec<usize>> {
    // Draw all lines on the map
    let mut map = vec![vec![0; dimensions.0 as usize + 1]; dimensions.1 as usize + 1];
    for ((start_x, start_y), (end_x, end_y)) in line_coordinates {
        if draw_diagonal_lines || start_x == end_x || start_y == end_y {
            let line_points = get_points_of_line(
                *start_x as f64,
                *start_y as f64,
                *end_x as f64,
                *end_y as f64,
            );
            for (x, y) in line_points {
                map[y][x] = map[y][x] + 1;
            }
        }
    }
    map
}

fn count_dangerous_areas(map: Vec<Vec<usize>>) -> i32 {
    // Count dangerous areas of the map (overlapping lines)
    let mut areas = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] > 1 {
                areas = areas + 1;
            }
        }
    }
    areas
}

fn main() {
    let coordinates = parse_line_coordinates(input_data());
    let dimensions = get_dimensions(&coordinates);

    // Solution for puzzle 1
    let map = fill_map(&coordinates, &dimensions, false);
    let dangerous_areas = count_dangerous_areas(map);
    println!(
        "Number of dangerous areas without diagonal lines: {}",
        dangerous_areas
    );

    // Solution for puzzle 2
    let map = fill_map(&coordinates, &dimensions, true);
    let dangerous_areas = count_dangerous_areas(map);
    println!(
        "Number of dangerous areas with diagonal lines: {}",
        dangerous_areas
    );
}
