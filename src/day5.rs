use std::{sync::Arc, thread};

#[derive(Debug, Clone)]
pub struct SeedMapping {
    pub dest_start: u64,
    pub src_start: u64,
    pub length: u64,
}

impl SeedMapping {
    pub fn contains_seed(&self, seed_number: &u64) -> bool {
        (self.src_start..self.src_start + self.length).contains(seed_number)
    }

    pub fn map_seed(&self, seed_number: &u64) -> u64 {
        if self.contains_seed(seed_number) {
            return self.dest_start + (seed_number - self.src_start);
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

    let seed_soil_maps: Vec<SeedMapping> = split_lines
        [seed_to_soil_pos + 1..soil_to_fertilizer_pos]
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
    let soil_fertilizer_maps: Vec<SeedMapping> = split_lines
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
    let fertilizer_water_maps: Vec<SeedMapping> = split_lines
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
    let water_light_maps: Vec<SeedMapping> = split_lines
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
    let light_temperature_maps: Vec<SeedMapping> = split_lines
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
    let temperature_humidity_maps: Vec<SeedMapping> = split_lines
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
    let humidity_location_maps: Vec<SeedMapping> = split_lines[humidity_to_location_pos + 1..]
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

    let mut locations: Vec<u64> = Vec::new();

    // NB this whole solution is immensely slow because we iterate over seeds applying anything
    // suitable to fix them instead of going mapping-back
    for seed in &seed_numbers {
        let m1: Vec<&SeedMapping> = seed_soil_maps
            .iter()
            .filter(|x| x.contains_seed(seed))
            .collect();
        let o1: u64 = match m1.len() {
            0 => seed.clone(),
            1 => m1[0].map_seed(seed),
            _ => panic!("Had map count applicable >1"),
        };

        let m2: Vec<&SeedMapping> = soil_fertilizer_maps
            .iter()
            .filter(|x| x.contains_seed(&o1))
            .collect();
        let o2: u64 = match m2.len() {
            0 => o1.clone(),
            1 => m2[0].map_seed(&o1),
            _ => panic!("Had map count applicable >1"),
        };

        let m3: Vec<&SeedMapping> = fertilizer_water_maps
            .iter()
            .filter(|x| x.contains_seed(&o2))
            .collect();
        let o3: u64 = match m3.len() {
            0 => o2.clone(),
            1 => m3[0].map_seed(&o2),
            _ => panic!("Had map count applicable >1"),
        };

        let m4: Vec<&SeedMapping> = water_light_maps
            .iter()
            .filter(|x| x.contains_seed(&o3))
            .collect();
        let o4: u64 = match m4.len() {
            0 => o3.clone(),
            1 => m4[0].map_seed(&o3),
            _ => panic!("Had map count applicable >1"),
        };

        let m5: Vec<&SeedMapping> = light_temperature_maps
            .iter()
            .filter(|x| x.contains_seed(&o4))
            .collect();
        let o5: u64 = match m5.len() {
            0 => o4.clone(),
            1 => m5[0].map_seed(&o4),
            _ => panic!("Had map count applicable >1"),
        };

        let m6: Vec<&SeedMapping> = temperature_humidity_maps
            .iter()
            .filter(|x| x.contains_seed(&o5))
            .collect();
        let o6: u64 = match m6.len() {
            0 => o5.clone(),
            1 => m6[0].map_seed(&o5),
            _ => panic!("Had map count applicable >1"),
        };

        let m7: Vec<&SeedMapping> = humidity_location_maps
            .iter()
            .filter(|x| x.contains_seed(&o6))
            .collect();
        let o7: u64 = match m7.len() {
            0 => o6.clone(),
            1 => m7[0].map_seed(&o6),
            _ => panic!("Had map count applicable >1"),
        };

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
    let seed_soil_maps_arc = Arc::new(seed_soil_maps);
    let soil_fertilizer_maps_arc = Arc::new(soil_fertilizer_maps);
    let fertilizer_water_maps_arc = Arc::new(fertilizer_water_maps);
    let water_light_maps_arc = Arc::new(water_light_maps);
    let light_temperature_maps_arc = Arc::new(light_temperature_maps);
    let temperature_humidity_maps_arc = Arc::new(temperature_humidity_maps);
    let humidity_location_maps_arc = Arc::new(humidity_location_maps);

    let full_seed_numbers = seed_numbers
        .chunks_exact(2)
        .flat_map(|x| x[0]..x[0] + x[1])
        .collect::<Vec<u64>>();

    println!("Expanded to {:#?} total seeds", &full_seed_numbers.len());

    let threads: Vec<thread::JoinHandle<u64>> = full_seed_numbers
        .into_iter()
        .map(|seed| {
            let seed_soil_maps_arc = seed_soil_maps_arc.clone();
            let soil_fertilizer_maps_arc = soil_fertilizer_maps_arc.clone();
            let fertilizer_water_maps_arc = fertilizer_water_maps_arc.clone();
            let water_light_maps_arc = water_light_maps_arc.clone();
            let light_temperature_maps_arc = light_temperature_maps_arc.clone();
            let temperature_humidity_maps_arc = temperature_humidity_maps_arc.clone();
            let humidity_location_maps_arc = humidity_location_maps_arc.clone();
            thread::spawn(move ||
    // NB this whole solution is immensely slow because we iterate over seeds applying anything
    // suitable to fix them instead of going mapping-back
    {
        let m1: Vec<&SeedMapping> = seed_soil_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&seed))
            .collect();
        let o1: u64 = match m1.len() {
            0 => seed.clone(),
            1 => m1[0].map_seed(&seed),
            _ => panic!("Had map count applicable >1"),
        };

        let m2: Vec<&SeedMapping> = soil_fertilizer_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o1))
            .collect();
        let o2: u64 = match m2.len() {
            0 => o1.clone(),
            1 => m2[0].map_seed(&o1),
            _ => panic!("Had map count applicable >1"),
        };

        let m3: Vec<&SeedMapping> = fertilizer_water_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o2))
            .collect();
        let o3: u64 = match m3.len() {
            0 => o2.clone(),
            1 => m3[0].map_seed(&o2),
            _ => panic!("Had map count applicable >1"),
        };

        let m4: Vec<&SeedMapping> = water_light_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o3))
            .collect();
        let o4: u64 = match m4.len() {
            0 => o3.clone(),
            1 => m4[0].map_seed(&o3),
            _ => panic!("Had map count applicable >1"),
        };

        let m5: Vec<&SeedMapping> = light_temperature_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o4))
            .collect();
        let o5: u64 = match m5.len() {
            0 => o4.clone(),
            1 => m5[0].map_seed(&o4),
            _ => panic!("Had map count applicable >1"),
        };

        let m6: Vec<&SeedMapping> = temperature_humidity_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o5))
            .collect();
        let o6: u64 = match m6.len() {
            0 => o5.clone(),
            1 => m6[0].map_seed(&o5),
            _ => panic!("Had map count applicable >1"),
        };

        let m7: Vec<&SeedMapping> = humidity_location_maps_arc
            .iter()
            .filter(|x| x.contains_seed(&o6))
            .collect();
        let o7: u64 = match m7.len() {
            0 => o6.clone(),
            1 => m7[0].map_seed(&o6),
            _ => panic!("Had map count applicable >1"),
        };
        o7
    })
        })
        .collect();
    let locations: Vec<u64> = threads.into_iter().map(|x| x.join().unwrap()).collect();
    println!("Minimum location: {:#?}", locations.iter().min());
}
