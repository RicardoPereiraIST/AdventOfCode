use std::{fs, collections::VecDeque};

const FILE_PATH : &str = "input.txt";

type TSteps = VecDeque<i64>;
type TValues = Vec<TSteps>;
type THistory = Vec<TValues>;

fn parse(contents: &String) -> THistory
{
    let mut history: THistory = THistory::new();
    for line in contents.lines()
    {
        history.push(TValues::new());

        let values: TSteps = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
        history.last_mut().unwrap().push(values);
    }

    return history;
}

fn calculate(history: &mut THistory)
{
    for values in history
    {
        let mut previous_step = values.last().unwrap();
        while !previous_step.iter().all(|elem| elem == &0)
        {
            let mut new_step: TSteps = TSteps::new();
            
            for i in 1..previous_step.len()
            {
                new_step.push_back(previous_step[i] - previous_step[i - 1]);
            }
            
            values.push(new_step);
            previous_step = values.last().unwrap();
        }

        for row in (0..values.len() - 1).rev()
        {
            let next: i64 = *values[row].back().unwrap() + *values[row + 1].back().unwrap();
            let previous: i64 = *values[row].front().unwrap() - *values[row + 1].front().unwrap();

            values[row].push_back(next);
            values[row].push_front(previous);
        }
    }
}

fn part1(history: &THistory) -> i64
{
    let mut result = 0;

    for values in history
    {
        result += values.first().unwrap().back().unwrap();
    }

    return result;
}

fn part2(history: &THistory) -> i64
{
    let mut result = 0;

    for values in history
    {
        result += values.first().unwrap().front().unwrap();
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut history: THistory = parse(&contents);
    calculate(&mut history);

    let part1_result: i64 = part1(&history);
    let part2_result: i64 = part2(&history);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}