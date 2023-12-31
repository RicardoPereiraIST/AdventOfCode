use std::{fs, collections::HashMap, cmp::Ordering, marker::PhantomData};

const FILE_PATH : &str = "input.txt";

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Cards
{
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14
}

fn convert_to_card(card_str: char) -> Cards
{
    return match card_str
    {
        '2' => Cards::Two,
        '3' => Cards::Three,
        '4' => Cards::Four,
        '5' => Cards::Five,
        '6' => Cards::Six,
        '7' => Cards::Seven,
        '8' => Cards::Eight,
        '9' => Cards::Nine,
        'T' => Cards::T,
        'J' => Cards::J,
        'Q' => Cards::Q,
        'K' => Cards::K,
        'A' => Cards::A,
        _ => Cards::Joker,
    };
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType
{
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeKind = 4,
    FullHouse = 5,
    FourKind = 6,
    FiveKind = 7,
}

fn to_type(first_card_count: u64, second_card_count: u64) -> HandType
{
    return match first_card_count
    {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => match second_card_count
        {
            2 => HandType::FullHouse,
            _ => HandType::ThreeKind,
        }
        2 => match second_card_count
        {
            2 => HandType::TwoPair,
            _ => HandType::OnePair,
        }
        _ => HandType::HighCard
    }
}

struct Part1;

struct Part2;

trait FindType
{
    fn find_type(card_count: &mut HashMap<Cards, u64>) -> HandType;
}

impl FindType for Part1
{
    fn find_type(card_count: &mut HashMap<Cards, u64>) -> HandType
    {
        let mut counter: Vec<(&Cards, &u64)> = card_count.iter().collect();
        counter.sort_by(|lhs, rhs| rhs.1.cmp(lhs.1));

        let first_card_count: u64 = if !counter.is_empty() { *counter[0].1 } else { 0 };
        let second_card_count: u64 = if counter.len() > 1 { *counter[1].1 } else { 0 };
        return to_type(first_card_count, second_card_count);
    }
}

impl FindType for Part2
{
    fn find_type(card_count: &mut HashMap<Cards, u64>) -> HandType
    {
        let joker_count = card_count.remove(&Cards::J).unwrap_or(0);

        let mut counter: Vec<(&Cards, &u64)> = card_count.iter().collect();
        counter.sort_by(|lhs, rhs| rhs.1.cmp(lhs.1));


        let first_card_count: u64 = if !counter.is_empty() { *counter[0].1 } else { 0 };
        let second_card_count: u64 = if counter.len() > 1 { *counter[1].1 } else { 0 };
        return to_type(first_card_count + joker_count, second_card_count);
    }
}

#[derive(PartialEq, Eq)]
struct SHand<TPart>
{
    cards: Vec<Cards>,
    hand_type: HandType,
    bid: u64,
    part: PhantomData<TPart>
}

trait CompareCard<TPart>
{
    fn compare_card(lhs: &Cards, rhs: &Cards) -> Ordering;
}

impl<TPart> CompareCard<TPart> for Part1
{
    fn compare_card(lhs: &Cards, rhs: &Cards) -> Ordering
    {
        return lhs.cmp(&rhs);
    }
}

impl<TPart> CompareCard<TPart> for Part2
{
    fn compare_card(lhs: &Cards, rhs: &Cards) -> Ordering
    {
        let lhs_card = if lhs == &Cards::J { &Cards::Joker } else { &lhs };
        let rhs_card = if rhs == &Cards::J { &Cards::Joker } else { &rhs };
        return lhs_card.cmp(rhs_card);
    }
}

trait CompareHand<TPart>
{
    fn compare(&self, rhs: &SHand<TPart>) -> Ordering;
}

impl<TPart: CompareCard<TPart>> CompareHand<TPart> for SHand<TPart>
{
    fn compare(&self, rhs: &SHand<TPart>) -> Ordering
    {
        if self.hand_type < rhs.hand_type
        {
            return Ordering::Less;
        }
        else if self.hand_type > rhs.hand_type
        {
            return Ordering::Greater;
        }

        for i in 0..self.cards.len()
        {
            if self.cards[i] != rhs.cards[i]
            {
                return TPart::compare_card(&self.cards[i], &rhs.cards[i]);
            }
        }
        
        return Ordering::Equal;
    }
}

impl <TPart: PartialOrd + CompareCard<TPart>> PartialOrd for SHand<TPart>
{
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
    {
        return Some(CompareHand::<TPart>::compare(self, rhs));
    }
}

impl <TPart: Ord + CompareCard<TPart>> Ord for SHand<TPart>
{
    fn cmp(&self, rhs: &Self) -> Ordering
    {
        return CompareHand::<TPart>::compare(self, rhs)
    }
}

fn calculate_hand_type<TPart: FindType>(cards: &Vec<Cards>) -> HandType
{
    let mut card_count: HashMap<Cards, u64> = HashMap::new();

    for card in cards
    {
        *card_count.entry(card.clone()).or_insert(0) += 1;
    }

    return TPart::find_type(&mut card_count);
}

fn parse_hand<TPart: FindType>(line: &str) -> SHand<TPart>
{
    let (hand_str, bid_str) = line.split_once(" ").unwrap().try_into().unwrap();
    let cards = hand_str.chars().filter_map(|c| Some(convert_to_card(c))).collect::<Vec<Cards>>();
    let bid: u64 = bid_str.parse::<u64>().unwrap();
    let hand_type = calculate_hand_type::<TPart>(&cards);

    return SHand { cards: cards, hand_type: hand_type, bid: bid, part: PhantomData };
}

fn parse_hands<TPart: FindType>(contents: &str) -> Vec<SHand<TPart>>
{
    let mut hands = Vec::new();

    for line in contents.lines()
    {
        hands.push(parse_hand(line));
    }

    return hands;
}

fn part1<TPart: FindType + CompareCard<TPart>>(contents: &str) -> u64
{
    let mut result: u64 = 0;
    let mut hands = parse_hands::<Part1>(&contents);

    hands.sort_by(|lhs, rhs| lhs.compare(rhs));

    for (i, hand) in hands.iter().enumerate()
    {
        result += hand.bid * (i as u64 + 1);
    }

    return result;
}

fn part2<TPart: FindType + CompareCard<TPart>>(contents: &str) -> u64
{
    let mut result: u64 = 0;
    let mut hands = parse_hands::<Part2>(&contents);

    hands.sort_by(|lhs, rhs| lhs.compare(rhs));

    for (i, hand) in hands.iter().enumerate()
    {
        result += hand.bid * (i as u64 + 1);
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let part1_result: u64 = part1::<Part1>(&contents);
    let part2_result: u64 = part2::<Part2>(&contents);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}