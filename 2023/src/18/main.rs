use std::{fs, ops::{Add, AddAssign, Mul}};

use num::{abs, integer::sqrt, pow};

const FILE_PATH : &str = "input.txt";

type Vertices = Vec<Position>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position
{
    x: i64,
    y: i64
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction
{
    Left,
    Right,
    Down,
    Up,
}

struct Part1;
struct Part2;

trait GetVertex
{
    fn get_vertex(current_vertex: &Position, line_split: Vec<&str>) -> Position;
}

impl GetVertex for Part1
{
    fn get_vertex(current_vertex: &Position, line_split: Vec<&str>) -> Position
    {
        let direction: Direction = Direction::from(line_split[0]).unwrap();
        let shift: i64 = line_split[1].parse::<i64>().unwrap();
        let new_vertex = *current_vertex + (convert_to_coordinates(direction) * shift);

        return new_vertex;
    }
}

impl GetVertex for Part2
{
    fn get_vertex(current_vertex: &Position, line_split: Vec<&str>) -> Position
    {
        let (hex, dir) = (u64::from_str_radix(&line_split[2][2..7], 16).unwrap(), line_split[2].chars().nth(7).unwrap().to_digit(10).unwrap() as u64);
        let direction: Direction = dir.try_into().unwrap();
        return *current_vertex + (convert_to_coordinates(direction) * hex.try_into().unwrap());
    }
}

impl Direction
{
    pub fn from(s: &str) -> Result<Direction, ()> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

impl TryFrom<u64> for Direction
{
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error>
    {
        match v
        {
            0 => Ok(Direction::Right),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Up),
            _ => Err(())
        }
    }
}

impl Add for Position
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        return Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position
{
    fn add_assign(&mut self, other: Self)
    {
        *self = Self
        {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<i64> for Position
{
    type Output = Self;

    fn mul(self, other: i64) -> Self
    {
        return Self
        {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

fn convert_to_coordinates(direction: Direction) -> Position
{
    return match direction
    {
        Direction::Left => Position{x: 0, y: -1},
        Direction::Right => Position{x: 0, y: 1},
        Direction::Up => Position{x: -1, y: 0},
        Direction::Down => Position{x: 1, y: 0}
    }
}

fn get_vertices<TPart : GetVertex>(contents: &String) -> Vertices
{
    let mut vertices: Vertices = Vertices::new();
    vertices.push(Position{x: 0, y: 0});

    for line in contents.lines()
    {
        let line_split: Vec<&str> = line.split(" ").collect();
        vertices.push(TPart::get_vertex(vertices.last().unwrap(), line_split));
    }

    return vertices;
}

fn determinant(pos1: &Position, pos2: &Position) -> i64
{
    return pos1.x * pos2.y - pos1.y * pos2.x;
}

fn calculate_area(positions: &Vertices) -> u64
{
    let mut area = 0;

    for i in 1..positions.len()
    {
        area += determinant(&positions[i-1], &positions[i]);
    }

    return abs(area / 2) as u64;
}

fn distance(pos1: &Position, pos2: &Position) -> u64
{
    return sqrt((pow(pos2.x - pos1.x, 2) + pow(pos2.y - pos1.y, 2)).try_into().unwrap());
}

fn calculate_points_along_vertices(vertices: &Vertices) -> u64
{
    let mut points: u64 = 0;

    for i in 1..vertices.len()
    {
        points += distance(&vertices[i - 1], &vertices[i]);
    }

    return points;
}

fn calculate_pick(area: u64, number_of_vertices: u64) -> u64
{
    return area - (number_of_vertices / 2) + 1;
}

fn calculate_total(vertices: &Vertices) -> u64
{
    let points = calculate_points_along_vertices(&vertices);
    let area = calculate_area(&vertices);
    let pick = calculate_pick(area, points);

    return points + pick;
}

fn part1(contents: &String) -> u64
{
    let vertices: Vertices = get_vertices::<Part1>(&contents);
    return calculate_total(&vertices);
}

fn part2(contents: &String) -> u64
{
    let vertices: Vertices = get_vertices::<Part2>(&contents);
    return calculate_total(&vertices);
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