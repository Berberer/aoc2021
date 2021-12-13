use std::cmp::max;
use std::collections::HashSet;

fn input_data() -> Vec<&'static str> {
    vec![
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ]
}

fn parse_input_data(input_data: Vec<&str>) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    // Split the input lines into point coordinates and fold instructions
    let mut dots = HashSet::new();
    let mut fold_instructions = Vec::new();
    let mut points_finished = false;

    for input_line in input_data {
        if !points_finished {
            if input_line.is_empty() {
                points_finished = true;
            } else {
                let (x, y) = input_line.split_once(',').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                dots.insert((x, y));
            }
        } else {
            let (axis, value) = input_line.split_once('=').unwrap();
            fold_instructions.push((axis.chars().last().unwrap(), value.parse().unwrap()))
        }
    }

    (dots, fold_instructions)
}

fn fold_paper(
    points: &HashSet<(usize, usize)>,
    fold_direction: char,
    fold_line: usize,
) -> HashSet<(usize, usize)> {
    // Reduce the set of points by executing a fold instruction

    // Partition the points by the fold line
    let (first_half, second_half): (HashSet<(usize, usize)>, HashSet<(usize, usize)>) =
        points.iter().partition(|(x, y)| {
            if fold_direction == 'x' {
                *x < fold_line
            } else {
                *y < fold_line
            }
        });

    // Project the points from the second half to the first half
    // and create the combined point hash set
    let mut remaining_points = first_half.clone();
    for (x, y) in second_half {
        let point_after_fold = if fold_direction == 'x' {
            (2 * fold_line - x, y)
        } else {
            (x, 2 * fold_line - y)
        };
        remaining_points.insert(point_after_fold);
    }

    remaining_points
}

fn print_points(points: HashSet<(usize, usize)>) {
    // Print the points on the paper
    let (max_x, max_y) = points
        .iter()
        .cloned()
        .fold((0, 0), |(m_x, m_y), (x, y)| (max(m_x, x), max(m_y, y)));
    let mut filled_paper = vec![vec!['.'; max_x + 1]; max_y + 1];
    for (x, y) in points {
        filled_paper[y][x] = '#';
    }
    println!("After all fold instructions were executed, the paper shows the following:");
    for y in 0..=max_y {
        println!("{:?}", filled_paper[y])
    }
}

fn main() {
    let (points, fold_instruction) = parse_input_data(input_data());

    // Solution for puzzle 1
    let (first_fold_direction, first_fold_line) = fold_instruction[0];
    let points_after_first_fold = fold_paper(&points, first_fold_direction, first_fold_line);
    println!(
        "The paper shows {} points after the first fold",
        points_after_first_fold.len()
    );

    // Solution for puzzle 2
    let mut points = points;
    for (fold_direction, fold_line) in fold_instruction {
        points = fold_paper(&points, fold_direction, fold_line);
    }
    print_points(points);
}
