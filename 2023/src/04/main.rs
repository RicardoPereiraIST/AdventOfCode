use std::{fs, collections::{HashMap, BTreeMap}};

const FILE_PATH : &str = "input.txt";

fn parse_scratchcard(scratchcard: &str) -> Vec<u64>
{
    return scratchcard.split(" ").filter(|s| !s.is_empty()).filter_map(|s| Some(s.trim().parse::<u64>().unwrap())).collect::<Vec<u64>>();
}

fn parse_scratchcards(current_game: &mut Vec<u64>, scratchcards: Vec<&str>)
{
    let winning_scratchcards: Vec<u64> = parse_scratchcard(scratchcards[0]);
    let mut elf_scratchcards: Vec<u64> = parse_scratchcard(scratchcards[1]);

    elf_scratchcards.retain(|e| winning_scratchcards.contains(e));
    current_game.extend(elf_scratchcards);
}

fn create_game_map(contents: &String) -> HashMap<u64, Vec<u64>>
{
    let mut game_map: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in contents.lines()
    {
        let (id_str, scratchcards_str) = line.split_once(": ").unwrap();
        let id: u64 = id_str.split(" ").last().unwrap().parse::<u64>().unwrap();
        let scratchcards: Vec<&str> = scratchcards_str.split(" | ").collect();

        let mut current_game = game_map.entry(id).or_insert(Vec::new());
        parse_scratchcards(&mut current_game, scratchcards);
    }

    return game_map;
}

fn part1(game_map: &HashMap<u64, Vec<u64>>) -> u64
{
    let mut result: u64 = 0;

    for (_, winning_numbers) in game_map
    {
        let scratchcard_amount = winning_numbers.len() as u32;

        if scratchcard_amount == 0
        {
            continue;
        }
        else
        {
            result += 2u64.pow(scratchcard_amount - 1);
        }
    }

    return result;
}

fn part2(game_map: &HashMap<u64, Vec<u64>>) -> u64
{
    let matches: BTreeMap<usize, usize> = game_map.iter().map(|elem| (*elem.0 as usize, elem.1.len())).collect();
    let mut computed: Vec<u64> = vec![0; matches.len()];

    for (id, cards) in matches.iter().rev()
    {
        let mut val: u64 = 1;
        for j in 0..*cards
        {
            val += computed[id + j];
        }
        computed[*id - 1] = val;
    }

    return computed.iter().sum();
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let game_map: HashMap<u64, Vec<u64>> = create_game_map(&contents);

    let part1_result: u64 = part1(&game_map);
    let part2_result: u64 = part2(&game_map);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}