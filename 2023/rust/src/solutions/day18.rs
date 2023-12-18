



#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Instruction<'a> {
    direction: Direction,
    count: usize,
    color: &'a str,
}

impl<'a> Instruction<'a> {
    fn convert(&self) -> Instruction<'_> {
        let color = self.color.trim_matches(&['#', '(', ')'] as &[_]);        
        let (count, direction) = color.split_at(5);
        let count = usize::from_str_radix(count, 16).expect("invalid number {count}");
        let direction = match direction {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("invalid direction"),
        };
        Instruction{count, direction, color: self.color}
    }

}

pub fn parse(input: &str) -> Vec<Instruction<'_>> {
    input.lines().filter(|line| !line.is_empty())
    .map(|line| {
            let tokens: Vec<_> = line.split_whitespace().collect();
            let direction = match tokens[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                a => panic!("unknown direction {a}"),
            };
            let count = tokens[1].parse::<usize>().expect("could not split number");
            Instruction{direction, count, color: tokens[2]}
        }).collect()
}

pub fn solve_vertices(instructions: &[Instruction]) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut perimeter: usize = 0;

    let mut vertices = vec![];


    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                y -= instruction.count as isize;
            },
            Direction::Down => {
                y += instruction.count as isize;
            },
            Direction::Right => {
                x += instruction.count as isize;
            },
            Direction::Left => {
                x -= instruction.count as isize;
            }
        };
        perimeter += instruction.count;
        vertices.push((y, x));
    }


    // Shoelace formula
    // A = (1/2) * sum((x-x')*(y+y'))

    // Pick's formula:
    // A = i + b/2 - 1
    //
    // 2*A = 2*i + b - 2
    // 2*A + b = 2*i + 2*b - 2
    // 2*i+2*b = 2*A + b + 2
    // i+b = (2*A+b+2)/2

    let twice_area: usize = (vertices.windows(2).map(|ps| {
        // acc + ps[0].0*ps[1].1-ps[1].0*ps[0].1
        (ps[1].1-ps[0].1)*(ps[1].0+ps[0].0)
    }).sum::<isize>().abs()).try_into().expect("could not convert area");

    (twice_area + perimeter+ 2) / 2
}

pub fn part1(input: &[Instruction]) -> usize {
    solve_vertices(input)
}

pub fn part2(input: &[Instruction]) -> usize {
    let new_instructions: Vec<_> = input.iter().map(Instruction::convert).collect();
    solve_vertices(&new_instructions)
}
