use std::collections::{HashMap, HashSet};

fn input_data() -> Vec<&'static str> {
    vec![
        "--- scanner 0 ---",
        "404,-588,-901",
        "528,-643,409",
        "-838,591,734",
        "390,-675,-793",
        "-537,-823,-458",
        "-485,-357,347",
        "-345,-311,381",
        "-661,-816,-575",
        "-876,649,763",
        "-618,-824,-621",
        "553,345,-567",
        "474,580,667",
        "-447,-329,318",
        "-584,868,-557",
        "544,-627,-890",
        "564,392,-477",
        "455,729,728",
        "-892,524,684",
        "-689,845,-530",
        "423,-701,434",
        "7,-33,-71",
        "630,319,-379",
        "443,580,662",
        "-789,900,-551",
        "459,-707,401",
        "",
        "--- scanner 1 ---",
        "686,422,578",
        "605,423,415",
        "515,917,-361",
        "-336,658,858",
        "95,138,22",
        "-476,619,847",
        "-340,-569,-846",
        "567,-361,727",
        "-460,603,-452",
        "669,-402,600",
        "729,430,532",
        "-500,-761,534",
        "-322,571,750",
        "-466,-666,-811",
        "-429,-592,574",
        "-355,545,-477",
        "703,-491,-529",
        "-328,-685,520",
        "413,935,-424",
        "-391,539,-444",
        "586,-435,557",
        "-364,-763,-893",
        "807,-499,-711",
        "755,-354,-619",
        "553,889,-390",
        "",
        "--- scanner 2 ---",
        "649,640,665",
        "682,-795,504",
        "-784,533,-524",
        "-644,584,-595",
        "-588,-843,648",
        "-30,6,44",
        "-674,560,763",
        "500,723,-460",
        "609,671,-379",
        "-555,-800,653",
        "-675,-892,-343",
        "697,-426,-610",
        "578,704,681",
        "493,664,-388",
        "-671,-858,530",
        "-667,343,800",
        "571,-461,-707",
        "-138,-166,112",
        "-889,563,-600",
        "646,-828,498",
        "640,759,510",
        "-630,509,768",
        "-681,-892,-333",
        "673,-379,-804",
        "-742,-814,-386",
        "577,-820,562",
        "",
        "--- scanner 3 ---",
        "-589,542,597",
        "605,-692,669",
        "-500,565,-823",
        "-660,373,557",
        "-458,-679,-417",
        "-488,449,543",
        "-626,468,-788",
        "338,-750,-386",
        "528,-832,-391",
        "562,-778,733",
        "-938,-730,414",
        "543,643,-506",
        "-524,371,-870",
        "407,773,750",
        "-104,29,83",
        "378,-903,-323",
        "-778,-728,485",
        "426,699,580",
        "-438,-605,-362",
        "-469,-447,-387",
        "509,732,623",
        "647,635,-688",
        "-868,-804,481",
        "614,-800,639",
        "595,780,-596",
        "",
        "--- scanner 4 ---",
        "727,592,562",
        "-293,-554,779",
        "441,611,-461",
        "-714,465,-776",
        "-743,427,-804",
        "-660,-479,-426",
        "832,-632,460",
        "927,-485,-438",
        "408,393,-506",
        "466,436,-512",
        "110,16,151",
        "-258,-428,682",
        "-393,719,612",
        "-211,-452,876",
        "808,-476,-593",
        "-575,615,604",
        "-485,667,467",
        "-680,325,-822",
        "-627,-443,-432",
        "872,-547,-609",
        "833,512,582",
        "807,604,487",
        "839,-516,451",
        "891,-625,532",
        "-652,-548,-490",
        "30,-46,-14",
    ]
}

fn parse_input_data(input_data: Vec<&str>) -> Vec<Scanner> {
    // Parse beacon measurements from input string and create Scanner structs accordingly
    let mut scanner_measurements = Vec::new();
    let mut current_scanner_measurements = Vec::new();
    let mut scanner_number = 0;
    for input_line in input_data {
        if input_line.contains("---") {
            if !current_scanner_measurements.is_empty() {
                scanner_measurements
                    .push(Scanner::new(scanner_number, current_scanner_measurements));
                scanner_number += 1;
                current_scanner_measurements = Vec::new();
            }
        } else if !input_line.is_empty() {
            let mut point = input_line
                .split(',')
                .map(|coordinate| coordinate.parse::<i32>().unwrap());
            current_scanner_measurements.push((
                point.next().unwrap(),
                point.next().unwrap(),
                point.next().unwrap(),
            ));
        }
    }
    if !current_scanner_measurements.is_empty() {
        scanner_measurements.push(Scanner::new(scanner_number, current_scanner_measurements));
    }
    scanner_measurements
}

#[derive(Clone)]
struct Scanner {
    number: i32,
    position: (i32, i32, i32),
    beacon_measurements: Vec<(i32, i32, i32)>,
}

impl Scanner {
    fn new(number: i32, beacon_measurements: Vec<(i32, i32, i32)>) -> Scanner {
        Scanner {
            number,
            position: (0, 0, 0),
            beacon_measurements,
        }
    }

    fn fix_position(&self, position: (i32, i32, i32), rotation_id: i32) -> Scanner {
        // Set the position of a scanner after alignment and translate all measurement vectors
        println!(
            "Fixed position of Scanner {} to {:?}",
            self.number, position
        );
        Scanner {
            number: self.number,
            position: position,
            beacon_measurements: self
                .beacon_measurements
                .iter()
                .cloned()
                .map(|measurement| add_vectors(rotate_vector(rotation_id, &measurement), position))
                .collect::<Vec<(i32, i32, i32)>>(),
        }
    }
}

fn create_vector_rotations(vec: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    // Create all possible 90° vector rotations
    (0..24)
        .map(|rotation_id| rotate_vector(rotation_id, &vec))
        .collect()
}

fn rotate_vector(rotation_id: i32, vector: &(i32, i32, i32)) -> (i32, i32, i32) {
    // Execute a vector rotation specified by the specific rotation identifier
    match (rotation_id, vector.clone()) {
        (1, (x, y, z)) => (x, -y, -z),
        (2, (x, y, z)) => (x, -z, y),
        (3, (x, y, z)) => (x, z, -y),
        (4, (x, y, z)) => (-x, -y, z),
        (5, (x, y, z)) => (-x, -z, -y),
        (6, (x, y, z)) => (-x, y, -z),
        (7, (x, y, z)) => (-x, z, y),
        (8, (x, y, z)) => (y, z, x),
        (9, (x, y, z)) => (y, -x, z),
        (10, (x, y, z)) => (y, -z, -x),
        (11, (x, y, z)) => (y, x, -z),
        (12, (x, y, z)) => (-y, x, z),
        (13, (x, y, z)) => (-y, -z, x),
        (14, (x, y, z)) => (-y, -x, -z),
        (15, (x, y, z)) => (-y, z, -x),
        (16, (x, y, z)) => (z, x, y),
        (17, (x, y, z)) => (z, y, -x),
        (18, (x, y, z)) => (z, -x, -y),
        (19, (x, y, z)) => (z, -y, x),
        (20, (x, y, z)) => (-z, y, x),
        (21, (x, y, z)) => (-z, x, -y),
        (22, (x, y, z)) => (-z, -y, -x),
        (23, (x, y, z)) => (-z, -x, y),
        (_, (x, y, z)) => (x, y, z),
    }
}

fn add_vectors(v_1: (i32, i32, i32), v_2: (i32, i32, i32)) -> (i32, i32, i32) {
    (v_1.0 + v_2.0, v_1.1 + v_2.1, v_1.2 + v_2.2)
}

fn subtract_vectors(v_1: (i32, i32, i32), v_2: (i32, i32, i32)) -> (i32, i32, i32) {
    (v_1.0 - v_2.0, v_1.1 - v_2.1, v_1.2 - v_2.2)
}

fn get_vectors_between_measurements(
    measurements: &Vec<(i32, i32, i32)>,
) -> HashMap<(i32, i32, i32), (i32, i32, i32)> {
    // Create vectors between all combinations of two measurement vectors in all possible rotations
    let mut v = HashMap::new();
    for (i, m_1) in measurements.iter().cloned().enumerate() {
        for (j, m_2) in measurements.iter().cloned().enumerate() {
            if i != j {
                let rel_vec = subtract_vectors(m_2, m_1);
                for rot_v in create_vector_rotations(rel_vec) {
                    v.insert(rot_v, m_1.clone());
                }
            }
        }
    }
    v
}

fn get_common_measurements(
    measurements_1: &HashMap<(i32, i32, i32), (i32, i32, i32)>,
    measurements_2: &HashMap<(i32, i32, i32), (i32, i32, i32)>,
) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    // Check both scanners for (rotated) vectors between two measurement vectors
    let mut coordinate_1 = HashSet::new();
    let mut coordinate_2 = HashSet::new();
    measurements_1
        .keys()
        .cloned()
        .filter(|v| measurements_2.contains_key(v))
        .map(|v| {
            (
                measurements_1.get(&v).unwrap().clone(),
                measurements_2.get(&v).unwrap().clone(),
            )
        })
        .filter(|(v_1, v_2)| {
            let add_pair = !(coordinate_1.contains(v_1) && coordinate_2.contains(v_2));
            coordinate_1.insert(v_1.clone());
            coordinate_2.insert(v_2.clone());
            add_pair
        })
        .collect()
}

fn align_scanner(fixed_scanner: &Vec<Scanner>, scanner_for_alignment: &Scanner) -> Option<Scanner> {
    // Search a scanner that has enough shared beacon measurement vectors for alignment via the shared vectors
    for scanner in fixed_scanner {
        let common_measurements = get_common_measurements(
            &get_vectors_between_measurements(&(scanner.beacon_measurements.clone())),
            &get_vectors_between_measurements(&(scanner_for_alignment.beacon_measurements)),
        );

        if common_measurements.len() >= 12 {
            for rotation_id in 0..24 {
                let distances = common_measurements
                    .iter()
                    .cloned()
                    .map(|(m_1, m_2)| (m_1, rotate_vector(rotation_id, &m_2)))
                    .map(|(m_1, m_2)| subtract_vectors(m_1, m_2))
                    .collect::<Vec<(i32, i32, i32)>>();
                if distances.windows(2).all(|d| d[0] == d[1]) {
                    return Some(scanner_for_alignment.fix_position(distances[0], rotation_id));
                }
            }
        }
    }
    None
}

fn manhattan_distance((x_1, y_1, z_1): (i32, i32, i32), (x_2, y_2, z_2): (i32, i32, i32)) -> i32 {
    (x_2 - x_1).abs() + (y_2 - y_1).abs() + (z_2 - z_1).abs()
}

fn main() {
    let scanners = parse_input_data(input_data());

    let mut fixed_scanners = vec![scanners[0].clone()];
    let mut scanners_to_be_aligned = scanners[1..].to_vec();
    scanners_to_be_aligned.reverse();

    // Align all scanners progressively starting with fixing the position of scanner o to (0, 0, 0)
    while let Some(scanner_to_be_aligned) = scanners_to_be_aligned.pop() {
        if let Some(aligned_scanner) = align_scanner(&fixed_scanners, &scanner_to_be_aligned) {
            println!(
                "{} Scanners are left for alignment",
                scanners_to_be_aligned.len()
            );
            fixed_scanners.push(aligned_scanner);
        } else {
            scanners_to_be_aligned.insert(0, scanner_to_be_aligned);
        }
    }

    // Solution for puzzle 1
    let mut beacons = HashSet::new();
    for scanner in fixed_scanners.clone() {
        beacons.extend(scanner.beacon_measurements);
    }
    println!("The probe released {} beacons", beacons.len());

    // Solution for puzzle 2
    let mut max_scanner_distance = 0;
    for (i, scanner_1) in fixed_scanners.iter().enumerate() {
        for (j, scanner_2) in fixed_scanners.iter().enumerate() {
            if i != j {
                let scanner_distance = manhattan_distance(scanner_1.position, scanner_2.position);
                max_scanner_distance = i32::max(max_scanner_distance, scanner_distance);
            }
        }
    }
    println!(
        "The maximum distance between two scanners is {}",
        max_scanner_distance
    );
}
