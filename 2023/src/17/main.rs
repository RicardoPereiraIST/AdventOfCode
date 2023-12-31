use std::{fs, ops::{AddAssign, Add}};

use crate::astar::astar;
use pathfinding::directed::astar;

const FILE_PATH : &str = "input.txt";

type Row = Vec<Node>;
type Matrix = Vec<Row>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Direction
{
    Left,
    Right,
    Up,
    Down,
    None
}

impl Direction
{
    fn convert(&self) -> Position
    {
        return match &self
        {
            Direction::Left => Position{x: 0, y: -1},
            Direction::Right => Position{x: 0, y: 1},
            Direction::Up => Position{x: -1, y: 0},
            Direction::Down => Position{x: 1, y: 0},
            Direction::None => Position{x: 0, y: 0}
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position
{
    x: i64,
    y: i64
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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Node
{
    position: Position,
    weight: u64,
    direction: Direction,
    steps: u64
}

struct Part1;
struct Part2;

trait CanMove
{
    fn can_move_current_direction(node: &Node) -> bool;
    fn can_move_other_direction(node: &Node) -> bool;
    fn is_goal(node: &Node, goal_node: &Node) -> bool;
}

impl CanMove for Part1
{
    fn can_move_current_direction(node: &Node) -> bool
    {
        return node.steps < 3;
    }

    fn can_move_other_direction(_: &Node) -> bool
    {
        return true;
    }

    fn is_goal(node: &Node, goal_node: &Node) -> bool
    {
        return node.position == goal_node.position;
    }
}

impl CanMove for Part2
{
    fn can_move_current_direction(node: &Node) -> bool
    {
        return node.steps < 10;
    }

    fn can_move_other_direction(node: &Node) -> bool
    {
        // first node can move in all directions
        return node.steps == 0 || node.steps >= 4;
    }

    fn is_goal(node: &Node, goal_node: &Node) -> bool
    {
        return node.position == goal_node.position && Self::can_move_other_direction(node);
    }
}

fn get_reverse_directions(direction: &Direction) -> Vec<Direction>
{
    return match direction
    {
        Direction::Left => [Direction::Up, Direction::Down].to_vec(),
        Direction::Right => [Direction::Up, Direction::Down].to_vec(),
        Direction::Up => [Direction::Left, Direction::Right].to_vec(),
        Direction::Down => [Direction::Left, Direction::Right].to_vec(),
        Direction::None => [Direction::Left, Direction::Right, Direction::Up, Direction::Down].to_vec()
    }
}

impl Node
{
    fn get_neighbor(node: &Node, direction: &Direction, matrix: &Matrix) -> Option<(Node, u64)>
    {
        let position_shift: Position = direction.convert();
        let position = node.position + position_shift;

        if (position.x >= 0 && position.x < matrix.len() as i64)
            && (position.y >= 0 && position.y < matrix[0].len() as i64)
        {
            let weight = matrix[position.x as usize][position.y as usize].weight;
            let steps = if node.direction == *direction { node.steps + 1 } else { 1 };
            return Some((Node
                {
                    position: position,
                    weight: weight,
                    direction: *direction,
                    steps: steps
                }, weight));
        }

        return None;
    }

    fn distance(&self, other: &Node) -> u64
    {
        return self.position.x.abs_diff(other.position.x) + self.position.y.abs_diff(other.position.y);
    }
    
    fn successors<TPart: CanMove>(&self, matrix: &Matrix) -> Vec<(Node, u64)>
    {
        let mut neighbors: Vec<(Node, u64)> = Vec::new();

        if TPart::can_move_current_direction(self)
        {
            let neighbor = Self::get_neighbor(self, &self.direction, &matrix);
            if neighbor.is_some()
            {
                neighbors.push(neighbor.unwrap());
            }
        }

        if TPart::can_move_other_direction(self)
        {
            let directions: Vec<Direction> = get_reverse_directions(&self.direction);
            for direction in directions
            {
                let neighbor = Self::get_neighbor(self, &direction, &matrix);
                if neighbor.is_some()
                {
                    neighbors.push(neighbor.unwrap());
                }
            }
        }
        
        return neighbors;
    }

    fn is_goal<TPart: CanMove>(&self, goal_node: &Node) -> bool
    {
        return TPart::is_goal(self, goal_node);
    }
}

fn parse(contents: &String) -> Matrix
{
    let mut matrix: Matrix = Matrix::new();

    for (i, line) in contents.lines().enumerate()
    {
        matrix.push(Row::new());

        for (j, c) in line.chars().enumerate()
        {
            matrix[i].push(Node
                {
                    position: Position
                    {
                        x: i as i64,
                        y: j as i64
                    },
                    weight: c.to_digit(10).unwrap() as u64,
                    direction: Direction::None,
                    steps: 0
                });
        }
    }

    return matrix;
}

fn part1(matrix: &Matrix) -> u64
{
    let goal: Node = matrix[matrix.len() - 1][matrix[0].len() - 1];
    let result = astar(&matrix[0][0], |node| node.successors::<Part1>(&matrix), |node| node.distance(&goal), |node| node.is_goal::<Part1>(&goal));

    return result.unwrap().1;
}

fn part2(matrix: &Matrix) -> u64
{
    let goal: Node = matrix[matrix.len() - 1][matrix[0].len() - 1];
    let result = astar(&matrix[0][0], |node| node.successors::<Part2>(&matrix), |node| node.distance(&goal), |node| node.is_goal::<Part2>(&goal));

    return result.unwrap().1;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
                .expect("Should have been able to read the file");

    let matrix: Matrix = parse(&contents);

    let part1_result: u64 = part1(&matrix);
    let part2_result: u64 = part2(&matrix);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}