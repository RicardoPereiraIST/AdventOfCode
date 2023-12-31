// Used idea from https://www.reddit.com/r/adventofcode/comments/18f1sgh/comment/kcr8tyf/ for part 2

use std::{fs, ops::{Add, AddAssign}};

use num::abs;

const FILE_PATH : &str = "input.txt";

type Vertices = Vec<Position>;
type World = Vec<Vec<char>>;

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
    Up,
    Down,
    None
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

fn convert_to_coordinates(direction: Direction) -> Position
{
    return match direction
    {
        Direction::Left => Position{x: -1, y: 0},
        Direction::Right => Position{x: 1, y: 0},
        Direction::Up => Position{x: 0, y: -1},
        Direction::Down => Position{x: 0, y: 1},
        Direction::None => Position{x: 0, y: 0},
    }
}

fn get_next_direction(previous_direction: Direction, pipe: char) -> Direction
{
    return match previous_direction
    {
        Direction::Up => match pipe
        {
            '|' => Direction::Up,
            '7' => Direction::Left,
            'F' => Direction::Right,
            _ => Direction::None
        },
        Direction::Down => match pipe
        {
            '|' => Direction::Down,
            'J' => Direction::Left,
            'L' => Direction::Right,
            _ => Direction::None
        },
        Direction::Left => match pipe
        {
            '-' => Direction::Left,
            'L' => Direction::Up,
            'F' => Direction::Down,
            _ => Direction::None
        },
        Direction::Right => match pipe
        {
            '-' => Direction::Right,
            'J' => Direction::Up,
            '7' => Direction::Down,
            _ => Direction::None
        },
        Direction::None => Direction::None
    }
}

fn parse_world(contents: &String) -> World
{
    let mut world: World = World::new();

    for line in contents.lines()
    {
        world.push(line.chars().collect());
    }

    return world;
}

fn find_start(world: &World) -> Position
{
    for (i, line) in world.iter().enumerate()
    {
        for (j, character) in line.iter().enumerate()
        {
            if *character == 'S'
            {
                return Position{x: j as i64, y: i as i64};
            }
        }
    }

    return Position { x: 0, y: 0 }
}

fn is_position_valid(position: &Position, max_position: &Position) -> bool
{
    return (position.x >= 0 && position.x <= max_position.x)
        && (position.y >= 0 && position.y <= max_position.y);
}

fn get_possible_start_adjacent(world: &World, start: &Position, max_position: &Position) -> Direction
{
    let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    for direction in directions
    {
        let new_position = *start + convert_to_coordinates(direction);

        if is_position_valid(&new_position, max_position)
            && get_next_direction(direction, world[new_position.y as usize][new_position.x as usize]) != Direction::None
        {
            return direction;
        }
    }

    return Direction::None;
}

fn calculate_vertices(world: &World, start: &Position, max_position: &Position) -> Vertices
{
    let mut vertices: Vertices = vec![start.clone()];

    let mut current_direction = get_possible_start_adjacent(&world, start, max_position);
    let mut current_position = *start + convert_to_coordinates(current_direction);

    while current_position != *start
    {
        vertices.push(current_position);

        current_direction = get_next_direction(current_direction, world[current_position.y as usize][current_position.x as usize]);
        current_position += convert_to_coordinates(current_direction);
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

    area += determinant(&positions[positions.len()-1], &positions[0]);

    return abs(area / 2) as u64;
}

fn calculate_pick(area: u64, number_of_vertices: u64) -> u64
{
    return area - (number_of_vertices / 2) + 1;
}

fn part1(vertices: &Vertices) -> u64
{
    let number_of_vertices = vertices.len() as u64;
    return number_of_vertices / 2;
}

fn part2(vertices: &Vertices) -> u64
{
    let area = calculate_area(&vertices);
    let number_of_vertices = vertices.len() as u64;

    return calculate_pick(area, number_of_vertices)
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let world: World = parse_world(&contents);
    let start = find_start(&world);
    let max_position =
        Position
        {
            x: contents.lines().last().unwrap().len() as i64 - 1,
            y: contents.lines().count() as i64 - 1
        };

    let vertices = calculate_vertices(&world, &start, &max_position);

    let part1_result: u64 = part1(&vertices);
    let part2_result: u64 = part2(&vertices);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}