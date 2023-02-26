use std::fmt;
use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("./example.txt").unwrap();

    let start = Instant::now();
    let stack_size = get_stack_size(&input);

    let stacks = parse_cargo_stacks(&input, stack_size);

    let mut commands: Vec<Command> = Vec::new();
    for line in input.lines()
        .skip_while(|l| !l.is_empty())
            .skip_while(|l| l.is_empty()) {
        commands.push(parse_command(line));
    }
    
    print!("moving crates one by one: ");
    Crane::execute_commands(&mut stacks.clone(), commands.clone(), &mut Crane::move_crate_one_by_one);
    println!();
    print!("moving multiple crates at a time: ");
    Crane::execute_commands(&mut stacks.clone(), commands.clone(), &mut Crane::move_multiple_crates_at_once);
    println!();

    let duration = start.elapsed();
    println!("duration: {:?}", duration);
}

fn print_result(stacks: &Vec<Stack<char>>) {
    for stack in stacks {
        if stack.size() > 0 {
            print!("{}", stack.last().unwrap());
        }
    }
    println!();
}

fn parse_command(line: &str) -> Command {
    let elements: Vec<&str> = line.split(" ").collect();
    Command {
        count: elements[1].parse::<usize>().unwrap(),
        from: elements[3].parse::<usize>().unwrap(),
        to: elements[5].parse::<usize>().unwrap()
    }
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_stack_size(input: &String) -> usize {
    let last_line_before_empty_line = input.lines().take_while(|l| !l.is_empty()).reduce(|_, e| e).unwrap();
    let (_, crates_size) = last_line_before_empty_line.trim().rsplit_once(" ").unwrap();

    return crates_size.parse::<usize>().unwrap();
}

fn parse_horizontal_stack(line: &str) -> Vec<char> {
    const START_IDX: usize = 1;
    let line_len = line.len();
    let mut out = vec![];
    for i in (START_IDX..line_len).step_by(4) {
        let t = line.chars().nth(i).unwrap();
        out.push(t);
    }
    return out;
}


fn parse_cargo_stacks(input: &String, stacks_size: usize) -> Vec<Stack<char>> {
    let mut stacks: Vec<Stack<char>> = Vec::with_capacity(stacks_size);
    stacks.fill_with(Stack::new);
    for _ in 0..stacks_size {
        stacks.push(Stack::new());
    }

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let horizontal_stack = parse_horizontal_stack(line);
        for (i, good) in horizontal_stack.into_iter().enumerate() {
            if good != ' ' && !good.is_numeric() {
                stacks.get_mut(i).unwrap().push(good);
            }
        }
    }

    for i in 0..stacks_size {
        stacks[i].reverse();
    }
    return stacks;
}

#[derive(Debug,Clone)]
struct Command {
    count: usize,
    from: usize,
    to: usize
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.count, self.from, self.to)
    }
}

#[derive(Debug,Clone)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            data: vec![]
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, data: T) {
        self.data.push(data);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn reverse(&mut self) {
        self.data.reverse();
    }
    
    fn last(&self) -> Option<&T> {
        self.data.last()
    }

    fn append(&mut self, other: &mut Vec<T>) {
        self.data.append(other);
    }
}

struct Crane {

}

impl Crane {
    fn execute_commands<F>(stacks: &mut Vec<Stack<char>>, commands: Vec<Command>, executor: &mut F)
        where F: Fn(&mut Vec<Stack<char>>, Command) {
            for command in commands {
                Crane::execute_command(stacks, command, executor);
            }
            print_result(&stacks);
    }

    fn execute_command<F>(stacks: &mut Vec<Stack<char>>, command: Command, executor: &mut F)
        where F: Fn(&mut Vec<Stack<char>>, Command) {
        executor(stacks, command);
    }

    fn move_crate_one_by_one(stacks: &mut Vec<Stack<char>>, command: Command) {
        for _ in 0..command.count {
            let top_of_stack = stacks[command.from - 1].pop().unwrap();
            stacks[command.to - 1].push(top_of_stack);
        }
    }

    fn move_multiple_crates_at_once(stacks: &mut Vec<Stack<char>>, command: Command) {
        let mut moveables = vec![];
        for _ in 0..command.count {
            moveables.push(stacks[command.from - 1].pop().unwrap());
        }
        moveables.reverse();
        stacks[command.to - 1].append(&mut moveables);
    }
}

