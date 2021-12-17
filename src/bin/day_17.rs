fn input_data() -> &'static str {
    "target area: x=20..30, y=-10..-5"
}

fn parse_input_data(target_coordinates: &str) -> (i32, i32, i32, i32) {
    // Read bounds coordinates of the target area from input
    let axis = target_coordinates.split_once(": ").unwrap().1.split(", ");
    let mut ranges = axis
        .map(|a| a.split_once('=').unwrap().1)
        .map(|a| a.split_once("..").unwrap())
        .map(|(c_0, c_1)| (c_0.parse::<i32>().unwrap(), c_1.parse::<i32>().unwrap()));
    let (x_0, x_1) = ranges.next().unwrap();
    let (y_0, y_1) = ranges.next().unwrap();
    (x_0, x_1, y_0, y_1)
}

fn follow_trajectory(
    velocity: (i32, i32),
    target: (i32, i32, i32, i32),
) -> Option<(i32, i32, i32)> {
    // Simulate the given trajectory in discrete steps and check if the target area is found
    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;
    let (mut v_x, mut v_y) = velocity;
    let (t_x_0, t_x_1, t_y_0, t_y_1) = target;
    let right_bound = i32::max(t_x_0, t_x_1);
    let lower_bound = i32::min(t_y_0, t_y_1);
    let (t_x, t_y) = (t_x_0..=t_x_1, t_y_0..=t_y_1);
    while x <= right_bound && y >= lower_bound {
        x += v_x;
        y += v_y;
        v_x = match v_x {
            i32::MIN..=-1 => v_x + 1,
            1..=i32::MAX => v_x - 1,
            0 => 0,
        };
        v_y -= 1;
        max_y = i32::max(max_y, y);
        if t_x.contains(&x) && t_y.contains(&y) {
            return Some((velocity.0, velocity.1, max_y));
        }
    }

    None
}

fn find_trajectories(target: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    // Find all trajectories that reach the target area
    let mut trajectories = Vec::new();
    for v_x in 0..=(target.1) {
        let y_distance = i32::max(target.2.abs(), target.3.abs());
        for v_y in (-y_distance)..=y_distance {
            if let Some(trajectory) = follow_trajectory((v_x, v_y), target) {
                trajectories.push(trajectory);
            }
        }
    }
    trajectories
}

fn main() {
    let target_area = parse_input_data(input_data());
    let trajectories = find_trajectories(target_area);

    // Solution for puzzle 1
    let (v_x, v_y, max_y) = trajectories
        .iter()
        .max_by_key(|trajectory| trajectory.2)
        .unwrap();
    println!(
        "Trajectory ({}, {}) reached highest trajectory {}",
        v_x, v_y, max_y
    );

    // Solution for puzzle 2
    println!(
        "{} different trajectories are found to reach target area",
        trajectories.len()
    );
}
