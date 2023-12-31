// Used LCM idea from https://www.reddit.com/r/adventofcode/comments/18df7px/2023_day_8_solutions/

use std::{fs, collections::HashMap};
use ::num::integer::lcm;

const FILE_PATH : &str = "input.txt";

enum Direction
{
    Left,
    Right
}

impl Direction
{
    pub fn from(c: char) -> Result<Direction, ()> {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

type Pos = String;
struct Choice
{
    left: Pos,
    right: Pos
}

trait Get
{
    fn get(&self, direction: Direction) -> &Pos;
}

impl Get for Choice
{
    fn get(&self, direction: Direction) -> &Pos
    {
        return match direction
        {
            Direction::Left => &self.left,
            Direction::Right => &self.right
        }
    }
}

type Map = HashMap<Pos, Choice>;

struct Path
{
    current_pos: String
}

struct Part1;
struct Part2;

trait IsEnd
{
    fn is_end(path: &Path) -> bool;
}

impl IsEnd for Part1
{
    fn is_end(path: &Path) -> bool
    {
        return path.current_pos == "ZZZ";
    }
}

impl IsEnd for Part2
{
    fn is_end(path: &Path) -> bool
    {
        return path.current_pos.ends_with("Z");
    }
}

fn create_map(contents: &String) -> Map
{
    let mut map : Map = HashMap::new();

    for line in contents.lines().skip(2)
    {
        let position_split: Vec<&str> = line.split(" = ").collect();
        let left_right_split: Vec<&str> = position_split.last().unwrap().split(", ").collect();
        
        let start_position = position_split.first().unwrap();
        let left = left_right_split.first().unwrap().replace("(", "");
        let right = left_right_split.last().unwrap().replace(")", "");
        map.insert(start_position.to_string(), Choice { left: left, right: right });
    }

    return map;
}

fn run<TPart: IsEnd>(path: &mut Path, map: &Map, instructions: &Vec<char>) -> u64
{
    let mut result = 0;
    let mut cur_index = 0;

    loop
    {
        if TPart::is_end(&path)
        {
            break;
        }

        let direction: Direction = Direction::from(instructions[cur_index]).unwrap();
        path.current_pos = map.get(&path.current_pos).unwrap().get(direction).to_string();

        cur_index = (cur_index + 1) % instructions.len();
        result += 1;
    }
    
    return result;
}

fn part1(instructions: &Vec<char>, map: &Map) -> u64
{
    let mut path: Path = Path { current_pos: "AAA".to_string()};
    return run::<Part1>(&mut path, &map, &instructions);
}

fn part2(instructions: &Vec<char>, map: &Map) -> u64
{
    let mut steps: Vec<u64> = Vec::new();
    for pos in map.keys()
    {
        if pos.ends_with('A')
        {
            let mut path: Path = Path { current_pos: pos.clone()};
            steps.push(run::<Part2>(&mut path, &map, &instructions));
        }
    }

    return steps.into_iter().reduce(|lhs, rhs| lcm(lhs, rhs)).unwrap();
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let instructions: Vec<char> = contents.lines().take(1).collect::<String>().chars().collect();
    let map : HashMap<Pos, Choice> = create_map(&contents);

    let part1_result: u64 = part1(&instructions, &map);
    let part2_result: u64 = part2(&instructions, &map);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}