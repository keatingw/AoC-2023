use std::thread;

#[derive(Debug, Clone)]
pub struct SeedMapping {
    pub dest_start: u64,
    pub src_start: u64,
    pub length: u64,
}

impl SeedMapping {
    pub fn contains_seed(&self, seed_number: &u64) -> bool {
        self.src_start <= *seed_number && *seed_number < self.src_start + self.length
    }

    pub fn map_seed(&self, seed_number: &u64) -> u64 {
        if self.contains_seed(seed_number) {
            return self.dest_start + (seed_number - self.src_start);
        }
        seed_number.clone()
    }
}

type SeedMapVec = Vec<SeedMapping>;

trait SeedMapSequence {
    fn map_sequential(&self, seed_number: &u64) -> u64;
}
impl SeedMapSequence for SeedMapVec {
    fn map_sequential(&self, seed_number: &u64) -> u64 {
        for i in self {
            if i.contains_seed(seed_number) {
                return i.map_seed(seed_number);
            }
        }
        seed_number.clone()
    }
}

pub fn read_day5_input() -> (
    Vec<u64>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
    Vec<SeedMapping>,
) {
    let input_str = include_str!("../examples/day5_input.txt");
    let split_lines: Vec<&str> = input_str.split("\n").filter(|x| x != &"").collect();

    let seed_numbers: Vec<u64> = split_lines[0]
        .split(" ")
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let seed_to_soil_pos = split_lines
        .iter()
        .position(|x| x == &"seed-to-soil map:")
        .unwrap();
    let soil_to_fertilizer_pos = split_lines
        .iter()
        .position(|x| x == &"soil-to-fertilizer map:")
        .unwrap();
    let fertilizer_to_water_pos = split_lines
        .iter()
        .position(|x| x == &"fertilizer-to-water map:")
        .unwrap();
    let water_to_light_pos = split_lines
        .iter()
        .position(|x| x == &"water-to-light map:")
        .unwrap();
    let light_to_temperature_pos = split_lines
        .iter()
        .position(|x| x == &"light-to-temperature map:")
        .unwrap();
    let temperature_to_humidity_pos = split_lines
        .iter()
        .position(|x| x == &"temperature-to-humidity map:")
        .unwrap();
    let humidity_to_location_pos = split_lines
        .iter()
        .position(|x| x == &"humidity-to-location map:")
        .unwrap();

    let seed_soil_maps: SeedMapVec = split_lines[seed_to_soil_pos + 1..soil_to_fertilizer_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let soil_fertilizer_maps: SeedMapVec = split_lines
        [soil_to_fertilizer_pos + 1..fertilizer_to_water_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let fertilizer_water_maps: SeedMapVec = split_lines
        [fertilizer_to_water_pos + 1..water_to_light_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let water_light_maps: SeedMapVec = split_lines
        [water_to_light_pos + 1..light_to_temperature_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let light_temperature_maps: SeedMapVec = split_lines
        [light_to_temperature_pos + 1..temperature_to_humidity_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let temperature_humidity_maps: SeedMapVec = split_lines
        [temperature_to_humidity_pos + 1..humidity_to_location_pos]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    let humidity_location_maps: SeedMapVec = split_lines[humidity_to_location_pos + 1..]
        .iter()
        .map(|x| {
            let elements: Vec<u64> = x.split(" ").map(|i| i.parse::<u64>().unwrap()).collect();
            match elements[..] {
                [dest_start, src_start, length] => SeedMapping {
                    dest_start,
                    src_start,
                    length,
                },
                _ => panic!("Found non-3 length mapping"),
            }
        })
        .collect();
    (
        seed_numbers,
        seed_soil_maps,
        soil_fertilizer_maps,
        fertilizer_water_maps,
        water_light_maps,
        light_temperature_maps,
        temperature_humidity_maps,
        humidity_location_maps,
    )
}

pub fn day5_p1() {
    let (
        seed_numbers,
        seed_soil_maps,
        soil_fertilizer_maps,
        fertilizer_water_maps,
        water_light_maps,
        light_temperature_maps,
        temperature_humidity_maps,
        humidity_location_maps,
    ) = read_day5_input();

    let mut locations: Vec<u64> = vec![];

    // NB this whole solution is immensely slow because we iterate over seeds applying anything
    // suitable to fix them instead of going mapping-back
    for seed in &seed_numbers {
        let o1 = seed_soil_maps.map_sequential(seed);
        let o2 = soil_fertilizer_maps.map_sequential(&o1);
        let o3 = fertilizer_water_maps.map_sequential(&o2);
        let o4 = water_light_maps.map_sequential(&o3);
        let o5 = light_temperature_maps.map_sequential(&o4);
        let o6 = temperature_humidity_maps.map_sequential(&o5);
        let o7 = humidity_location_maps.map_sequential(&o6);
        locations.push(o7);
    }
    println!("Minimum location: {:#?}", locations.iter().min());
}

pub fn day5_p2() {
    let (
        seed_numbers,
        seed_soil_maps,
        soil_fertilizer_maps,
        fertilizer_water_maps,
        water_light_maps,
        light_temperature_maps,
        temperature_humidity_maps,
        humidity_location_maps,
    ) = read_day5_input();

    let full_seed_numbers = seed_numbers
        .chunks_exact(2)
        .flat_map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<u64>>();

    println!("Expanded to {:#?} total seeds", &full_seed_numbers.len());

    let mut locations: Vec<u64> = vec![];
    thread::scope(|s| {
        let mut threads: Vec<thread::ScopedJoinHandle<u64>> = Vec::new();
        for seed in &full_seed_numbers {
            // NB this whole solution is immensely slow because we iterate over seeds applying anything
            // suitable to fix them instead of going mapping-back
            let t = s.spawn(|| {
                let o1 = seed_soil_maps.map_sequential(seed);
                let o2 = soil_fertilizer_maps.map_sequential(&o1);
                let o3 = fertilizer_water_maps.map_sequential(&o2);
                let o4 = water_light_maps.map_sequential(&o3);
                let o5 = light_temperature_maps.map_sequential(&o4);
                let o6 = temperature_humidity_maps.map_sequential(&o5);
                humidity_location_maps.map_sequential(&o6)
            });
            threads.push(t);
        }
        locations = threads.into_iter().map(|x| x.join().unwrap()).collect();
        println!("Minimum location: {:#?}", locations.iter().min());
    })
}
