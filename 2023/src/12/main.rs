// Used idea from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day12p1.py

use std::{fs, cmp::min, collections::HashMap};

const FILE_PATH : &str = "input.txt";

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Spring
{
    Operational,
    Damaged,
    Unknown
}

impl Spring
{
    pub fn from(c: char) -> Result<Spring, ()> {
        match c {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(()),
        }
    }
}

type SpringConfiguration = Vec<Spring>;
type Grouping = Vec<u64>;
type Cache = HashMap<Configuration, u64>;

#[derive(PartialEq, Eq, Clone, Hash)]
struct Configuration
{
    configuration: SpringConfiguration,
    grouping: Grouping
}

type Configurations = Vec<Configuration>;

fn parse_configuration(configuration_str: &str) -> SpringConfiguration
{
    return configuration_str.chars().map(|c| Spring::from(c).unwrap()).collect();
}

fn parse(contents: &String) -> Configurations
{
    let mut configurations: Configurations = Configurations::new();

    for line in contents.lines()
    {
        let line_split: Vec<&str> = line.split(" ").collect();
        let configuration: SpringConfiguration = parse_configuration(line_split[0]);
        let grouping: Grouping = line_split[1].split(",").map(|c| c.parse::<u64>().unwrap()).collect();

        configurations.push(Configuration{ configuration: configuration, grouping: grouping });
    }

    return configurations;
}

fn count(configuration: Configuration, cache: &mut Cache) -> u64
{
    if configuration.configuration.is_empty()
    {
        return configuration.grouping.is_empty() as u64;
    }

    if configuration.grouping.is_empty()
    {
        return configuration.configuration.iter().all(|elem| *elem != Spring::Damaged) as u64;
    }

    if cache.contains_key(&configuration)
    {
        return cache[&configuration];
    }

    let mut result: u64 = 0;

    if configuration.configuration[0] == Spring::Operational || configuration.configuration[0] == Spring::Unknown
    {
        result += count(
            Configuration
            {
                configuration: configuration.configuration[1..].to_vec(),
                grouping: configuration.grouping.clone()
            }, cache);
    }

    if configuration.configuration[0] == Spring::Damaged || configuration.configuration[0] == Spring::Unknown
    {
        if (configuration.grouping[0] <= configuration.configuration.len() as u64)
            && (configuration.configuration[0..configuration.grouping[0] as usize].iter().all(|elem| *elem != Spring::Operational))
            && ((configuration.grouping[0] == configuration.configuration.len() as u64) || (configuration.configuration[configuration.grouping[0] as usize] != Spring::Damaged))
        {
            result += count(
                Configuration
                {
                    configuration: configuration.configuration[(min(configuration.grouping[0] as usize + 1, configuration.configuration.len()))..].to_vec(),
                    grouping: configuration.grouping[1..].to_vec()
                }, cache);
        }
    }

    cache.insert(configuration, result);
    return result;
}

fn fix_input(configuration: &mut SpringConfiguration) -> SpringConfiguration
{
    configuration.push(Spring::Unknown);

    let mut repeated_config = configuration.repeat(5);
    repeated_config.pop();

    return repeated_config;
}

fn part1(configurations: &Configurations) -> u64
{
    let mut cache: Cache = Cache::new();
    let mut result = 0;

    for configuration in configurations
    {
        result += count(configuration.clone(), &mut cache);
    }

    return result;
}

fn part2(configurations: &Configurations) -> u64
{
    let mut cache: Cache = Cache::new();
    let mut result = 0;

    for configuration in configurations
    {
        let new_configuration =
            Configuration
            {
                configuration: fix_input(&mut configuration.configuration.clone()),
                grouping: configuration.grouping.repeat(5)
            };

        result += count(new_configuration, &mut cache);
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let configurations: Configurations = parse(&contents);

    let part1_result: u64 = part1(&configurations);
    let part2_result: u64 = part2(&configurations);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}