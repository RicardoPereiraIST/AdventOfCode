use std::fs;

const FILE_PATH : &str = "input.txt";

struct SRace
{
    time_ms: u64,
    distance_mm: u64
}

struct SRoot
{
    root_1: f64,
    root_2: f64
}

trait Parse
{
    fn parse(line: &str) -> Vec<u64>;
}

trait Solve
{
    fn solve(self) -> SRoot;
}

struct IndividualParser;
struct KerningParser;

impl Parse for IndividualParser
{
    fn parse(line: &str) -> Vec<u64>
    {
        return line.split(" ").filter(|s| !s.is_empty())
                        .filter_map(|s| Some(s.parse::<u64>().unwrap())).collect::<Vec<u64>>();
    }
}

impl Parse for KerningParser
{
    fn parse(line: &str) -> Vec<u64>
    {
        let value = line.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>().join("").parse::<u64>().unwrap();
        return vec![value];
    }
}

struct SQuadraticFunction
{
    a: f64,
    b: f64,
    c: f64
}

impl Solve for SQuadraticFunction
{
    fn solve(self) -> SRoot
    {
        let root_1 = (-self.b + (((self.b.powf(2.0) - 4.0 * self.a * self.c).sqrt()))) / (2.0 * self.a);
        let root_2 = (-self.b - (((self.b.powf(2.0) - 4.0 * self.a * self.c).sqrt()))) / (2.0 * self.a);

        return SRoot { root_1: root_1, root_2: root_2 };
    }
}

fn parse_races<T: Parse>(time_str: &str, distance_str: &str) -> Vec<SRace>
{
    let mut races: Vec<SRace> = Vec::new();

    let times_str = time_str.split(":").collect::<Vec<&str>>()[1];
    let distances_str = distance_str.split(":").collect::<Vec<&str>>()[1];
    
    let times = T::parse(times_str);
    let distances = T::parse(distances_str);

    assert_eq!(times.len(), distances.len());

    for i in 0..times.len()
    {
        races.push(
            SRace
            {
                time_ms: times[i],
                distance_mm: distances[i]
            }
        );
    }

    return races;
}

fn solve_race(race: &SRace) -> u64
{
    let quadratic_formula = SQuadraticFunction{a: 1.0, b: -(race.time_ms as f64), c: race.distance_mm as f64};
    let roots = quadratic_formula.solve();
    let max_zero = f64::max(roots.root_1, roots.root_2).ceil() as u64;
    let min_zero = f64::min(roots.root_1, roots.root_2).floor() as u64;
    return max_zero - 1 - min_zero;
}

fn part1(time_str: &str, distance_str: &str) -> u64
{
    let mut result: u64 = 1;
    let races = parse_races::<IndividualParser>(time_str, distance_str);
    
    for race in races
    {
        result *= solve_race(&race);
    }

    return result;
}

fn part2(time_str: &str, distance_str: &str) -> u64
{
    let races = parse_races::<KerningParser>(time_str, distance_str);
    let race = races.first().unwrap();

    return solve_race(&race);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut lines = contents.lines();
    let (time_str, distance_str) = (lines.next().unwrap(), lines.next().unwrap());

    let part1_result: u64 = part1(&time_str, &distance_str);
    let part2_result: u64 = part2(&time_str, &distance_str);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}