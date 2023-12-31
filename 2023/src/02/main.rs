use std::{fs, collections::{HashMap}, cmp::{max}, str::FromStr};

const FILE_PATH : &str = "input.txt";

#[derive(Eq, PartialEq, Hash)]
enum Color
{
    Blue,
    Red,
    Green
}

struct Round
{
    cubes: HashMap<Color, u64>
}

impl FromStr for Color
{
    type Err = ();

    fn from_str(color_str: &str) -> Result<Self, Self::Err>
    {
        return match color_str
        {
            "blue" => Ok(Color::Blue),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            _ => Err(()),
        };
    }
}

fn parse_rounds(current_game: &mut Vec<Round>, rounds: Vec<&str>)
{
    for round in rounds
    {
        current_game.push(Round { cubes: {HashMap::new() } });
        let cubes_str: Vec<&str> = round.split(", ").collect();

        for cube_str in cubes_str
        {
            let (amount_str, color_str) = cube_str.split_once(" ").unwrap();
            let amount = amount_str.parse::<u64>().unwrap();
            let color = Color::from_str(color_str).unwrap();

            let last_element: &mut Round = current_game.last_mut().unwrap();
            *last_element.cubes.entry(color).or_insert(0) += amount;
        }
    }
}

fn create_game_map(contents: &String) -> HashMap<u64, Vec<Round>>
{
    let mut game_map: HashMap<u64, Vec<Round>> = HashMap::new();
    for line in contents.lines()
    {
        let (id_str, rounds_str) = line.split_once(": ").unwrap();
        let id: u64 = id_str.split_once(" ").unwrap().1.parse::<u64>().unwrap();
        let rounds: Vec<&str> = rounds_str.split("; ").collect();

        let mut current_game = game_map.entry(id).or_insert(Vec::new());
        parse_rounds(&mut current_game, rounds);
    }

    return game_map;
}

fn is_possible_round(wanted_configuration: &Round, rounds: &Vec<Round>) -> bool
{
    for round in rounds
    {
        if round.cubes.get(&Color::Red).unwrap_or(&0) > wanted_configuration.cubes.get(&Color::Red).unwrap_or(&0)
            || round.cubes.get(&Color::Green).unwrap_or(&0) > wanted_configuration.cubes.get(&Color::Green).unwrap_or(&0)
            || round.cubes.get(&Color::Blue).unwrap_or(&0) > wanted_configuration.cubes.get(&Color::Blue).unwrap_or(&0)
        {
            return false;
        }
    }
    return true;
}

fn get_min_configuration(rounds: &Vec<Round>) -> (u64, u64, u64)
{
    let mut min_red: u64 = 0;
    let mut min_blue: u64 = 0;
    let mut min_green: u64 = 0;

    for round in rounds
    {
        min_red = max(min_red, *round.cubes.get(&Color::Red).unwrap_or(&0));
        min_blue = max(min_blue, *round.cubes.get(&Color::Blue).unwrap_or(&0));
        min_green = max(min_green, *round.cubes.get(&Color::Green).unwrap_or(&0));
    }

    return (min_red, min_blue, min_green);
}

fn part1(game_map: &HashMap<u64, Vec<Round>>, wanted_configuration: &Round) -> u64
{
    let mut result: u64 = 0;
    
    for (id, rounds) in game_map
    {
        if is_possible_round(&wanted_configuration, rounds)
        {
            result += id;
        }
    }

    return result;
}

fn part2(game_map: &HashMap<u64, Vec<Round>>) -> u64
{
    let mut result: u64 = 0;
    
    for (_, rounds) in game_map
    {
        let (red, blue, green) = get_min_configuration(rounds);
        result += red * blue * green;
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let wanted_configuration = Round
    {
        cubes :
        [
            (Color::Red, 12),
            (Color::Green, 13),
            (Color::Blue, 14),
        ].into()
    };

    let game_map: HashMap<u64, Vec<Round>> = create_game_map(&contents);

    let part1_result: u64 = part1(&game_map, &wanted_configuration);
    let part2_result: u64 = part2(&game_map);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}