use std::fs;

const FILE_PATH : &str = "input.txt";
const PART1_THRESHOLD: u64 = 0;
const PART2_THRESHOLD: u64 = 1;

type Row = Vec<char>;
type Matrix = Vec<Row>;
type Mirrors = Vec<Matrix>;

fn parse(contents: &String) -> Mirrors
{
    let mut mirrors: Mirrors = Mirrors::new();
    let mut matrix: Matrix = Matrix::new();

    for line in contents.lines()
    {
        if line == ""
        {
            mirrors.push(matrix.clone());
            matrix.clear();
            continue;
        }

        matrix.push(line.chars().collect());
    }

    mirrors.push(matrix.clone());

    return mirrors;
}

fn compare(row1: &Row, row2: &Row) -> u64
{
    let mut differences: u64 = 0;

    for i in 0..row1.len()
    {
        differences += (row1[i] != row2[i]) as u64;
    }

    return differences;
}

// Gotten from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
{
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len).map(|_|
            {
                iters.iter_mut().map(|n| n.next().unwrap()).collect::<Vec<T>>()
            }).collect()
}

fn check_mirror(matrix: &Matrix, threshold: u64) -> u64
{
    for i in 1..matrix.len()
    {
        let mut differences: u64 = 0;
        
        let mut previous = i as i64 - 1;
        let mut next = i as i64;

        while (previous >= 0) && ((next as usize) < matrix.len())
        {
            differences += compare(&matrix[previous as usize], &matrix[next as usize]);

            if differences > threshold
            {
                break;
            }

            previous -= 1;
            next += 1;
        }

        if differences == threshold
        {
            return i as u64;
        }
    }

    return 0;
}

fn check_mirrors(mirror: &Matrix, threshold: u64) -> u64
{
    let transposed = transpose(mirror.clone());
    return check_mirror(&mirror, threshold) * 100 + check_mirror(&transposed, threshold);
}

fn part1(mirrors: &Mirrors) -> u64
{
    let mut result: u64 = 0;
    
    for mirror in mirrors
    {
        result += check_mirrors(&mirror, PART1_THRESHOLD);
    }
    
    return result;
}

fn part2(mirrors: &Mirrors) -> u64
{
    let mut result: u64 = 0;
    
    for mirror in mirrors
    {
        result += check_mirrors(&mirror, PART2_THRESHOLD);
    }
    
    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mirrors: Mirrors = parse(&contents);

    let part1_result: u64 = part1(&mirrors);
    let part2_result: u64 = part2(&mirrors);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}