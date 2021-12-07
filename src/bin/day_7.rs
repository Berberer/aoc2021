fn input_data() -> &'static str {
    // Replace these crab ship positions with different numbers
    "16,1,2,0,4,2,7,1,2,14"
}

fn parse_input_data(input_data: &str) -> Vec<i32> {
    // Get sorted numeric representation from input string
    // Sorting for easier access to min, max, and median
    let mut positions = input_data
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    positions.sort();
    positions
}

fn calculate_linear_fuel_cost(target_position: i32, positions: &Vec<i32>) -> i32 {
    // Cost for each position is the distance to the target position
    positions.iter().fold(0, |cost, position| {
        cost + (position - target_position).abs()
    })
}

fn calculate_sum_fuel_cost(target_position: i32, positions: &Vec<i32>) -> i32 {
    // Cost for each position is the sum from 1 to the distance to the target position
    // Sum is calculated with Gauss sum
    positions.iter().fold(0, |cost, position| {
        let target_distance = (position - target_position).abs();
        cost + (target_distance * (target_distance + 1)) / 2
    })
}

fn find_optimal_position(
    positions: &Vec<i32>,
    cost_function: fn(i32, &Vec<i32>) -> i32,
) -> (i32, i32) {
    // Cost function is convex and can be found with linearly searching in the correct direction
    // Any encountered local optimum will be the global optimum
    let min = positions[0];
    let max = positions[positions.len() - 1];

    // Start with median as starting point for optimum search
    let mut optimum_position = positions[positions.len() / 2];
    let mut optimum_cost = cost_function(optimum_position, positions);

    // Check for local optimum at the starting point
    let predecessor_cost = cost_function(optimum_position - 1, positions);
    let successor_cost = cost_function(optimum_position + 1, positions);
    if optimum_position < predecessor_cost && optimum_cost < successor_cost {
        return (optimum_position, optimum_cost);
    }

    // Determine optimization search direction
    let step = if optimum_cost > predecessor_cost {
        optimum_position = optimum_position - 1;
        optimum_cost = predecessor_cost;
        -1
    } else {
        optimum_position = optimum_position + 1;
        optimum_cost = successor_cost;
        1
    };

    // Search stepwise for local optimum
    while optimum_position >= min && optimum_position <= max {
        let next_cost = cost_function(optimum_position + step, positions);
        if next_cost < optimum_cost {
            optimum_position = optimum_position + step;
            optimum_cost = next_cost;
        } else {
            break;
        }
    }

    (optimum_position, optimum_cost)
}

fn main() {
    let ship_positions = parse_input_data(input_data());

    // Solution for puzzle 1
    let (optimum_position, optimum_cost) =
        find_optimal_position(&ship_positions, calculate_linear_fuel_cost);
    println!(
        "Optimal Position linear fuel costs is {} with fuel cost {}",
        optimum_position, optimum_cost
    );

    // Solution for puzzle 2
    let (optimum_position, optimum_cost) =
        find_optimal_position(&ship_positions, calculate_sum_fuel_cost);
    println!(
        "Optimal Position with summed fuel costs is {} with fuel cost {}",
        optimum_position, optimum_cost
    );
}
