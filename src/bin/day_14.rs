use std::collections::HashMap;

fn input_data() -> Vec<&'static str> {
    vec![
        "NNCB",
        "",
        "CH -> B",
        "HH -> N",
        "CB -> H",
        "NH -> C",
        "HB -> C",
        "HC -> B",
        "HN -> C",
        "NN -> C",
        "BH -> H",
        "NC -> B",
        "NB -> B",
        "BN -> B",
        "BB -> N",
        "BC -> B",
        "CC -> N",
        "CN -> C",
    ]
}

fn parse_input_data(input_data: Vec<&str>) -> (String, HashMap<String, (String, String, char)>) {
    // Split into input text and replacement rules
    // Replacement rules are mappings from pairs to the two new pairs and the new char
    let start_text = String::from(input_data[0]);
    let replacements = input_data[2..]
        .iter()
        .map(|replacement| replacement.split_once(" -> ").unwrap())
        .map(|(pattern, replacement)| {
            (
                String::from(pattern),
                (
                    format!("{}{}", pattern.chars().nth(0).unwrap(), replacement),
                    format!("{}{}", replacement, pattern.chars().last().unwrap()),
                    replacement.chars().nth(0).unwrap(),
                ),
            )
        })
        .collect();

    (start_text, replacements)
}

fn initial_chars_count(text: &String) -> HashMap<char, i64> {
    // Count chars in the input text
    let mut counter = HashMap::new();
    for c in text.chars() {
        *counter.entry(c).or_default() += 1;
    }
    counter
}

fn initial_pairs_count(text: &String) -> HashMap<String, i64> {
    // Count pairs in the input text
    let mut pair_counter = HashMap::new();
    for i in 0..(text.len() - 1) {
        let pair = String::from(&text[i..i + 2]);
        *pair_counter.entry(pair).or_default() += 1;
    }
    pair_counter
}

fn execute_replacement_step(
    pair_counter: HashMap<String, i64>,
    char_counter: HashMap<char, i64>,
    replacement_rules: &HashMap<String, (String, String, char)>,
) -> (HashMap<String, i64>, HashMap<char, i64>) {
    // Get current pair counts and char counts
    // Execute all applicable replacements and update counters
    // For all pairs without a replacement rule, keep the current count
    let mut pair_counter_with_replacements = HashMap::new();
    let mut char_counter_with_replacements = char_counter;

    for (pair, count) in pair_counter {
        if let Some((pair_1, pair_2, new_char)) = replacement_rules.get(&pair) {
            *pair_counter_with_replacements
                .entry(pair_1.clone())
                .or_default() += count;
            *pair_counter_with_replacements
                .entry(pair_2.clone())
                .or_default() += count;
            *char_counter_with_replacements.entry(*new_char).or_default() += count;
        } else {
            *pair_counter_with_replacements
                .entry(pair.clone())
                .or_default() += count;
        }
    }

    (
        pair_counter_with_replacements,
        char_counter_with_replacements,
    )
}

fn execute_replacements(
    text: &String,
    replacement_steps: i64,
    replacements: &HashMap<String, (String, String, char)>,
) -> HashMap<char, i64> {
    // Iteratively execute replacements and update counters
    let mut pair_counter = initial_pairs_count(text);
    let mut char_counter = initial_chars_count(text);
    for _ in 0..replacement_steps {
        let counters = execute_replacement_step(pair_counter, char_counter, replacements);
        pair_counter = counters.0;
        char_counter = counters.1;
    }
    char_counter
}

fn get_min_and_max(counter: HashMap<char, i64>) -> (i64, i64) {
    // Get min and max count of a hashmap
    (
        *counter.values().min().unwrap(),
        *counter.values().max().unwrap(),
    )
}

fn main() {
    let (start_text, replacements) = parse_input_data(input_data());

    // Solution for puzzle 1
    let char_counter = execute_replacements(&start_text, 10, &replacements);
    let (min_char_count, max_char_count) = get_min_and_max(char_counter);
    println!(
        "Difference of the most common and least common char occurrences after 10 replacements is: {}",
        max_char_count - min_char_count
    );

    // Solution for puzzle 2
    let char_counter = execute_replacements(&start_text, 40, &replacements);
    let (min_char_count, max_char_count) = get_min_and_max(char_counter);
    println!(
        "Difference of the most common and least common char occurrences after 40 replacements is: {}",
        max_char_count - min_char_count
    );
}
