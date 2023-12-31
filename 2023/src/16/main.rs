use std::{fs, collections::{VecDeque, HashMap}, ops::{AddAssign, Add}, cmp::max};

const FILE_PATH : &str = "input.txt";

type Matrix = Vec<Vec<Obstacle>>;
type Beams = VecDeque<Beam>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction
{
    Left,
    Right,
    Up,
    Down
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Obstacle
{
    RightMirror,
    LeftMirror,
    UpSplitter,
    LeftSplitter,
    None
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

#[derive(Copy, Clone)]
struct Beam
{
    position: Position,
    direction: Direction
}

fn get_next_direction(to_direction: Direction, obstacle: Obstacle) -> Vec<Direction>
{
    return match to_direction
    {
        Direction::Left => match obstacle
            {
                Obstacle::None => [Direction::Left].to_vec(),
                Obstacle::LeftSplitter => [Direction::Left].to_vec(),
                Obstacle::UpSplitter => [Direction::Up, Direction::Down].to_vec(),
                Obstacle::LeftMirror => [Direction::Up].to_vec(),
                Obstacle::RightMirror => [Direction::Down].to_vec(),
            },
        Direction::Right => match obstacle
            {
                Obstacle::None => [Direction::Right].to_vec(),
                Obstacle::LeftSplitter => [Direction::Right].to_vec(),
                Obstacle::UpSplitter => [Direction::Up, Direction::Down].to_vec(),
                Obstacle::LeftMirror => [Direction::Down].to_vec(),
                Obstacle::RightMirror => [Direction::Up].to_vec(),
            },
        Direction::Up => match obstacle
            {
                Obstacle::None => [Direction::Up].to_vec(),
                Obstacle::LeftSplitter => [Direction::Left, Direction::Right].to_vec(),
                Obstacle::UpSplitter => [Direction::Up].to_vec(),
                Obstacle::LeftMirror => [Direction::Left].to_vec(),
                Obstacle::RightMirror => [Direction::Right].to_vec(),
            },
        Direction::Down => match obstacle
            {
                Obstacle::None => [Direction::Down].to_vec(),
                Obstacle::LeftSplitter => [Direction::Left, Direction::Right].to_vec(),
                Obstacle::UpSplitter => [Direction::Down].to_vec(),
                Obstacle::LeftMirror => [Direction::Right].to_vec(),
                Obstacle::RightMirror => [Direction::Left].to_vec(),
            },
    }
}

fn convert(direction: Direction) -> Position
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
        matrix.push(line.chars().map(|c|
            {
                match c
                {
                    '\\' => Obstacle::LeftMirror,
                    '/' => Obstacle::RightMirror,
                    '-' => Obstacle::LeftSplitter,
                    '|' => Obstacle::UpSplitter,
                    _ => Obstacle::None
                }
            }).collect())
    }

    return matrix;
}

fn run(matrix: &Matrix, start_pos: &Position, start_direction: &Direction) -> u64
{
    type Seen = HashMap<Position, Vec<Direction>>;

    let mut beams: Beams = vec![Beam{position: *start_pos, direction: *start_direction}; 1].into();
    let mut seen: Seen = Seen::new();

    while !beams.is_empty()
    {
        let current_beam = beams.pop_front().unwrap();
        let seen_entry = seen.get_mut(&current_beam.position);

        if seen_entry.is_some()
        {
            let seen_direction = seen_entry.unwrap();
            if seen_direction.iter().find(|elem| *elem == &current_beam.direction).is_some()
            {
                continue;
            }
            else
            {
                seen_direction.push(current_beam.direction);
            }
        }
        else
        {
            seen.insert(current_beam.position, [current_beam.direction].to_vec());
        }

        let new_directions: Vec<Direction> = get_next_direction(current_beam.direction, matrix[current_beam.position.x as usize][current_beam.position.y as usize]);
        for new_direction in new_directions
        {
            let position_shift = convert(new_direction);
            let mut new_position = current_beam.position + position_shift;
            new_position.x = num::clamp(new_position.x, 0, matrix.len() as i64 - 1);
            new_position.y = num::clamp(new_position.y, 0, matrix[0].len() as i64 - 1);

            beams.push_back(Beam{position: new_position, direction: new_direction});
        }
    }

    return seen.len() as u64;
}

fn part1(matrix: &Matrix) -> u64
{
    return run(&matrix, &Position{x: 0, y:0}, &Direction::Right);
}

fn part2(matrix: &Matrix) -> u64
{
    let mut result = 0;
    let mut beams: Beams = Beams::new();

    for i in 0..matrix.len()
    {
        beams.push_back(Beam{position: Position{x: i as i64, y: 0 as i64}, direction: Direction::Right });
        beams.push_back(Beam{position: Position{x: i as i64, y: matrix[i].len() as i64 - 1 as i64}, direction: Direction::Left });
    }

    for j in 0..matrix[0].len()
    {
        beams.push_back(Beam{position: Position{x: 0 as i64, y: j as i64}, direction: Direction::Down });
        beams.push_back(Beam{position: Position{x: matrix.len() as i64 - 1, y: j as i64}, direction: Direction::Up });
    }

    for beam in beams
    {
        result = max(result, run(&matrix, &beam.position, &beam.direction));
    }

    return result;
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