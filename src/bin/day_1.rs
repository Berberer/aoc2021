fn input_data() -> Vec<i32> {
    // Replace these numbers with a different set of input numbers
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
}

fn count_increasing_numbers(numbers: &Vec<i32>) -> i32 {
    numbers
        .windows(2)
        .map(|pair| if pair[1] > pair[0] { 1 } else { 0 })
        .sum()
}

fn count_increasing_window_sums(numbers: &Vec<i32>) -> i32 {
    count_increasing_numbers(
        &numbers
            .windows(3)
            .map(|triple| triple.iter().sum())
            .collect(),
    )
}

fn main() {
    let numbers = input_data();
    // Solution for puzzle 1
    println!(
        "Amount of increasing numbers: {}",
        count_increasing_numbers(&numbers)
    );
    // Solution for puzzle 2
    println!(
        "Amount of increasing window sums: {}",
        count_increasing_window_sums(&numbers)
    );
}
