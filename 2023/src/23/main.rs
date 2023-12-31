// Idea for part 1 from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day23p1.py

use std::{fs, ops::{AddAssign, Add}, collections::{HashMap, VecDeque, HashSet}, vec, cmp::max};

const FILE_PATH : &str = "input.txt";

type Row = Vec<Obstacle>;
type Matrix = Vec<Row>;
type Edges = Vec<Position>;
type Directions = Vec<Direction>;
type Graph = HashMap<Position, HashMap<Position, u64>>;

#[derive(PartialEq, Eq)]
enum Obstacle
{
    Path,
    Forest,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction
{
    Left,
    Right,
    Up,
    Down
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
            Direction::Down => Position{x: 1, y: 0}
        }
    }
}

struct Part1;
struct Part2;

trait ObstacleToPosition
{
    fn convert(obstacle: &Obstacle) -> Directions;
}

impl ObstacleToPosition for Part1
{
    fn convert(obstacle: &Obstacle) -> Directions
    {
        return match obstacle
        {
            Obstacle::SlopeLeft => vec![Direction::Left],
            Obstacle::SlopeRight => vec![Direction::Right],
            Obstacle::SlopeUp => vec![Direction::Up],
            Obstacle::SlopeDown => vec![Direction::Down],
            Obstacle::Path => vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down],
            Obstacle::Forest => vec![],
        }
    }
}

impl ObstacleToPosition for Part2
{
    fn convert(obstacle: &Obstacle) -> Directions
    {
        return match obstacle
        {
            Obstacle::Forest => vec![],
            _ => vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down],
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

impl Position
{
    fn is_valid(&self, matrix: &Matrix) -> bool
    {
        let in_bounds: bool = (self.x >= 0 && self.x < matrix.len() as i64)
                            && (self.y >= 0 && self.y < matrix[0].len() as i64);
        
        return in_bounds && matrix[self.x as usize][self.y as usize] != Obstacle::Forest;
    }
}

fn convert(c: char) -> Obstacle
{
    return match c
    {
        '.' => Obstacle::Path,
        '#' => Obstacle::Forest,
        '<' => Obstacle::SlopeLeft,
        '>' => Obstacle::SlopeRight,
        '^' => Obstacle::SlopeUp,
        'v' => Obstacle::SlopeDown,
        _ => Obstacle::Forest
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

fn find_start(matrix: &Matrix) -> Position
{
    for (i, obstacle) in matrix[0].iter().enumerate()
    {
        if *obstacle == Obstacle::Path
        {
            return Position{x: 0, y: i as i64};
        }
    }

    return Position{x: 0, y: 0};
}

fn find_end(matrix: &Matrix) -> Position
{
    for (i, obstacle) in matrix[matrix.len() - 1].iter().enumerate()
    {
        if *obstacle == Obstacle::Path
        {
            return Position{x: (matrix.len() - 1) as i64, y: i as i64};
        }
    }

    return Position{x: 0, y: 0};
}

fn calculate_edge_contraction(matrix: &Matrix, start: &Position, end: &Position) -> Edges
{
    let mut edges: Edges = vec![*start, *end];
    let directions: Vec<Direction> = vec![Direction::Up, Direction::Left, Direction::Right, Direction::Down];

    for (i, row) in matrix.iter().enumerate()
    {
        for (j, col) in row.iter().enumerate()
        {
            if *col == Obstacle::Forest
            {
                continue;
            }

            let mut neighbor_count: u64 = 0;
            for direction in directions.iter()
            {
                let position = Position{x: i as i64, y: j as i64} + direction.convert();
                if position.is_valid(&matrix)
                {
                    neighbor_count += 1;
                }
            }

            if neighbor_count >= 3
            {
                edges.push(Position{x: i as i64, y: j as i64});
            }
        }
    }

    return edges;
}

fn initialize_graph(edge_contractions: &Edges) -> Graph
{
    let mut graph: Graph = Graph::new();
    for edge in edge_contractions
    {
        graph.insert(*edge, HashMap::new());
    }

    return graph;
}

fn create_graph<TPart: ObstacleToPosition>(matrix: &Matrix, edge_contractions: &Edges) -> Graph
{
    type Node = (Position, u64);
    let mut graph: Graph = initialize_graph(edge_contractions);

    for edge in edge_contractions
    {
        let mut stack: VecDeque<Node> = vec![(*edge, 0)].into();
        let mut seen: HashSet<Position> = HashSet::new();
        seen.insert(*edge);

        while !stack.is_empty()
        {
            let node: Node = stack.pop_front().unwrap();

            if node.1 != 0 && edge_contractions.contains(&node.0)
            {
                let key = graph.entry(*edge).or_insert(HashMap::new());
                *key.entry(node.0).or_insert(0) = node.1;
                continue;
            }

            let directions = TPart::convert(&matrix[node.0.x as usize][node.0.y as usize]);
            for direction in directions
            {
                let position = node.0 + direction.convert();
                if position.is_valid(&matrix) && !seen.contains(&position)
                {
                    stack.push_back((position, node.1 + 1));
                    seen.insert(position);
                }
            }
        }
    }

    return graph;
}

fn dfs(graph: &Graph, seen: &mut HashSet<Position>, position: &Position, end: &Position) -> u64
{
    if position == end
    {
        return 0;
    }

    let mut max_path = std::i64::MIN;
    seen.insert(*position);

    for next_position in graph.get(position).unwrap().keys()
    {
        if !seen.contains(next_position)
        {
            max_path = max(max_path, (dfs(&graph, seen, next_position, &end) + graph.get(position).unwrap().get(next_position).unwrap()) as i64);
        }
    }

    seen.remove(position);

    return max_path as u64;
}

fn part1(matrix: &Matrix, edge_contractions: &Edges, start: &Position, end: &Position) -> u64
{
    let graph: Graph = create_graph::<Part1>(&matrix, &edge_contractions);
    let mut seen: HashSet<Position> = HashSet::new();
    return dfs(&graph, &mut seen, &start, &end);
}

fn part2(matrix: &Matrix, edge_contractions: &Edges, start: &Position, end: &Position) -> u64
{
    let graph: Graph = create_graph::<Part2>(&matrix, &edge_contractions);
    let mut seen: HashSet<Position> = HashSet::new();
    return dfs(&graph, &mut seen, &start, &end);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
                .expect("Should have been able to read the file");

    let matrix: Matrix = parse(&contents);
    let (start, end) = (find_start(&matrix), find_end(&matrix));
    let edge_contractions: Edges = calculate_edge_contraction(&matrix, &start, &end);

    let part1_result: u64 = part1(&matrix, &edge_contractions, &start, &end);
    let part2_result: u64 = part2(&matrix, &edge_contractions, &start, &end);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}