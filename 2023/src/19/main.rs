// Original code was not flexible enough for part 2.
// Idea from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day19p2.py

use std::{fs, collections::HashMap, cmp::{min, max}};

const FILE_PATH : &str = "input.txt";

type Workflows = HashMap<String, Workflow>;
type Parts = Vec<Part>;
type Fallback = String;
type Range = (u64, u64);
type Ranges = HashMap<char, Range>;

#[derive(Default)]
struct Rule
{
    key: char,
    op: char,
    value: u64,
    target: String
}

struct Workflow
{
    rules: Vec<Rule>,
    fallback: Fallback
}

#[derive(Debug)]
struct Part
{
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

fn convert_to_operation(c: char) -> fn(u64, u64) -> bool
{
    return match c
    {
        '>' => |lhs, rhs| return lhs > rhs,
        '<' => |lhs, rhs| return lhs < rhs,
        _ => |_, _| return false
    }
}

fn convert_to_category(part: &Part, variable: char) -> u64
{
    return match variable
    {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => 0
    }
}

fn parse(contents: &String) -> (Workflows, Parts)
{
    let mut map: Workflows = Workflows::new();
    let mut parts: Parts = Parts::new();

    let lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    let content_split: Vec<_> = lines.split(|line| line.is_empty()).collect();
    let (map_data, parts_data) = (content_split[0], content_split[1]);
    
    for map_node in map_data
    {
        let node_split: Vec<&str> = map_node.split("{").collect();
        let (node_name, node_data) = (node_split[0], node_split[1]);
        let mut rules: Vec<&str> = node_data.split(",").collect();
        let fallback = rules.pop().unwrap().replace("}", "");

        let workflow: Workflow = Workflow
            {
                rules: Vec::new(),
                fallback: fallback.to_string()
            };

        let cur_node = map.entry(node_name.to_string()).or_insert(workflow);

        for rule in rules
        {
            let condition_split: Vec<&str> = rule.split(":").collect();
            let (operation_str, next_node) = (condition_split[0], condition_split[1]);
            let (variable, op, value) = (operation_str.chars().nth(0).unwrap(), operation_str.chars().nth(1).unwrap(), operation_str[2..].parse::<u64>().unwrap());
            
            cur_node.rules.push(Rule
                {
                    key: variable,
                    op: op,
                    value: value,
                    target: next_node.to_string()
                });
        }
    }

    for part in parts_data
    {
        let part_clean = &part[1..part.len() - 1];
        let categories: Vec<&str> = part_clean.split(",").collect();
        let x = categories[0][2..categories[0].len()].parse::<u64>().unwrap();
        let m = categories[1][2..categories[1].len()].parse::<u64>().unwrap();
        let a = categories[2][2..categories[2].len()].parse::<u64>().unwrap();
        let s = categories[3][2..categories[3].len()].parse::<u64>().unwrap();

        parts.push(
            Part
            {
                x: x,
                m: m,
                a: a,
                s: s
            });
    }

    return (map, parts);
}

fn accept(workflows: &Workflows, part: &Part, name: &String) -> bool
{
    if name == "R"
    {
        return false;
    }
    
    if name == "A"
    {
        return true;
    }

    let workflow = &workflows[name];

    for rule in workflow.rules.iter()
    {
        if convert_to_operation(rule.op)(convert_to_category(part, rule.key), rule.value)
        {
            return accept(workflows, part, &rule.target);
        }
    }
    
    return accept(workflows, part, &workflow.fallback);
}

fn subdivide_ranges(op: char, low: u64, high: u64, value: u64) -> (Range, Range)
{
    if op == '<'
    {
        return ((low, min(value - 1, high)),
            (max(value, low), high));
    }
    else
    {
        return ((max(value + 1, low), high),
            (low, min(value, high)));
    }
}

fn setup_range(mut ranges: Ranges, key: char, update_range: &Range) -> Ranges
{
    *ranges.get_mut(&key).unwrap() = *update_range;
    return ranges;
}

fn iterate_rules(workflows: &Workflows, mut ranges: Ranges, workflow: &Workflow, total: &mut u64) -> (Ranges, bool)
{
    for rule in workflow.rules.iter()
    {
        let (low, high) = ranges[&rule.key];
        let (true_range, false_range) = subdivide_ranges(rule.op, low, high, rule.value);

        if true_range.0 <= true_range.1
        {
            let copy_ranges = setup_range(ranges.clone(), rule.key, &true_range);
           *total += count(workflows, copy_ranges, &rule.target);
        }

        if false_range.0 <= false_range.1
        {
            ranges = setup_range(ranges.clone(), rule.key, &false_range);
        }
        else
        {
            return (ranges, false);
        }
    }

    return (ranges, true);
}

fn count(workflows: &Workflows, ranges: Ranges, name: &String) -> u64
{
    if name == "R"
    {
        return 0;
    }

    if name == "A"
    {
        let mut product: u64 = 1;
        for (low, high) in ranges.values()
        {
            product *= high - low + 1;
        }
        return product;
    }
    
    let workflow = &workflows[name];
    let mut total = 0;

    let (ranges, needs_fallback) = iterate_rules(workflows, ranges, &workflow, &mut total);
    
    if needs_fallback
    {
        total += count(workflows, ranges, &workflow.fallback);
    }

    return total;
}

fn part1(workflows: &Workflows, parts: &Parts) -> u64
{
    let mut result = 0;
    let start_node = "in".to_string();

    for part in parts
    {
        if accept(workflows, &part, &start_node)
        {
            result += part.x + part.m + part.a + part.s;
        }
    }

    return result;
}

fn part2(workflows: &Workflows) -> u64
{
    let ranges: Ranges = Ranges::from(
        [
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]);

    return count(&workflows, ranges, &"in".to_string());
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let (workflows, parts) = parse(&contents);

    let part1_result: u64 = part1(&workflows, &parts);
    let part2_result: u64 = part2(&workflows);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}