// Idea for part 1 from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day22p1.py
// Idea for part 2 from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day22p2.py

use std::{fs, cmp::{max, min}, cell::RefCell, collections::{VecDeque, HashSet}};

const FILE_PATH : &str = "input.txt";

type Bricks = Vec<RefCell<Brick>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position
{
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug)]
struct Brick
{
    start: Position,
    end: Position,
    supports: Vec<usize>,
    is_supported: Vec<usize>
}

fn parse(contents: &String) -> Bricks
{
    let mut bricks: Bricks = Bricks::new();

    for line in contents.lines()
    {
        let coordinates_str: Vec<&str> = line.split("~").collect();
        let (start_str, end_str) = (coordinates_str[0], coordinates_str[1]);
        let start_coordinates: Vec<i64> = start_str.split(",").map(|c| c.parse::<i64>().unwrap()).collect();
        let end_coordinates: Vec<i64> = end_str.split(",").map(|c| c.parse::<i64>().unwrap()).collect();

        let start_pos = Position{x: start_coordinates[0], y: start_coordinates[1], z: start_coordinates[2]};
        let end_pos = Position{x: end_coordinates[0], y: end_coordinates[1], z: end_coordinates[2]};

        bricks.push(Brick
            {
                start: start_pos,
                end: end_pos,
                supports: Vec::new(),
                is_supported: Vec::new()
            }.into());
    }

    bricks.sort_by_key(|brick| brick.borrow().start.z);

    return bricks;
}

fn overlaps(lhs: &Brick, rhs: &Brick) -> bool
{
    return max(lhs.start.x, rhs.start.x) <= min(lhs.end.x, rhs.end.x)
        && max(lhs.start.y, rhs.start.y) <= min(lhs.end.y, rhs.end.y);
}

fn fall_bricks(bricks: &mut Bricks)
{
    for i in 0..bricks.len()
    {
        let mut max_z = 1;

        let mut brick = bricks[i].borrow_mut();
        for j in 0..i
        {
            let other_brick = bricks[j].borrow();
            if overlaps(&brick, &other_brick)
            {
                max_z = max(max_z, other_brick.end.z + 1);
            }
        }

        brick.end.z -= brick.start.z - max_z;
        brick.start.z = max_z;
    }

    bricks.sort_by_key(|brick| brick.borrow().start.z);
}

fn fill_support(bricks: &mut Bricks)
{
    for i in 0..bricks.len()
    {
        let mut upper_brick = bricks[i].borrow_mut();
        for j in 0..i
        {
            let mut lower_brick = bricks[j].borrow_mut();
            if overlaps(&upper_brick, &lower_brick) && upper_brick.start.z == lower_brick.end.z + 1
            {
                lower_brick.supports.push(i);
                upper_brick.is_supported.push(j);
            }
        }

    }
}

fn part1(bricks: &Bricks) -> u64
{
    let mut result: u64 = 0;

    for brick in bricks
    {
        let are_bricks_supported = brick.borrow().supports.iter().all(|id|
            {
                let other_brick = bricks[*id].borrow();
                return other_brick.is_supported.len() > 1;
            });

        if are_bricks_supported
        {
            result += 1;
        }
    }

    return result;
}

fn fill_deque(bricks: &Bricks, brick: &Brick) -> VecDeque<usize>
{
    let mut deque: VecDeque<usize> = VecDeque::new();

    for id in brick.supports.iter()
    {
        let other_brick = bricks[*id].borrow();
        if other_brick.is_supported.len() == 1
        {
            deque.push_back(*id);
        }
    }

    return deque;
}

fn disintegrate(bricks: &Bricks, only_supported_by_one: &mut VecDeque<usize>, falling: &mut HashSet<usize>)
{
    while !only_supported_by_one.is_empty()
    {
        let id = only_supported_by_one.pop_front().unwrap();
        let brick = bricks[id].borrow();

        for i in brick.supports.iter()
        {
            if !falling.contains(i)
            {
                let supported_brick = bricks[*i].borrow();
                if supported_brick.is_supported.iter().all(|brick| falling.contains(brick))
                {
                    only_supported_by_one.push_back(*i);
                    falling.insert(*i);
                }
            }
        }
    }
}

fn part2(bricks: &Bricks) -> u64
{
    let mut result: u64 = 0;
    
    for i in 0..bricks.len()
    {
        let brick = bricks[i].borrow();

        let mut only_supported_by_one: VecDeque<usize> = fill_deque(&bricks, &brick);
        let mut falling: HashSet<usize> = HashSet::from_iter(only_supported_by_one.clone());
        falling.insert(i);

        disintegrate(&bricks, &mut only_supported_by_one, &mut falling);
        
        result += falling.len() as u64 - 1;
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut bricks: Bricks = parse(&contents);
    fall_bricks(&mut bricks);
    fill_support(&mut bricks);

    let part1_result: u64 = part1(&bricks);
    let part2_result: u64 = part2(&bricks);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}