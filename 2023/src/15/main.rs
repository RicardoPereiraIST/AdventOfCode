#![feature(linked_list_remove)]

use std::{fs, collections::LinkedList, fmt::Display};

const FILE_PATH : &str = "input.txt";

type Bucket<TKey, TValue> = LinkedList<(TKey, TValue)>;
type THashMapContainer<TKey, TValue> = [Bucket<TKey, TValue>; 256];
type Data<'a> = Vec<&'a str>;

#[derive(PartialEq, Eq)]
enum Operation
{
    Insert,
    Remove
}

struct THashMap<TKey, TValue>
{
    map: THashMapContainer<TKey, TValue>
}

impl<TKey: std::fmt::Debug, TValue: std::fmt::Debug> Display for THashMap<TKey, TValue>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        for (i, bucket) in self.map.iter().enumerate()
        {
            if !bucket.is_empty()
            {
                writeln!(f, "{}: {:?}", i, bucket)?;
            }
        }

        return Ok(());
    }
}

trait THash
{
    fn hash(&self) -> u64;
}

impl THash for String
{
    fn hash(&self) -> u64
    {
        let mut result: u64 = 0;

        for c in self.chars()
        {
            result = ((result + c as u64) * 17) % 256;
        }

        return result;
    }
}

trait Container<TKey, TValue>
{
    fn insert(&mut self, key: &TKey, value: &TValue);
    fn remove(&mut self, key: &TKey);
}

impl<TKey, TValue> Container<TKey, TValue> for THashMap<TKey, TValue>
    where
        TKey: THash + PartialEq<TKey> + Clone,
        TValue: PartialEq<TValue> + Copy
{
    fn insert(&mut self, key: &TKey, value: &TValue)
    {
        let hash = key.hash();
        let bucket = &mut self.map[hash as usize];

        for it in bucket.iter_mut()
        {
            if *key == it.0
            {
                it.1 = *value;
                return;
            }
        }

        bucket.push_back((key.clone(), *value));
    }

    fn remove(&mut self, key: &TKey)
    {
        let hash = key.hash();
        let bucket = &mut self.map[hash as usize];

        for (i, it) in bucket.iter_mut().enumerate()
        {
            if *key == it.0
            {
                bucket.remove(i);
                return;
            }
        }
    }
}

fn parse(contents: &String) -> Data
{
    return contents.split(",").collect();
}

fn check_operation(string: &str) -> Operation
{
    if string.contains(|c| c == '=')
    {
        return Operation::Insert;
    }

    return Operation::Remove;
}

fn setup_hashmap(data: &Data, hashmap: &mut THashMap<String, u64>)
{
    for string in data
    {
        let operation = check_operation(string);

        match operation
        {
            Operation::Insert =>
            {
                let key_value: Vec<&str> = string.split("=").collect();
                let (key, value) = (key_value[0].to_string(), key_value[1].parse::<u64>().unwrap());
                hashmap.insert(&key, &value);
            },
            Operation::Remove =>
            {
                let key_value: Vec<&str> = string.split("-").collect();
                let key = key_value[0].to_string();
                hashmap.remove(&key);
            }
        }
    }
}

fn calculate_power(hashmap: &THashMap<String, u64>) -> u64
{
    let mut result = 0;

    for (i, bucket) in hashmap.map.iter().enumerate()
    {
        for (j, item) in bucket.iter().enumerate()
        {
            result += ((i + 1) * (j + 1)) as u64 * item.1;
        }
    }

    return result;
}

fn part1(data: &Data) -> u64
{
    let mut result = 0;

    for string in data
    {
        result += string.to_string().hash();
    }

    return result;
}

fn part2(data: &Data) -> u64
{
    const BUCKET: Bucket<String, u64> = Bucket::new();
    let mut hashmap: THashMap<String, u64> = THashMap{map: [BUCKET; 256] };

    setup_hashmap(data, &mut hashmap);
    return calculate_power(&hashmap);
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let data: Data = parse(&contents);

    let part1_result: u64 = part1(&data);
    let part2_result: u64 = part2(&data);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}