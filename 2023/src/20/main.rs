use std::{fs, collections::{HashMap, VecDeque}, cell::RefCell};

use num::integer::lcm;

const FILE_PATH : &str = "input.txt";
const BUTTON_PRESS: u64 = 1000;

type Modules = HashMap<String, RefCell<Box<dyn Module>>>;
type StrToU64Map = HashMap<String, u64>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse
{
    Low,
    High
}

#[derive(PartialEq, Eq)]
enum ModuleType
{
    FlipFlop,
    Conjunction,
    Broadcast,
    Button
}

struct Node
{
    modules: Vec<String>
}

struct FlipFlop
{
    node: Node,
    pulse: Pulse
}

struct Conjunction
{
    node: Node,
    connections: HashMap<String, Pulse>
}

struct Broadcast
{
    node: Node,
    pulse: Pulse
}

struct Button
{
    node: Node,
    pulse: Pulse
}

trait Module
{
    fn output_signal(&self) -> Pulse;
    fn update(&mut self, name: &String, pulse: Pulse) -> bool;
    fn get_modules(&self) -> &Vec<String>;
    fn get_type(&self) -> ModuleType;
    fn add_connection(&mut self, name: &String);
    fn reset(&mut self);
}

impl Module for FlipFlop
{
    fn output_signal(&self) -> Pulse
    {
        return self.pulse;
    }

    fn update(&mut self, _: &String, pulse: Pulse) -> bool
    {
        if pulse == Pulse::Low
        {
            if self.pulse == Pulse::Low
            {
                self.pulse = Pulse::High;
            }
            else
            {
                self.pulse = Pulse::Low;
            }
            return true;
        }
        return false;
    }

    fn get_modules(&self) -> &Vec<String>
    {
        return &self.node.modules;
    }

    fn get_type(&self) -> ModuleType
    {
        return ModuleType::FlipFlop;
    }

    fn add_connection(&mut self, _: &String)
    {

    }

    fn reset(&mut self)
    {
        self.pulse = Pulse::Low;
    }
}

impl Module for Conjunction
{
    fn output_signal(&self) -> Pulse
    {
        if self.connections.values().all(|p| p == &Pulse::High)
        {
            return Pulse::Low;
        }

        return Pulse::High;
    }

    fn update(&mut self, name: &String, pulse: Pulse) -> bool
    {
        *self.connections.entry(name.to_string()).or_insert(Pulse::Low) = pulse;
        return true;
    }

    fn get_modules(&self) -> &Vec<String>
    {
        return &self.node.modules;
    }

    fn get_type(&self) -> ModuleType
    {
        return ModuleType::Conjunction;
    }

    fn add_connection(&mut self, name: &String)
    {
        self.connections.insert(name.to_string(), Pulse::Low);
    }

    fn reset(&mut self)
    {
        for (_, pulse) in self.connections.iter_mut()
        {
            *pulse = Pulse::Low;
        }
    }
}

impl Module for Broadcast
{
    fn output_signal(&self) -> Pulse
    {
        return self.pulse;
    }

    fn update(&mut self, _: &String, pulse: Pulse) -> bool
    {
        self.pulse = pulse;
        return true;
    }

    fn get_modules(&self) -> &Vec<String>
    {
        return &self.node.modules;
    }

    fn get_type(&self) -> ModuleType
    {
        return ModuleType::Broadcast;
    }

    fn add_connection(&mut self, _: &String)
    {
        
    }

    fn reset(&mut self)
    {
        self.pulse = Pulse::Low;
    }
}

impl Module for Button
{
    fn output_signal(&self) -> Pulse
    {
        return self.pulse;
    }

    fn update(&mut self, _: &String, _: Pulse) -> bool
    {
        return true;
    }

    fn get_modules(&self) -> &Vec<String>
    {
        return &self.node.modules;
    }

    fn get_type(&self) -> ModuleType
    {
        return ModuleType::Button;
    }

    fn add_connection(&mut self, _: &String)
    {
        
    }

    fn reset(&mut self)
    {
        self.pulse = Pulse::Low;
    }
}

struct Part1
{
    low_pulses: u64,
    high_pulses: u64
}

struct Part2
{
    presses: u64,
    feed: String,
    cycle_len: StrToU64Map,
    seen: StrToU64Map
}

trait Cycle
{
    fn on_cycle(&mut self);
    fn check(&mut self, name: &String, module: &String, pulse: Pulse);
    fn should_return(&self) -> bool;
    fn get_return_value(&self) -> Option<StrToU64Map>;
}

impl Cycle for Part1
{
    fn on_cycle(&mut self)
    {

    }

    fn check(&mut self, _: &String, _: &String, pulse: Pulse)
    {
        if pulse == Pulse::Low
        {
            self.low_pulses += 1;
        }
        else
        {
            self.high_pulses += 1;
        }
    }

    fn should_return(&self) -> bool
    {
        return false;
    }

    fn get_return_value(&self) -> Option<StrToU64Map>
    {
        return None;
    }
}

impl Cycle for Part2
{
    fn on_cycle(&mut self)
    {
        self.presses += 1;
    }

    fn check(&mut self, name: &String, module: &String, pulse: Pulse)
    {
        if module == &self.feed && pulse == Pulse::High
        {
            *self.seen.get_mut(name).unwrap() += 1;

            if !self.cycle_len.contains_key(name)
            {
                self.cycle_len.insert(name.to_string(), self.presses);
            }
        }
    }

    fn should_return(&self) -> bool
    {
        return self.seen.iter().all(|elem| *elem.1 != 0);
    }

    fn get_return_value(&self) -> Option<StrToU64Map>
    {
        return Some(self.cycle_len.clone());
    }
}

fn convert(module_type: char) -> ModuleType
{
    return match module_type
    {
        '&' => ModuleType::Conjunction,
        '%' => ModuleType::FlipFlop,
        'b' => ModuleType::Broadcast,
        _ => ModuleType::Button
    }
}

fn create_node(module_type: ModuleType, modules: &Vec<String>) -> Box<dyn Module>
{
    return match module_type
    {
        ModuleType::Conjunction => Box::new(Conjunction{node: Node {modules: modules.clone()}, connections: Default::default()}),
        ModuleType::FlipFlop => Box::new(FlipFlop{node: Node {modules: modules.clone()}, pulse: Pulse::Low}),
        ModuleType::Broadcast => Box::new(Broadcast{node: Node {modules: modules.clone()}, pulse: Pulse::Low}),
        ModuleType::Button => Box::new(Button{node: Node {modules: modules.clone()}, pulse: Pulse::Low}),
    };
}

fn parse(contents: &String) -> Modules
{
    let button = "button".to_string();
    let mut modules: Modules = Modules::new();

    modules.insert(button.clone(), create_node(ModuleType::Button, &vec!["broadcaster".to_string()]).into());

    for line in contents.lines()
    {
        let modules_str: Vec<&str> = line.split(" -> ").collect();
        let (start_module_str, destination_modules_str) = (modules_str[0], modules_str[1]);
        let (module_type, module_name) = (convert(start_module_str.chars().nth(0).unwrap()), start_module_str.chars().filter(|c| c.is_alphabetic()).collect::<String>());
        let destination_modules: Vec<String> = destination_modules_str.split(", ").map(|s| s.to_string()).collect();
        let node = create_node(module_type, &destination_modules);

        modules.insert(module_name, node.into());
    }

    for (name, module) in modules.iter()
    {
        for destination_module in module.borrow().get_modules()
        {

            let node = modules.get(destination_module);
            if node.is_some()
            {
                let borrow_node = &mut node.unwrap().borrow_mut();
                if borrow_node.get_type() == ModuleType::Conjunction
                {
                    borrow_node.add_connection(name);
                }
            }
        }
    }

    return modules;
}

fn reset(modules: &mut Modules)
{
    for (_, module) in modules
    {
        module.borrow_mut().reset();
    }
}

fn run_cycle<TPart: Cycle>(modules: &Modules, part: &mut TPart) -> Option<StrToU64Map>
{
    let mut deque: VecDeque<String> = vec!["button".to_string()].into();

    part.on_cycle();

    while !deque.is_empty()
    {
        let name = deque.pop_front().unwrap();
        let node = modules.get(&name).unwrap().borrow();
        let pulse = node.output_signal();

        for module in node.get_modules()
        {
            part.check(&name, module, pulse);

            if part.should_return()
            {
                return part.get_return_value();
            }

            let module_node = modules.get(module);
            if module_node.is_some()
            {
                let borrow_node = &mut module_node.unwrap().borrow_mut();
                if borrow_node.update(&name, pulse)
                {
                    deque.push_back(module.to_string());
                }
            }
        }
    }

    return None;
}

fn find_feed(modules: &Modules, end: &String) -> Option<String>
{
    for (name, module) in modules
    {
        if module.borrow().get_modules().contains(end)
        {
            return Some(name.to_string());
        }
    }

    return None;
}

fn initialize(modules: &Modules, feed: &String) -> StrToU64Map
{
    let mut map: StrToU64Map = StrToU64Map::new();

    for (name, module) in modules
    {
        if module.borrow().get_modules().contains(feed)
        {
            map.insert(name.to_string(), 0);
        }
    }

    return map;
}

fn get_cycles(modules: &Modules) -> StrToU64Map
{
    let feed = find_feed(modules, &"rx".to_string()).unwrap();
    
    let mut part: Part2 = Part2
        {
            presses: 0,
            feed: feed.to_string(),
            cycle_len: StrToU64Map::new(),
            seen: initialize(&modules, &feed)
        };

    loop
    {
        let result = run_cycle(modules, &mut part);
        if result.is_some()
        {
            return result.unwrap();
        }
    }
}

fn part1(modules: &mut Modules) -> u64
{
    reset(modules);

    let mut part: Part1 = Part1
        {
            low_pulses: 0,
            high_pulses: 0
        };

    for _ in 0..BUTTON_PRESS
    {
        run_cycle(modules, &mut part);
    }

    return part.low_pulses * part.high_pulses;
}

// LCM idea gotten from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day20p2.py
fn part2(modules: &mut Modules) -> u64
{
    reset(modules);

    let cycle_len: StrToU64Map = get_cycles(&modules);
    let mut result: u64 = 1;

    for value in cycle_len.values()
    {
        result = lcm(result, *value);
    }

    return result;
}

fn main()
{
    let contents: String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut modules: Modules = parse(&contents);

    let part1_result: u64 = part1(&mut modules);
    let part2_result: u64 = part2(&mut modules);
    
    println!("{}", part1_result);
    println!("{}", part2_result);
}