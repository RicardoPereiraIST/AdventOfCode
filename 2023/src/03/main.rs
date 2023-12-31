use std::{fs, cmp::{max, min}};
use regex::Regex;

const FILE_PATH : &str = "input.txt";

fn get_numbers(re: &Regex, lines: &[&str]) -> Vec<(usize, usize, u64)>
{
    let mut neighbors: Vec<(usize, usize, u64)> = Vec::new();
    for line in lines
    {
        for group in re.captures_iter(line)
        {
            for (_, re_match) in group.iter().filter(|m| !m.unwrap().is_empty()).enumerate()
            {
                let (start, end, value) = (re_match.unwrap().range().start, re_match.unwrap().range().end, re_match.unwrap().as_str());
                neighbors.push((start, end, value.parse::<u64>().unwrap()));
            }
        }
    }
    return neighbors;
}

fn get_slice(lines: &[&str], min_index: usize, max_index: usize) -> Vec<char>
{
    let mut slices: Vec<char> = Vec::new();

    for line in lines
    {
        slices.extend(line.chars().collect::<Vec<char>>()[min_index..max_index].to_vec());
    }

    return slices;
}


fn check_line(current_line: &str, previous_line: &str, next_line: &str, numbers_re: &Regex) -> u64
{
    let mut result = 0;
    let neighbors: Vec<(usize, usize, u64)> = get_numbers(numbers_re, &[current_line]);

    for (start, end, value) in neighbors
    {
        let (min_index, max_index) = (max(min(start.wrapping_sub(1), start), 0), min(end + 1, current_line.len()));
        let slices: Vec<char> = get_slice(&[previous_line, current_line, next_line], min_index, max_index);

        if slices.iter().any(|c| !c.is_numeric() && *c != '.')
        {
            result += value;
        }
    }

    return result;
}

fn get_proximity_numbers(neighbors: &Vec<(usize, usize, u64)>, start: usize) -> Vec<u64>
{
    let mut proximity_numbers: Vec<u64> = Vec::new();
    for neighbor in neighbors
    {
        if (neighbor.0 >= start - 1 && neighbor.0 <= start + 1)
            || (neighbor.1 - 1 >= start - 1 && neighbor.1 - 1 <= start + 1)
        {
            proximity_numbers.push(neighbor.2);
        }
    }
    return proximity_numbers;
}

fn calculate_gears(current_line: &str, previous_line: &str, next_line: &str, numbers_re: &Regex, gears_re: &Regex) -> u64
{
    let mut result = 0;
    let neighbors: Vec<(usize, usize, u64)> = get_numbers(numbers_re, &[previous_line, current_line, next_line]);

    for gear_group in gears_re.captures_iter(current_line)
    {
        for (_, re_match) in gear_group.iter().filter(|m| !m.unwrap().is_empty()).enumerate()
        {
            let start: usize = re_match.unwrap().range().start;
            let proximity_numbers: Vec<u64> = get_proximity_numbers(&neighbors, start);
            
            if proximity_numbers.len() == 2
            {
                result += proximity_numbers[0] * proximity_numbers[1];
            }
        }
    }

    return result;
}

fn part1(lines: &Vec<&str>, fake_line: &str, numbers_re: &Regex) -> u64
{
    let mut result: u64 = 0;

    for (i, line) in lines.iter().enumerate()
    {
        let previous_line = if i == 0 { &fake_line } else { lines.get(i - 1).unwrap_or(&fake_line) };
        let next_line = lines.get(i + 1).unwrap_or(&fake_line);

        result += check_line(line, previous_line, next_line, &numbers_re);
    }

    return result;
}

fn part2(lines: &Vec<&str>, fake_line: &str, numbers_re: &Regex, gears_re: &Regex) -> u64
{
    let mut result: u64 = 0;

    for (i, line) in lines.iter().enumerate()
    {
        let previous_line = lines.get(i.wrapping_sub(1)).unwrap_or(&fake_line);
        let next_line = lines.get(i + 1).unwrap_or(&fake_line);

        result += calculate_gears(line, previous_line, next_line, &numbers_re, &gears_re);
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let fake_line: String = (0..lines[0].len()).map(|_| ".").collect::<String>();
    let fake_line_str: &str = fake_line.as_str();

    let numbers_re = Regex::new(r"[0-9]*").unwrap();
    let gear_re = Regex::new(r"[\*]*").unwrap();

    let part1_result: u64 = part1(&lines, &fake_line_str, &numbers_re);
    let part2_result: u64 = part2(&lines, &fake_line_str, &numbers_re, &gear_re);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}