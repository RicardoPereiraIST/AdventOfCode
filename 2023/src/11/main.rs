use std::{fs, cmp::{max, min}};

const FILE_PATH : &str = "input.txt";

const PART1_FACTOR: u64 = 2 - 1;
const PART2_FACTOR: u64 = 1000000 - 1;

type Expanded = Vec<u64>;
type Matrix = Vec<Vec<char>>;
type Galaxies = Vec<Position>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Position
{
    x: u64,
    y: u64
}

fn convert_to_matrix(contents: &String) -> Matrix
{
    let mut matrix: Matrix = Matrix::new();
    
    for line in contents.lines()
    {
        matrix.push(line.chars().collect());
    }

    return matrix;
}

fn parse_galaxies(matrix: &Matrix) -> Galaxies
{
    let mut galaxies: Galaxies = Galaxies::new();

    for (i, row) in matrix.iter().enumerate()
    {
        for (j, char) in row.iter().enumerate()
        {
            if *char != '.'
            {
                galaxies.push(Position{x: i as u64, y: j as u64});
            }
        }
    }

    return galaxies;
}

fn find_expanded_rows(matrix: &Matrix) -> Expanded
{
    let mut expanded_rows: Expanded = Vec::new();

    for (i, row) in matrix.iter().enumerate()
    {
        if row.iter().all(|c| *c == '.')
        {
            expanded_rows.push(i as u64);
        }
    }

    return expanded_rows;
}

fn find_expanded_columns(matrix: &Matrix) -> Expanded
{
    let mut expanded_columns: Expanded = Vec::new();

    for column in 0..matrix.first().unwrap().len()
    {
        let mut found = false;

        for row in 0..matrix.len()
        {
            if matrix[row][column] != '.'
            {
                found = true;
                break;
            }
        }

        if !found
        {
            expanded_columns.push(column as  u64);
        }
    }

    return expanded_columns;
}

fn find_expanded(matrix: &Matrix) -> (Expanded, Expanded)
{
    return (find_expanded_rows(&matrix), find_expanded_columns(&matrix));
}

fn update_galaxies_x(galaxies: &mut Galaxies, expanded_rows: &Expanded, factor: u64)
{
    for galaxy in galaxies
    {
        for expanded_row in expanded_rows.iter().rev()
        {
            if galaxy.x > *expanded_row
            {
                galaxy.x += factor;
            }
        }
    }
}

fn update_galaxies_y(galaxies: &mut Galaxies, expanded_columns: &Expanded, factor: u64)
{
    for galaxy in galaxies
    {
        for expanded_column in expanded_columns.iter().rev()
        {
            if galaxy.y > *expanded_column
            {
                galaxy.y += factor;
            }
        }
    }
}

fn update_galaxies(galaxies: &mut Galaxies, expanded_rows: &Expanded, expanded_columns: &Expanded, factor: u64)
{
    update_galaxies_x(galaxies, expanded_rows, factor);
    update_galaxies_y(galaxies, expanded_columns, factor);
}

fn distance(pos1: &Position, pos2: &Position) -> u64
{
    let x_distance = max(pos1.x, pos2.x) - min(pos1.x, pos2.x);
    let y_distance = max(pos1.y, pos2.y) - min(pos1.y, pos2.y);
    return x_distance + y_distance;
}

fn calculate_distances(galaxies: &Galaxies) -> u64
{
    let mut result = 0;

    for i in 0..galaxies.len()
    {
        for j in i+1..galaxies.len()
        {
            result += distance(&galaxies[i], &galaxies[j]);
        }
    }

    return result;
}

fn part1(galaxies: &mut Galaxies, expanded_rows: &Expanded, expanded_columns: &Expanded) -> u64
{
    update_galaxies(galaxies, &expanded_rows, &expanded_columns, PART1_FACTOR);
    return calculate_distances(&galaxies);
}

fn part2(galaxies: &mut Galaxies, expanded_rows: &Expanded, expanded_columns: &Expanded) -> u64
{
    update_galaxies(galaxies, &expanded_rows, &expanded_columns, PART2_FACTOR);
    return calculate_distances(&galaxies);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let matrix = convert_to_matrix(&contents);
    let (expanded_rows, expanded_columns) = find_expanded(&matrix);
    let galaxies: Galaxies = parse_galaxies(&matrix);

    let part1_result: u64 = part1(&mut galaxies.clone(), &expanded_rows, &expanded_columns);
    let part2_result: u64 = part2(&mut galaxies.clone(), &expanded_rows, &expanded_columns);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}