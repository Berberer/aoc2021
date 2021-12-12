use std::collections::HashMap;

fn input_data() -> Vec<&'static str> {
    vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
}

enum CaveType {
    Start,
    Big,
    Small,
    End,
}

fn get_cave_type(id: &'static str) -> CaveType {
    // Get the type of the cave based on the id
    match id {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        s if s.chars().all(|c| c.is_uppercase()) => CaveType::Big,
        _ => CaveType::Small,
    }
}

fn create_cave_system(input_paths: Vec<&'static str>) -> HashMap<&'static str, Vec<&'static str>> {
    // Create an adjacency list for each cave from the input data
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    for path in input_paths {
        let (start, end) = path.split_once('-').unwrap();

        if !paths.contains_key(start) {
            paths.insert(start, Vec::new());
        }
        if !paths.contains_key(end) {
            paths.insert(end, Vec::new());
        }

        paths.get_mut(start).unwrap().push(end);
        paths.get_mut(end).unwrap().push(start);
    }

    paths
}

fn create_paths(
    cave_system: &HashMap<&'static str, Vec<&'static str>>,
    second_small_cave_visit: bool,
) -> Vec<Vec<&'static str>> {
    // Start at the end cave and search all valid paths to the start cave
    let mut search_front: Vec<(Vec<&str>, bool)> = Vec::new();
    for predecessor in cave_system.get("end").unwrap().clone() {
        search_front.push((vec!["end", predecessor], false));
    }
    let mut valid_paths = Vec::new();
    while !search_front.is_empty() {
        let mut incomplete_paths: Vec<(Vec<&str>, bool)> = Vec::new();

        for (path, second_visit_done) in search_front {
            let current_cave = path.last().unwrap().clone();
            if current_cave == "start" {
                valid_paths.push(path.iter().cloned().rev().collect());
            } else {
                for predecessor in cave_system.get(current_cave).unwrap().clone() {
                    let mut new_path = path.clone();
                    new_path.push(predecessor);
                    match get_cave_type(predecessor) {
                        CaveType::Big | CaveType::Start => {
                            incomplete_paths.push((new_path, second_visit_done));
                        }
                        CaveType::Small if !path.contains(&predecessor) => {
                            incomplete_paths.push((new_path, second_visit_done));
                        }
                        CaveType::Small if second_small_cave_visit && !second_visit_done => {
                            incomplete_paths.push((new_path, true))
                        }
                        _ => {}
                    };
                }
            }
        }

        search_front = incomplete_paths;
    }

    valid_paths
}

fn main() {
    let cave_system = create_cave_system(input_data());

    // Solution for puzzle 1
    let paths = create_paths(&cave_system, false);
    println!(
        "Cave system has {} different paths from start to end without visiting a small cave twice",
        paths.len()
    );

    // Solution for puzzle 2
    let paths = create_paths(&cave_system, true);
    println!(
        "Cave system has {} different paths from start to end when a single small cave visit for the second time is allowed",
        paths.len()
    );
}
