// Idea for part 1 from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day24p1.py
// Idea for part 2 from https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day24/part2.rs

use std::fs;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

const FILE_PATH : &str = "input.txt";
const BOUNDS: (i64, i64) = (200000000000000, 400000000000000);

type Hailstones = Vec<Hailstone>;

#[derive(Clone, Copy, Debug)]
struct Hailstone
{
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64
}

fn parse(contents: &String) -> Hailstones
{
    let mut hailstones: Hailstones = Hailstones::new();

    for line in contents.lines()
    {
        let data_str: Vec<&str> = line.split(" @ ").collect();
        let (position_str, velocity_str) = (data_str[0], data_str[1]);
        let position: Vec<f64> = position_str.split(", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();
        let velocity: Vec<f64> = velocity_str.split(", ").map(|s| s.trim().parse::<f64>().unwrap()).collect();

        hailstones.push(Hailstone
            {
                px: position[0],
                py: position[1],
                pz: position[2],
                vx: velocity[0],
                vy: velocity[1],
                vz: velocity[2],
                a: velocity[1],
                b: -velocity[0],
                c: (velocity[1] * position[0]) - (velocity[0] * position[1])
            });
    }

    return hailstones;
}

fn are_parallel(lhs: &Hailstone, rhs: &Hailstone) -> bool
{
    return (lhs.a * rhs.b) == (lhs.b * rhs.a);
}

fn in_bounds(x: f64, y: f64) -> bool
{
    return ((x >= BOUNDS.0 as f64) && (x <= BOUNDS.1 as f64))
        && ((y >= BOUNDS.0 as f64) && (y <= BOUNDS.1 as f64));
}

fn will_intersect(hailstone: &Hailstone, other_hailstone: &Hailstone, x: f64, y: f64) -> bool
{
    return (((x - hailstone.px as f64) * hailstone.vx as f64) >= 0.0)
        && (((y - hailstone.py as f64) * hailstone.vy as f64) >= 0.0)
        && (((x - other_hailstone.px as f64) * other_hailstone.vx as f64) >= 0.0)
        && (((y - other_hailstone.py as f64) * other_hailstone.vy as f64) >= 0.0);
}

fn calculate_intersections(hailstones: &Hailstones) -> u64
{
    let mut result: u64 = 0;

    for (i, hailstone) in hailstones.iter().enumerate()
    {
        for other_hailstone in hailstones[0..i].iter()
        {
            if are_parallel(hailstone, other_hailstone)
            {
                continue;
            }

            let x: f64 = ((hailstone.c * other_hailstone.b) - (other_hailstone.c * hailstone.b)) / ((hailstone.a * other_hailstone.b) - (other_hailstone.a * hailstone.b));
            let y: f64 = ((other_hailstone.c * hailstone.a) - (hailstone.c * other_hailstone.a)) / ((hailstone.a * other_hailstone.b) - (other_hailstone.a * hailstone.b));
            
            if in_bounds(x, y)
            {
                if will_intersect(&hailstone, &other_hailstone, x, y)
                {
                    result += 1;
                }
            }
        }
    }
    
    return result;
}

fn solve(hailstones: &Hailstones) -> u64
{
    let config = Config::new();
    let context = Context::new(&config);
    let solver = Solver::new(&context);

    let px = Int::new_const(&context, "px");
    let py = Int::new_const(&context, "py");
    let pz = Int::new_const(&context, "pz");
    let vx = Int::new_const(&context, "vx");
    let vy = Int::new_const(&context, "vy");
    let vz = Int::new_const(&context, "vz");

    for hailstone in hailstones
    {
        let hailstone_px = Int::from_i64(&context, hailstone.px as i64);
        let hailstone_py = Int::from_i64(&context, hailstone.py as i64);
        let hailstone_pz = Int::from_i64(&context, hailstone.pz as i64);
        let hailstone_vx = Int::from_i64(&context, hailstone.vx as i64);
        let hailstone_vy = Int::from_i64(&context, hailstone.vy as i64);
        let hailstone_vz = Int::from_i64(&context, hailstone.vz as i64);
        let hailstone_t = Int::fresh_const(&context, "t");

        solver.assert(&(&hailstone_px + &hailstone_vx * &hailstone_t)._eq(&(&px + &vx * &hailstone_t)));
        solver.assert(&(&hailstone_py + &hailstone_vy * &hailstone_t)._eq(&(&py + &vy * &hailstone_t)));
        solver.assert(&(&hailstone_pz + &hailstone_vz * &hailstone_t)._eq(&(&pz + &vz * &hailstone_t)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    return (x + y + z) as u64;
}

fn part1(hailstones: &Hailstones) -> u64
{
    return calculate_intersections(&hailstones);
}

fn part2(hailstones: &Hailstones) -> u64
{
    return solve(&hailstones);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
                .expect("Should have been able to read the file");

    let hailstones: Hailstones = parse(&contents);

    let part1_result: u64 = part1(&hailstones);
    let part2_result: u64 = part2(&hailstones);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}