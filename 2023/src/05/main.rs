use std::{fs, collections::HashMap, cmp::{min, max}};

const FILE_PATH : &str = "input.txt";

#[derive(Default, PartialEq, Eq, Hash)]
struct SRange
{
    start: u64,
    end: u64
}

#[derive(Default)]
struct SAlmanac
{
    seed_to_soil: HashMap<SRange, SRange>,
    soil_to_fertilizer: HashMap<SRange, SRange>,
    fertilizer_to_water: HashMap<SRange, SRange>,
    water_to_light: HashMap<SRange, SRange>,
    light_to_temperature: HashMap<SRange, SRange>,
    temperature_to_humidity: HashMap<SRange, SRange>,
    humidity_to_location: HashMap<SRange, SRange>,
}

fn get_from_almanac<'a>(almanac: &'a mut SAlmanac, map_str: &str) -> &'a mut HashMap<SRange, SRange>
{
    return match map_str
    {
        "seed-to-soil" => &mut almanac.seed_to_soil,
        "soil-to-fertilizer" => &mut almanac.soil_to_fertilizer,
        "fertilizer-to-water" => &mut almanac.fertilizer_to_water,
        "water-to-light" => &mut almanac.water_to_light,
        "light-to-temperature" => &mut almanac.light_to_temperature,
        "temperature-to-humidity" => &mut almanac.temperature_to_humidity,
        "humidity-to-location" => &mut almanac.humidity_to_location,
        _ => &mut almanac.seed_to_soil
    };
}

fn parse_almanac(contents: &String) -> SAlmanac
{
    let mut almanac: SAlmanac = SAlmanac { ..Default::default() };
    let mut current_map = &mut almanac.seed_to_soil;

    for line in contents.lines().skip(2).filter(|x| !x.is_empty())
    {
        if line.rfind(":").is_some()
        {
            current_map = get_from_almanac(&mut almanac, line.split(" ").collect::<Vec<&str>>().first().unwrap());
        }
        else
        {
            let data = line.split(" ").filter_map(|x| Some(x.parse::<u64>().unwrap())).collect::<Vec<u64>>();
            let (destination_start, source_start, length) = (data[0], data[1], data[2]);
            current_map.insert(
                SRange
                {
                    start: source_start,
                    end: source_start + length - 1
                },
            SRange
                {
                    start: destination_start,
                    end:destination_start + length - 1
                }
            );
        }
    }

    return almanac;
}

fn is_in_range(value: u64, range: &SRange) -> bool
{
    return value >= range.start && value <= range.end;
}

fn get_destination(value: u64, source: &SRange, destination: &SRange) -> u64
{
    return destination.start + (value - source.start);
}

fn intersects(range: &SRange, seed_range: &SRange) -> bool
{
    return (range.start >= seed_range.start && range.start <= seed_range.end)
        || (range.end >= seed_range.start && range.end <= seed_range.end);
}

fn intersect(range: &SRange, source: &SRange, destination: &SRange) -> (Vec<SRange>, Vec<SRange>)
{
    let mut split_ranges: Vec<SRange> = Vec::new();
    let mut new_ranges: Vec<SRange> = Vec::new();

    if range.start < source.start
    {
        split_ranges.push(
            SRange
            {
                start: range.start,
                end: min(source.start - 1, range.end)
            });
    }

    new_ranges.push(
        SRange
        {
            start: get_destination(max(range.start, source.start), source, destination),
            end: get_destination(min(range.end, source.end), source, destination)
        });

    if range.end > source.end
    {
        split_ranges.push(
            SRange
            {
                start: max(range.start, source.end + 1),
                end: range.end
            });
    }

    return (split_ranges, new_ranges);
}

fn parse_range(ranges: &mut Vec<SRange>, maps: &[&HashMap<SRange, SRange>; 7]) -> u64
{
    for map in maps
    {
        let mut next_ranges: Vec<SRange> = Vec::new();
        
        while !ranges.is_empty()
        {
            let mut intersected = false;
            let range = ranges.pop().unwrap();

            for (source, destination) in map.iter()
            {
                if intersects(&range, source)
                {
                    let (split_ranges, new_ranges) = intersect(&range, source, destination);
                    
                    next_ranges.extend(new_ranges);
                    ranges.extend(split_ranges);
                    
                    intersected = true;
                    break;
                }
            }
            if !intersected
            {
                next_ranges.push(range);
            }
        }

        *ranges = next_ranges;
    }

    return ranges.iter().map(|x| x.start).min().unwrap();
}

fn part1(first_line: &String, almanac: &SAlmanac) -> u64
{
    let seeds: Vec<u64> = first_line.split(": ").collect::<Vec<&str>>().last().unwrap().split(" ").filter_map(|x| Some(x.parse::<u64>().unwrap())).collect();
    let maps = [&almanac.seed_to_soil, &almanac.soil_to_fertilizer, &almanac.fertilizer_to_water, &almanac.water_to_light, &almanac.light_to_temperature, &almanac.temperature_to_humidity, &almanac.humidity_to_location];

    let mut current_search: u64;
    let mut seed_locations: Vec<u64> = Vec::new();

    for seed in seeds
    {
        current_search = seed;
        for map in maps
        {
            for range in map.iter()
            {
                if is_in_range(current_search, range.0)
                {
                    current_search = get_destination(current_search, range.0, range.1);
                    break;
                }
            }
        }

        seed_locations.push(current_search);
    }

    return *seed_locations.iter().min().unwrap();
}

fn part2(first_line: &String, almanac: &SAlmanac) -> u64
{
    let maps: [&HashMap<SRange, SRange>; 7] = [&almanac.seed_to_soil, &almanac.soil_to_fertilizer, &almanac.fertilizer_to_water, &almanac.water_to_light, &almanac.light_to_temperature, &almanac.temperature_to_humidity, &almanac.humidity_to_location];
    let seeds_range: Vec<u64> = first_line.split(": ").collect::<Vec<&str>>().last().unwrap().split(" ").filter_map(|x| Some(x.parse::<u64>().unwrap())).collect();
    
    let mut ranges: Vec<SRange> = Vec::new();
    for seed_range in seeds_range.chunks(2).into_iter()
    {
        let (range_start, range_length) = (seed_range[0], seed_range[1]);
        let range_end = range_start + range_length;
        ranges.push(SRange{start: range_start, end: range_end});
    }

    return parse_range(&mut ranges, &maps);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let first_line = contents.lines().take(1).collect::<String>();
    let almanac = parse_almanac(&contents);

    let part1_result: u64 = part1(&first_line, &almanac);
    let part2_result: u64 = part2(&first_line, &almanac);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}