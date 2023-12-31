use std::{fs, collections::BTreeMap};

const FILE_PATH : &str = "input.txt";
const STRINGS_TO_SEARCH: [&str; 18] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

trait Search
{
    fn search(line: &str, substring: &str) -> Option<usize>;
}

struct Finder;
struct ReverseFinder;

impl Search for Finder
{
    fn search(line: &str, substring: &str) -> Option<usize>
    {
        return line.find(substring);
    }
}

impl Search for ReverseFinder
{
    fn search(line: &str, substring: &str) -> Option<usize>
    {
        return line.rfind(substring);
    }
}

fn convert(digit: &str) -> u64
{
    return match digit
    {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0
    };
}

fn find<T: Search>(line: &str, substring: &str, index: &mut usize) -> bool
{
    let it = T::search(line, substring);
    if it.is_some()
    {
        *index = it.unwrap();
        return true;
    }

    return false;
}

fn search(line: &str, substring: &str, indices: &mut BTreeMap<usize, u64>)
{
    let mut index: usize = 0;
    if find::<Finder>(line, substring, &mut index)
    {
        indices.insert(index, convert(substring));
    }

    if find::<ReverseFinder>(line, substring, &mut index)
    {
        indices.insert(index, convert(substring));
    }
}

fn find_indices(line: &str) -> (u64, u64)
{
    let mut indices: BTreeMap<usize, u64> = BTreeMap::new();

    for string_to_search in STRINGS_TO_SEARCH
    {
        search(line, string_to_search, &mut indices);
    }

    return (*indices.iter().next().unwrap().1, *indices.iter().next_back().unwrap().1);
}

fn part1(contents: &String) -> u64
{
    let mut result: u64 = 0;

    for line in contents.lines()
    {
        let digits: Vec<u64> = line.chars().filter_map(|c: char| c.to_digit(10).map(u64::from)).collect();
        result += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    return result;
}

fn part2(contents: &String) -> u64
{
    let mut result: u64 = 0;

    for line in contents.lines()
    {
        let (first, last) = find_indices(line);
        result += first * 10 + last;
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let part1_result: u64 = part1(&contents);
    let part2_result: u64 = part2(&contents);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}