fn input_data() -> &'static str {
    // Replace these fish age representations with different numbers
    "3,4,3,1,2"
}

fn parse_input_data(input_data: &str) -> [i64; 9] {
    let mut population = [0; 9];
    // Get numeric representation from input string as amount of fish per timer state
    input_data
        .split(',')
        .map(|s| s.parse::<usize>().unwrap_or(0))
        .for_each(|n| population[n] = population[n] + 1);
    population
}

fn simulate_fish_population(number_of_days: i32, initial_population: &mut [i64; 9]) -> i64 {
    for _ in 0..number_of_days {
        let reproducing_fish = initial_population[0];
        // Reduce timer state for not reproducing fish
        for n in 0..8 {
            initial_population[n] = initial_population[n + 1];
        }
        // Handle reproducing fish
        initial_population[8] = reproducing_fish;
        initial_population[6] = initial_population[6] + reproducing_fish;
    }
    // Sum population size
    initial_population.iter().sum()
}

fn main() {
    // Solution for puzzle 1
    let mut initial_population = parse_input_data(input_data());
    let population_size = simulate_fish_population(80, &mut initial_population);
    println!("Size of fish population after 80 days: {}", population_size);

    // Solution for puzzle 2
    let mut initial_population = parse_input_data(input_data());
    let population_size = simulate_fish_population(256, &mut initial_population);
    println!(
        "Size of fish population after 256 days: {}",
        population_size
    );
}
