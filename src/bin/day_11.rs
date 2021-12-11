fn input_data() -> Vec<&'static str> {
    vec![
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ]
}

fn parse_input_data(input: Vec<&str>) -> Vec<Vec<i32>> {
    // Create numerical octopus energy levels from input lines
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn generate_neighbors(x: i32, y: i32) -> Vec<(usize, usize)> {
    // Get all valid neighbor coordinates for a coordinate
    let mut neighbors = Vec::new();
    for n_x in (x - 1)..=(x + 1) {
        for n_y in (y - 1)..=(y + 1) {
            if (n_x != x || n_y != y) && (0..10).contains(&n_x) && (0..10).contains(&n_y) {
                neighbors.push((n_x as usize, n_y as usize));
            }
        }
    }
    neighbors
}

fn simulate_step(octupus_energy_levels: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, i32) {
    //Simulate the octopus simulation for one step

    // Increase all octopi by one
    let mut increased_levels = octupus_energy_levels
        .iter()
        .map(|line| line.iter().map(|energy| energy + 1).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    // Execute and count flashes as well as increase adjacent octopi energy levels
    // Flashed octopi are marked with a -1
    let mut flashed = true;
    let mut flashes = 0;
    while flashed {
        flashed = false;
        for x in 0..10 {
            for y in 0..10 {
                if increased_levels[y][x] > 9 && increased_levels[y][x] != -1 {
                    flashed = true;
                    flashes = flashes + 1;
                    increased_levels[y][x] = -1;
                    for (n_x, n_y) in generate_neighbors(x as i32, y as i32) {
                        if increased_levels[n_y][n_x] != -1 {
                            increased_levels[n_y][n_x] = increased_levels[n_y][n_x] + 1;
                        }
                    }
                }
            }
        }
    }

    // Reset all energy levels after a flash
    let levels_after_reset = increased_levels
        .iter()
        .map(|line| {
            line.iter()
                .cloned()
                .map(|level| if level == -1 { 0 } else { level })
                .collect()
        })
        .collect();
    (levels_after_reset, flashes)
}

fn main() {
    let octopus_starting_energy_levels = parse_input_data(input_data());

    // Solution for puzzle 1
    let mut octopus_flashes = 0;
    let mut octopus_current_energy_levels = octopus_starting_energy_levels;
    for _ in 0..100 {
        let step_results = simulate_step(octopus_current_energy_levels);
        octopus_current_energy_levels = step_results.0;
        octopus_flashes = octopus_flashes + step_results.1;
    }
    println!("Amount of flashes during 100 steps: {}", octopus_flashes);

    // Solution for puzzle 2
    let mut synchronized_flash_step = 1;
    let mut waiting = true;
    let mut octopus_current_energy_levels = parse_input_data(input_data());
    while waiting {
        octopus_current_energy_levels = simulate_step(octopus_current_energy_levels).0;
        if octopus_current_energy_levels
            .iter()
            .all(|line| line.iter().cloned().all(|energy_level| energy_level == 0))
        {
            waiting = false;
        } else {
            synchronized_flash_step = synchronized_flash_step + 1;
        }
    }
    println!(
        "Steps until a synchronized flash: {}",
        synchronized_flash_step
    );
}
