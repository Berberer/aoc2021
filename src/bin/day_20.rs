fn input_data() -> Vec<&'static str> {
    vec![
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
        "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
        ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
        ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
        ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
        "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
        "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        "",
        "#..#.",
        "#....",
        "##..#",
        "..#..",
        "..###",
    ]
}

fn parse_input_data(input_data: Vec<&str>) -> (Vec<bool>, Vec<Vec<bool>>) {
    // Create the enhancement algorithm data and the image pixels from the input lines
    let mut enhancement = Vec::new();
    let mut image = Vec::new();
    let mut enhancement_finished = false;
    for line in input_data
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
    {
        if enhancement_finished {
            image.push(line);
        } else {
            if line.is_empty() {
                enhancement_finished = true;
            } else {
                enhancement.extend(line);
            }
        }
    }
    (enhancement, image)
}

fn get_enhancement_value(line: Vec<bool>, enhancements: &Vec<bool>) -> bool {
    // Lookup of the enhancement algorithm value based on the bit values of the surrounding pixels
    let enhancement_index = usize::from_str_radix(
        line.iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    enhancements[enhancement_index]
}

fn get_image_value(
    x: i32,
    y: i32,
    image: &Vec<Vec<bool>>,
    enhancements: &Vec<bool>,
    enhancement_step: i32,
) -> bool {
    // Get the current image value for the actual image and alternating value for the 'infinity' values
    if (0..(image.len() as i32)).contains(&y) {
        if (0..(image[0].len() as i32)).contains(&x) {
            image[y as usize][x as usize]
        } else {
            if enhancements[0] && enhancements[0] != enhancements[enhancements.len() - 1] {
                enhancement_step % 2 == 1
            } else {
                false
            }
        }
    } else {
        if enhancements[0] && enhancements[0] != enhancements[enhancements.len() - 1] {
            enhancement_step % 2 == 1
        } else {
            false
        }
    }
}

fn enhance_image(
    image: Vec<Vec<bool>>,
    enhancements: &Vec<bool>,
    enhancement_step: i32,
) -> Vec<Vec<bool>> {
    // Perform a single enhancement step by enhancing each pixel based on the bit values of its surrounding pixels
    // If the first and the last enhancements bits are different and the first bit is true:
    // Then pixels in the 'infinity' alternate between lit and dark between each step
    // This is handled inside of get_image_value
    let mut enhanced_image = Vec::new();
    for y in (-1)..=((image.len()) as i32) {
        let mut enhanced_image_line = Vec::new();
        for x in (-1)..=((image[0].len()) as i32) {
            let image_pixel_window = vec![
                get_image_value(x - 1, y - 1, &image, enhancements, enhancement_step),
                get_image_value(x, y - 1, &image, enhancements, enhancement_step),
                get_image_value(x + 1, y - 1, &image, enhancements, enhancement_step),
                get_image_value(x - 1, y, &image, enhancements, enhancement_step),
                get_image_value(x, y, &image, enhancements, enhancement_step),
                get_image_value(x + 1, y, &image, enhancements, enhancement_step),
                get_image_value(x - 1, y + 1, &image, enhancements, enhancement_step),
                get_image_value(x, y + 1, &image, enhancements, enhancement_step),
                get_image_value(x + 1, y + 1, &image, enhancements, enhancement_step),
            ];
            enhanced_image_line.push(get_enhancement_value(image_pixel_window, enhancements));
        }
        enhanced_image.push(enhanced_image_line);
    }
    enhanced_image
}

fn multiple_enhancement_steps(
    enhancement_steps: i32,
    image: Vec<Vec<bool>>,
    enhancements: &Vec<bool>,
) -> Vec<Vec<bool>> {
    let mut enhanced_image = image;
    for i in 0..enhancement_steps {
        enhanced_image = enhance_image(enhanced_image, enhancements, i);
    }
    enhanced_image
}

fn main() {
    let (enhancements, image) = parse_input_data(input_data());

    // Solution for puzzle 1
    let enhanced_image = multiple_enhancement_steps(2, image.clone(), &enhancements);
    let number_of_lit_pixels: i32 = enhanced_image
        .iter()
        .flatten()
        .map(|pixel| if *pixel { 1 } else { 0 })
        .sum();
    println!(
        "Number of lit image pixels after 2 enhancement_steps: {}",
        number_of_lit_pixels
    );

    // Solution for puzzle 2
    let enhanced_image = multiple_enhancement_steps(50, image.clone(), &enhancements);
    let number_of_lit_pixels: i32 = enhanced_image
        .iter()
        .flatten()
        .map(|pixel| if *pixel { 1 } else { 0 })
        .sum();
    println!(
        "Number of lit image pixels after 50 enhancement_steps: {}",
        number_of_lit_pixels
    );
}
