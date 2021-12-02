fn input_data() -> Vec<&'static str> {
    // Replace these commands with a different set of movement commands
    vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]
}

fn parse_movement_commands<'a>(movements: &'a Vec<&str>) -> Vec<(&'a str, i32)> {
    // Split command str into direction str and units
    movements
        .iter()
        .map(|movement| movement.split_once(' ').unwrap_or(("forward", "0")))
        .map(|movement| (movement.0, movement.1.parse::<i32>().unwrap_or(1)))
        .collect()
}

fn execute_submarine_movements(movements: &Vec<&str>) -> (i32, i32) {
    let movements = parse_movement_commands(movements);

    movements
        .iter()
        .fold((0, 0), |position, movement| match (position, movement) {
            ((x, y), ("forward", n)) => (x + n, y),
            ((x, y), ("up", n)) => (x, y - n),
            ((x, y), ("down", n)) => (x, y + n),
            _ => position,
        })
}

fn execute_submarine_movements_with_aim(movements: &Vec<&str>) -> (i32, i32) {
    let movements = parse_movement_commands(movements);

    let position =
        movements
            .iter()
            .fold((0, 0, 0), |position, movement| match (position, movement) {
                ((x, y, a), ("forward", n)) => (x + n, y + a * n, a),
                ((x, y, a), ("up", n)) => (x, y, a - n),
                ((x, y, a), ("down", n)) => (x, y, a + n),
                _ => position,
            });

    (position.0, position.1)
}

fn main() {
    let movement_commands = input_data();

    // Solution for puzzle 1
    let position = execute_submarine_movements(&movement_commands);
    println!(
        "Submarine moved to position: {:?} (Multiplication: {})",
        position,
        position.0 * position.1
    );

    // Solution for puzzle 2
    let position = execute_submarine_movements_with_aim(&movement_commands);
    println!(
        "Submarine moved with aim to position: {:?} (Multiplication: {})",
        position,
        position.0 * position.1
    );
}
