use std::{fs, collections::{VecDeque, HashSet}, ops::{Add, AddAssign}};

const FILE_PATH : &str = "input.txt";
const PART1_STEPS: u64 = 64;
const PART2_STEPS: u64 = 26501365;

type Neighbors = Vec<Position>;
type Row = Vec<Obstacle>;
type Matrix = Vec<Row>;

#[derive(PartialEq, Eq)]
enum Obstacle
{
    Garden,
    Rock
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction
{
    Left,
    Right,
    Up,
    Down
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position
{
    x: i64,
    y: i64
}

struct Node
{
    position: Position,
    steps: u64
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

struct Part1;
struct Part2;

trait IsValid
{
    fn is_valid(position: &Position, matrix: &Matrix) -> bool;
}

impl IsValid for Part1
{
    fn is_valid(position: &Position, matrix: &Matrix) -> bool
    {
        let in_bounds: bool = (position.x >= 0 && position.x < matrix.len() as i64)
                            && (position.y >= 0 && position.y < matrix[0].len() as i64);
        
        return in_bounds && matrix[position.x as usize][position.y as usize] != Obstacle::Rock;
    }
}

impl IsValid for Part2
{
    fn is_valid(position: &Position, matrix: &Matrix) -> bool
    {
        let modulo_x = position.x.rem_euclid(matrix.len() as i64);
        let modulo_y = position.y.rem_euclid(matrix[0].len() as i64);
        return matrix[modulo_x as usize][modulo_y as usize] != Obstacle::Rock;
    }
}

fn convert(c: char) -> Obstacle
{
    return match c
    {
        '.' => Obstacle::Garden,
        '#' => Obstacle::Rock,
        _ => Obstacle::Garden
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

fn parse(contents: &String) -> Matrix
{
    let mut matrix: Matrix = Matrix::new();

    for line in contents.lines()
    {
        matrix.push(line.chars().map(|c| convert(c)).collect());
    }

    return matrix;
}

fn find_start(contents: &String) -> Position
{
    for (i, line) in contents.lines().enumerate()
    {
        for (j, column) in line.chars().enumerate()
        {
            if column == 'S'
            {
                return Position{x: i as i64, y: j as i64};
            }
        }
    }

    return Position{x: 0, y: 0};
}

fn get_neighbors<TPart: IsValid>(matrix: &Matrix, position: &Position) -> Neighbors
{
    let directions: Vec<Direction> = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];
    let mut neighbors: Neighbors = Neighbors::new();

    for direction in directions
    {
        let new_position: Position = *position + convert_to_coordinates(direction);        
        if TPart::is_valid(&new_position, &matrix)
        {
            neighbors.push(new_position);
        }
    }

    return neighbors;
}

fn fill<TPart: IsValid>(matrix: &Matrix, start: &Position, steps: u64) -> u64
{
    type Positions = HashSet<Position>;

    let mut end_positions: Positions = Positions::new();
    let mut seen: Positions = Positions::new();
    let mut nodes: VecDeque<Node> = vec![Node{position: *start, steps: steps}].into();

    while !nodes.is_empty()
    {
        let node: Node = nodes.pop_front().unwrap();

        if node.steps % 2 == 0
        {
            end_positions.insert(node.position);
        }

        if node.steps != 0
        {
            let neighbors = get_neighbors::<TPart>(&matrix, &node.position);
            for neighbor in neighbors
            {
                if !seen.contains(&neighbor)
                {
                    seen.insert(neighbor);
                    nodes.push_back(Node{position: neighbor, steps: node.steps - 1});
                }
            }
        }
    }

    return end_positions.len() as u64;
}

fn langrange_polynomial(quadratic_points: &Vec<Position>, steps: u64) -> u64
{
    let mut result = 0;
    let mut products: Vec<i64> = Vec::new();

    for pos_i in quadratic_points
    {
        let mut product: i64 = 1;

        for pos_j in quadratic_points
        {
            if pos_i.x != pos_j.x
            {
                product *= (steps as i64 - pos_j.x) / ((pos_i.x - pos_j.x));
            }
        }

        products.push(product);
    }

    for i in 0..quadratic_points.len()
    {
        result += products[i] * quadratic_points[i].y;
    }

    return result as u64;
}

fn part1(matrix: &Matrix, start: &Position) -> u64
{
    return fill::<Part1>(matrix, start, PART1_STEPS);
}

// Idea gotten from https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/kee6vn6/
fn part2(matrix: &Matrix, start: &Position) -> u64
{
    let size: Position = Position{x: matrix.len() as i64, y: matrix[0].len() as i64};
    let mut quadratic_points: Vec<Position> = Vec::new();

    for i in 0..3
    {
        let x: u64 = ((size.x / 2) + (i * size.x)) as u64;
        let y: u64 = fill::<Part2>(matrix, start, x);
        quadratic_points.push(Position{x: x as i64, y: y as i64});
    }

    return langrange_polynomial(&quadratic_points, PART2_STEPS);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let matrix: Matrix = parse(&contents);
    let start: Position = find_start(&contents);

    let part1_result: u64 = part1(&matrix, &start);
    let part2_result: u64 = part2(&matrix, &start);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}