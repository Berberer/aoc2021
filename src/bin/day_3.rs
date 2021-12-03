fn input_data() -> Vec<&'static str> {
    // Replace these measurements with a different set of measurements
    vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]
}

fn count_bits_in_measurement_column(
    column_index: usize,
    measurements: &Vec<&str>,
) -> (usize, usize) {
    // Count the occurrences of zeros and ones in a given column of the measurements
    let column_string = measurements
        .iter()
        .map(move |measurement_string| measurement_string.chars().nth(column_index).unwrap_or('-'))
        .collect::<String>();
    (
        column_string.matches('0').count(),
        column_string.matches('1').count(),
    )
}

fn calculate_gamma_epsilon_diagnostics(measurements: &Vec<&str>) -> (i32, i32) {
    let line_length = measurements[0].len();
    (0..line_length)
        .map(|index| count_bits_in_measurement_column(index, measurements))
        .rev()
        .enumerate()
        .fold((0, 0), |(gamma, epsilon), (index, (zeros, ones))| {
            if zeros > ones {
                (gamma, epsilon + 2i32.pow(index as u32))
            } else {
                (gamma + 2i32.pow(index as u32), epsilon)
            }
        })
}

fn calculate_air_diagnostics(measurements: &Vec<&str>, is_oxygen: bool) -> i32 {
    let line_length = measurements[0].len();
    let oxygen_measurement =
        (0..line_length).fold(measurements.clone(), |remaining_measurements, index| {
            if remaining_measurements.len() == 1 {
                remaining_measurements
            } else {
                let (zeros, ones) =
                    count_bits_in_measurement_column(index, &remaining_measurements);
                let filter_bit = match (is_oxygen, zeros, ones) {
                    (true, zeros, ones) if zeros > ones => '0',
                    (true, _, _) => '1',
                    (false, zeros, ones) if zeros > ones => '1',
                    (false, _, _) => '0',
                };
                remaining_measurements
                    .iter()
                    .cloned()
                    .filter(|measurement_string| {
                        measurement_string.chars().nth(index).unwrap_or('-') == filter_bit
                    })
                    .collect()
            }
        })[0];
    i32::from_str_radix(oxygen_measurement, 2).unwrap()
}

fn main() {
    let measurements = input_data();

    // Solution for puzzle 1
    let (gamma, epsilon) = calculate_gamma_epsilon_diagnostics(&measurements);
    println!(
        "Gamma: {} - Epsilon: {} (Multiplied: {})",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // Solution for puzzle 2
    let oxygen = calculate_air_diagnostics(&measurements, true);
    let co2 = calculate_air_diagnostics(&measurements, false);
    println!(
        "Oxygen: {} - CO2: {} (Multiplied: {})",
        oxygen,
        co2,
        oxygen * co2
    );
}
