use std::collections::{HashMap, HashSet};

fn input_data() -> Vec<&'static str> {
    // Replace these digits with a different set
    vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ]
}

fn parse_input_data(input: Vec<&str>) -> Vec<(Vec<&str>, Vec<&str>)> {
    // Separate line into input signals and outputs
    // Both are splitted into the indiviual digits
    input
        .iter()
        .map(|input_line| input_line.split_once('|').unwrap())
        .map(|(signal_line, output_line)| {
            (
                signal_line.trim().split(' ').collect(),
                output_line.trim().split(' ').collect(),
            )
        })
        .collect()
}

fn get_digits_with_sizes(digits: &Vec<&'static str>, sizes: &Vec<usize>) -> Vec<&'static str> {
    // Helper function to find digits with certain sizes
    digits
        .iter()
        .cloned()
        .filter(|digit| sizes.contains(&digit.len()))
        .collect()
}

fn main() {
    let inputs = parse_input_data(input_data());

    // Solution for puzzle 1
    let unique_output_digits = count_output_digits_with_unique_sizes(&inputs);
    println!(
        "Number of output digits with a unique length: {}",
        unique_output_digits
    );

    // Solution for puzzle 2
    let sum_of_outputs = sum_of_outputs(&inputs);
    println!("Sum of output digits : {}", sum_of_outputs);
}

// Puzzle 1 //

fn count_output_digits_with_unique_sizes(
    input: &Vec<(Vec<&'static str>, Vec<&'static str>)>,
) -> usize {
    // Filter and count the digits with unique sizes (Digit->Size: 1->2, 7->3, 4->4, 8->7)
    let unique_sizes = vec![2, 3, 4, 7];
    input
        .iter()
        .map(|(_, outputs)| get_digits_with_sizes(outputs, &unique_sizes))
        .flatten()
        .count()
}

// Puzzle 2 //

fn sort_digit_chars(digit: &str) -> String {
    // Sort the chars of a digit and combine to String for lookup
    let mut digit_chars: Vec<char> = digit.chars().collect();
    digit_chars.sort_by(|a, b| b.cmp(a));
    digit_chars.reverse();
    String::from_iter(&digit_chars)
}

fn create_decoding_dict(input_signals: &Vec<&'static str>) -> HashMap<String, char> {
    // Deduct decoding mapping for digit chars based on the input signals
    let mut decoding_dict = HashMap::new();

    // At first, find the digits with the unique sizes (Digit->Size: 1->2, 7->3, 4->4, 8->7)
    // Add their digits to the decoding dictionary
    // Additionally, create a hash set of chars for each of them to perform set operations
    let digits_one = input_signals.iter().find(|d| d.len() == 2).unwrap();
    let digit_set_one = digits_one.chars().collect::<HashSet<char>>();
    decoding_dict.insert(sort_digit_chars(digits_one), '1');

    let digits_four = input_signals.iter().find(|d| d.len() == 4).unwrap();
    let digit_set_four = digits_four.chars().collect::<HashSet<char>>();
    decoding_dict.insert(sort_digit_chars(digits_four), '4');

    let digits_seven = input_signals.iter().find(|d| d.len() == 3).unwrap();
    let digit_set_seven = digits_seven.chars().collect::<HashSet<char>>();
    decoding_dict.insert(sort_digit_chars(digits_seven), '7');

    let digits_eight = input_signals.iter().find(|d| d.len() == 7).unwrap();
    let digit_set_eight = digits_eight.chars().collect::<HashSet<char>>();
    decoding_dict.insert(sort_digit_chars(digits_eight), '8');

    // Encoded a is the set difference of 7 (a, c, f) and 1 (c, f)
    let encoded_a = *digit_set_seven.difference(&digit_set_one).next().unwrap();
    let a_set = [encoded_a].iter().cloned().collect::<HashSet<char>>();

    // Create two candidate sets with two candidates each to distinguish the remaining numbers
    // Candidates c + f: Take from encoded 1
    let candidates_c_f = digit_set_one;
    // Candidates e + g: Difference of 8 (a, b, c, d, e, f, g) and the union of 4 (b, c, d, f) and a
    let candidates_e_g = digit_set_eight
        .difference(
            &digit_set_four
                .union(&a_set)
                .cloned()
                .collect::<HashSet<char>>(),
        )
        .cloned()
        .collect::<HashSet<char>>();

    // Remaining digits:
    // Digits with size 5:
    // 2 (a, c, d, e, g)
    // 3 (a, c, d, f, g)
    // 5 (a, b, d, f, g)
    // Digits with size 6:
    // 0 (a, b, c, e, f, g)
    // 6 (a, b, d, e, f, g)
    // 9 (a, b, c, d, f, g)
    // They can be distinguished via the combination of:
    // 1) digit size
    // 2) intersection with c + f
    // 3) intersection with e + g
    let remaining_digits: Vec<(HashSet<char>, String, usize)> =
        get_digits_with_sizes(input_signals, &vec![5, 6])
            .iter()
            .map(|digit| {
                (
                    digit.chars().collect::<HashSet<char>>(),
                    sort_digit_chars(digit),
                    digit.len(),
                )
            })
            .collect();
    for (digit_chars_set, digit_key, digit_len) in remaining_digits {
        match (
            digit_len,
            digit_chars_set.intersection(&candidates_c_f).count(),
            digit_chars_set.intersection(&candidates_e_g).count(),
        ) {
            (5, 1, 2) => decoding_dict.insert(digit_key, '2'),
            (5, 2, 1) => decoding_dict.insert(digit_key, '3'),
            (5, 1, 1) => decoding_dict.insert(digit_key, '5'),
            (6, 2, 2) => decoding_dict.insert(digit_key, '0'),
            (6, 1, 2) => decoding_dict.insert(digit_key, '6'),
            (6, 2, 1) => decoding_dict.insert(digit_key, '9'),
            _ => None,
        };
    }

    decoding_dict
}

fn sum_of_outputs(input: &Vec<(Vec<&'static str>, Vec<&str>)>) -> i32 {
    // 1) Ceate a decoding dictionary based on the input signals for each line
    // 2) Use these dictionaries to decode the outputs of each line and create the output number
    // 3) Calculate the sum of all output line numbers
    input
        .iter()
        .map(|(input_signals, outputs)| {
            let decoding_dict = create_decoding_dict(input_signals);
            let output_number_string =
                String::from_iter(outputs.iter().cloned().map(|output_digit| {
                    decoding_dict.get(&sort_digit_chars(output_digit)).unwrap()
                }));
            output_number_string.parse::<i32>().unwrap()
        })
        .sum()
}
