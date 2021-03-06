use std::ops::RangeInclusive;

fn input_data() -> Vec<&'static str> {
    vec![
        "on x=-5..47,y=-31..22,z=-19..33",
        "on x=-44..5,y=-27..21,z=-14..35",
        "on x=-49..-1,y=-11..42,z=-10..38",
        "on x=-20..34,y=-40..6,z=-44..1",
        "off x=26..39,y=40..50,z=-2..11",
        "on x=-41..5,y=-41..6,z=-36..8",
        "off x=-43..-33,y=-45..-28,z=7..25",
        "on x=-33..15,y=-32..19,z=-34..11",
        "off x=35..47,y=-46..-34,z=-11..5",
        "on x=-14..36,y=-6..44,z=-16..29",
        "on x=-57795..-6158,y=29564..72030,z=20435..90618",
        "on x=36731..105352,y=-21140..28532,z=16094..90401",
        "on x=30999..107136,y=-53464..15513,z=8553..71215",
        "on x=13528..83982,y=-99403..-27377,z=-24141..23996",
        "on x=-72682..-12347,y=18159..111354,z=7391..80950",
        "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709",
        "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856",
        "on x=-52752..22273,y=-49450..9096,z=54442..119054",
        "on x=-29982..40483,y=-108474..-28371,z=-24328..38471",
        "on x=-4958..62750,y=40422..118853,z=-7672..65583",
        "on x=55694..108686,y=-43367..46958,z=-26781..48729",
        "on x=-98497..-18186,y=-63569..3412,z=1232..88485",
        "on x=-726..56291,y=-62629..13224,z=18033..85226",
        "on x=-110886..-34664,y=-81338..-8658,z=8914..63723",
        "on x=-55829..24974,y=-16897..54165,z=-121762..-28058",
        "on x=-65152..-11147,y=22489..91432,z=-58782..1780",
        "on x=-120100..-32970,y=-46592..27473,z=-11695..61039",
        "on x=-18631..37533,y=-124565..-50804,z=-35667..28308",
        "on x=-57817..18248,y=49321..117703,z=5745..55881",
        "on x=14781..98692,y=-1341..70827,z=15753..70151",
        "on x=-34419..55919,y=-19626..40991,z=39015..114138",
        "on x=-60785..11593,y=-56135..2999,z=-95368..-26915",
        "on x=-32178..58085,y=17647..101866,z=-91405..-8878",
        "on x=-53655..12091,y=50097..105568,z=-75335..-4862",
        "on x=-111166..-40997,y=-71714..2688,z=5609..50954",
        "on x=-16602..70118,y=-98693..-44401,z=5197..76897",
        "on x=16383..101554,y=4615..83635,z=-44907..18747",
        "off x=-95822..-15171,y=-19987..48940,z=10804..104439",
        "on x=-89813..-14614,y=16069..88491,z=-3297..45228",
        "on x=41075..99376,y=-20427..49978,z=-52012..13762",
        "on x=-21330..50085,y=-17944..62733,z=-112280..-30197",
        "on x=-16478..35915,y=36008..118594,z=-7885..47086",
        "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456",
        "off x=2032..69770,y=-71013..4824,z=7471..94418",
        "on x=43670..120875,y=-42068..12382,z=-24787..38892",
        "off x=37514..111226,y=-45862..25743,z=-16714..54663",
        "off x=25699..97951,y=-30668..59918,z=-15349..69697",
        "off x=-44271..17935,y=-9516..60759,z=49131..112598",
        "on x=-61695..-5813,y=40978..94975,z=8655..80240",
        "off x=-101086..-9439,y=-7088..67543,z=33935..83858",
        "off x=18020..114017,y=-48931..32606,z=21474..89843",
        "off x=-77139..10506,y=-89994..-18797,z=-80..59318",
        "off x=8476..79288,y=-75520..11602,z=-96624..-24783",
        "on x=-47488..-1262,y=24338..100707,z=16292..72967",
        "off x=-84341..13987,y=2429..92914,z=-90671..-1318",
        "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188",
        "off x=-27365..46395,y=31009..98017,z=15428..76570",
        "off x=-70369..-16548,y=22648..78696,z=-1892..86821",
        "on x=-53470..21291,y=-120233..-33476,z=-44150..38147",
        "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
    ]
}

#[derive(Clone, Debug)]
struct Cuboid {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
    volume: i64,
}

impl Cuboid {
    fn new(
        x_range: RangeInclusive<i64>,
        y_range: RangeInclusive<i64>,
        z_range: RangeInclusive<i64>,
    ) -> Cuboid {
        // Create new cuboid and calculate volume
        let volume = (x_range.end() + 1 - x_range.start()).abs()
            * (y_range.end() + 1 - y_range.start()).abs()
            * (z_range.end() + 1 - z_range.start()).abs();
        Cuboid {
            x_range,
            y_range,
            z_range,
            volume,
        }
    }
}

fn parse_input_data(input_data: Vec<&str>) -> Vec<(bool, Cuboid)> {
    // Parse all initialization/reboot lines to check whether the cubes are meant to be on or off
    // and in which cuboid the cubes are affected
    let mut id = 0;
    input_data
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(value, ranges)| (value == "on", ranges.split(',').collect::<Vec<&str>>()))
        .map(|(value, ranges)| {
            (
                value,
                ranges
                    .iter()
                    .map(|range| range.split_once('=').unwrap().1)
                    .map(|range| range.split_once("..").unwrap())
                    .map(|(range_start, range_end)| {
                        (
                            range_start.parse::<i64>().unwrap(),
                            range_end.parse::<i64>().unwrap(),
                        )
                    })
                    .collect::<Vec<(i64, i64)>>(),
            )
        })
        .map(|(value, ranges)| {
            id += 1;
            (
                value,
                Cuboid::new(
                    ranges[0].0..=ranges[0].1,
                    ranges[1].0..=ranges[1].1,
                    ranges[2].0..=ranges[2].1,
                ),
            )
        })
        .collect()
}

fn axis_intersection(
    axis_1: &RangeInclusive<i64>,
    axis_2: &RangeInclusive<i64>,
) -> Option<RangeInclusive<i64>> {
    // Check if two axis intersect and if yes, return the intersection
    if axis_1.contains(&axis_2.start())
        || axis_1.contains(&axis_2.end())
        || axis_2.contains(&axis_1.start())
        || axis_2.contains(&axis_1.end())
    {
        Some(i64::max(*axis_1.start(), *axis_2.start())..=i64::min(*axis_1.end(), *axis_2.end()))
    } else {
        None
    }
}

fn cuboid_intersection(c_1: &Cuboid, c_2: &Cuboid) -> Option<Cuboid> {
    // Check if two cuboids intersect and if yes, return the intersection cuboid
    match (
        axis_intersection(&c_1.x_range, &c_2.x_range),
        axis_intersection(&c_1.y_range, &c_2.y_range),
        axis_intersection(&c_1.z_range, &c_2.z_range),
    ) {
        (Some(x_intersection), Some(y_intersection), Some(z_intersection)) => {
            Some(Cuboid::new(x_intersection, y_intersection, z_intersection))
        }
        _ => None,
    }
}

fn lit_cubes_after_procedure_steps(cuboid: &Cuboid, procedure_steps: &[(bool, Cuboid)]) -> i64 {
    // Check how many cubes are lit after a sequence of initialization/reboot procedure steps
    match procedure_steps {
        [] => 0, // No steps to execute => No lit cubes
        [(value, procedure_cuboid)] => {
            // A single step => Volume of the step cuboid, if the step is an 'on' step, 0 otherwise
            match (*value, cuboid_intersection(cuboid, procedure_cuboid)) {
                (true, Some(intersection_cuboid)) => intersection_cuboid.volume,
                (_, None) | (false, Some(_)) => 0,
            }
        }
        [previous_procedure_steps @ .., (current_procedure_step_value, current_procedure_step_cuboid)] =>
        {
            // At least two steps

            // How many cubes were lit before the current step
            let lit_reactor_cubes_before_current_step =
                lit_cubes_after_procedure_steps(cuboid, previous_procedure_steps);

            if let Some(intersection_cuboid) =
                cuboid_intersection(cuboid, current_procedure_step_cuboid)
            {
                // Calculate the lit cubes only inside of the intersection
                let lit_cubes_in_intersection_before_current_step =
                    lit_cubes_after_procedure_steps(&intersection_cuboid, previous_procedure_steps);

                // Current step is 'on'
                //    -> Complete intersection + Lit cubes outside of intersection (all lit cubes before current step - lit cubes only in intersection before current step)
                //       (Lit cubes inside of intersection have to be removed because they would be counted twice otherwise)
                // Current step is 'off'
                //    -> Lit cubes outside of intersection (all lit cubes before current step - lit cubes only in intersection before current step)
                //       (Lit cubes inside of intersection are deactivated in this step and have to be remove from count)
                lit_reactor_cubes_before_current_step
                    - lit_cubes_in_intersection_before_current_step
                    + if *current_procedure_step_value {
                        intersection_cuboid.volume
                    } else {
                        0
                    }
            } else {
                // Cuboid of the current step is outside of the observed range does not influence the result
                lit_reactor_cubes_before_current_step
            }
        }
    }
}

fn main() {
    let procedure = parse_input_data(input_data());

    // Solution for puzzle 1
    let number_of_lit_cubes = lit_cubes_after_procedure_steps(
        &Cuboid::new(-50..=50, -50..=50, -50..=50),
        &procedure.as_slice(),
    );
    println!(
        "Active reactor core cubes after executing initialization procedure: {}",
        number_of_lit_cubes
    );

    // Solution for puzzle 2
    let (min_x, max_x, min_y, max_y, min_z, max_z) = procedure.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(min_x, max_x, min_y, max_y, min_z, max_z), (_, cuboid)| {
            (
                i64::min(*cuboid.x_range.start(), min_x),
                i64::max(*cuboid.x_range.end(), max_x),
                i64::min(*cuboid.y_range.start(), min_y),
                i64::max(*cuboid.y_range.end(), max_y),
                i64::min(*cuboid.z_range.start(), min_z),
                i64::max(*cuboid.z_range.end(), max_z),
            )
        },
    );
    let number_of_lit_cubes = lit_cubes_after_procedure_steps(
        &Cuboid::new(min_x..=max_x, min_y..=max_y, min_z..=max_z),
        &procedure.as_slice(),
    );
    println!(
        "Active reactor core cubes after reboot procedure: {}",
        number_of_lit_cubes
    );
}
