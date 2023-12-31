use std::{fs, collections::HashMap};

const FILE_PATH : &str = "input.txt";
const NUMBER_CYCLES: u64 = 1000000000;

type Row = Vec<Obstacle>;
type Matrix = Vec<Row>;
type Configuration = Vec<Position>;

#[derive(PartialEq, Eq, Debug)]
enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Obstacle
{
    Round,
    Cube,
    None
}

#[derive(PartialEq, Eq, Hash)]
struct Position
{
    x: usize,
    y: usize
}

impl Obstacle
{
    pub fn from(c: char) -> Result<Obstacle, ()> {
        return match c {
            'O' => Ok(Obstacle::Round),
            '#' => Ok(Obstacle::Cube),
            '.' => Ok(Obstacle::None),
            _ => Err(()),
        }
    }
}

fn convert_to_shift(direction: Direction) -> i64
{
    return match direction
    {
        Direction::Left => 1,
        Direction::Right => -1,
        Direction::Up => 1,
        Direction::Down => -1
    }
}

fn parse(contents: &String) -> Matrix
{
    let mut matrix: Matrix = Matrix::new();

    for line in contents.lines()
    {
        matrix.push(line.chars().map(|c| Obstacle::from(c).unwrap()).collect());
    }

    return matrix;
}

fn calculate_vertical_change(matrix: &mut Matrix, i: usize, j: usize, last_obstacles: &mut Vec<i64>, direction: Direction)
{
    assert_ne!(direction, Direction::Left);
    assert_ne!(direction, Direction::Right);

    let shift = convert_to_shift(direction);

    if matrix[i][j] == Obstacle::Cube
    {
        last_obstacles[j] = i as i64;
    }

    if matrix[i][j] == Obstacle::Round
    {
        if i != (last_obstacles[j] + shift) as usize
        {
            matrix[(last_obstacles[j] + shift) as usize][j] = Obstacle::Round;
            matrix[i][j] = Obstacle::None;
        }

        last_obstacles[j] += shift as i64;
    }
}

fn calculate_horizontal_change(matrix: &mut Matrix, i: usize, j: usize, last_obstacles: &mut Vec<i64>, direction: Direction)
{
    assert_ne!(direction, Direction::Up);
    assert_ne!(direction, Direction::Down);

    let shift = convert_to_shift(direction);

    if matrix[i][j] == Obstacle::Cube
    {
        last_obstacles[i] = j as i64;
    }

    if matrix[i][j] == Obstacle::Round
    {
        if j != (last_obstacles[i] + shift) as usize
        {
            matrix[i][(last_obstacles[i] + shift) as usize] = Obstacle::Round;
            matrix[i][j] = Obstacle::None;
        }

        last_obstacles[i] += shift as i64;
    }
}

fn move_up(matrix: &mut Matrix)
{
    let mut last_obstacles: Vec<i64> = vec![-1; matrix[0].len()];

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            calculate_vertical_change(matrix, i, j, &mut last_obstacles, Direction::Up);
        }
    }
}

fn move_down(matrix: &mut Matrix)
{
    let mut last_obstacles: Vec<i64> = vec![matrix.len() as i64; matrix[0].len()];

    for i in (0..matrix.len()).rev()
    {
        for j in 0..matrix[i].len()
        {
            calculate_vertical_change(matrix, i, j, &mut last_obstacles, Direction::Down);
        }
    }
}

fn move_left(matrix: &mut Matrix)
{
    let mut last_obstacles: Vec<i64> = vec![-1 as i64; matrix.len()];

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            calculate_horizontal_change(matrix, i, j, &mut last_obstacles, Direction::Left);
        }
    }
}

fn move_right(matrix: &mut Matrix)
{
    let mut last_obstacles: Vec<i64> = vec![matrix.len() as i64; matrix.len()];

    for i in 0..matrix.len()
    {
        for j in (0..matrix[i].len()).rev()
        {
            calculate_horizontal_change(matrix, i, j, &mut last_obstacles, Direction::Right);
        }
    }
}

fn cycle(matrix: &mut Matrix)
{
    move_up(matrix);
    move_left(matrix);
    move_down(matrix);
    move_right(matrix);
}

fn count_load(matrix: &Matrix) -> u64
{
    let mut load: u64 = 0;

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == Obstacle::Round
            {
                load += (matrix.len() - i) as u64;
            }
        }
    }

    return load;
}

fn get_configuration(matrix: &Matrix) -> Configuration
{
    let mut configuration: Configuration = Configuration::new();

    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == Obstacle::Round
            {
                configuration.push(Position{x: i, y: j});
            }
        }
    }

    return configuration;
}

fn clear_moveable_obstacles(matrix: &mut Matrix)
{
    for i in 0..matrix.len()
    {
        for j in 0..matrix[i].len()
        {
            if matrix[i][j] == Obstacle::Round
            {
                matrix[i][j] = Obstacle::None;
            }
        }
    }
}

fn setup_configuration(matrix: &mut Matrix, configuration: &Configuration)
{
    clear_moveable_obstacles(matrix);

    for configuration in configuration
    {
        matrix[configuration.x][configuration.y] = Obstacle::Round;
    }
}

fn part1(matrix: &mut Matrix) -> u64
{
    move_up(matrix);
    return count_load(matrix);
}

fn part2(matrix: &mut Matrix) -> u64
{
    let mut seen_configurations: HashMap<Configuration, u64> = HashMap::new();

    for i in 0..NUMBER_CYCLES
    {
        let configuration: Configuration = get_configuration(matrix);
        let iteration_option = seen_configurations.get(&configuration);

        if iteration_option.is_some()
        {
            let iteration = iteration_option.unwrap();
            let cycle_count = i - iteration;
            let remaining_cyles = NUMBER_CYCLES - iteration;
            let final_cycle = remaining_cyles % cycle_count + iteration;

            let final_configuration = seen_configurations.iter().find_map(|(key, &val)| if val == final_cycle { Some(key) } else { None }).unwrap();
            setup_configuration(matrix, &final_configuration);
            break;
        }
        else
        {
            seen_configurations.insert(configuration, i);
            cycle(matrix);
        }        
    }
    
    return count_load(matrix);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut matrix: Matrix = parse(&contents);

    let part1_result: u64 = part1(&mut matrix);
    let part2_result: u64 = part2(&mut matrix);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}