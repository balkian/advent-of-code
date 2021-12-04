#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<usize>>,
    marked: Vec<usize>,
}

impl Board {
    fn mark(&mut self, num: usize) -> bool {
        if self.grid.iter().flatten().any(|x| *x == num) {
            self.marked.push(num);
            self.check()
        } else {
            false
        }
    }

    fn check(&self) -> bool {
        for row in self.grid.iter() {
            if row.iter().all(|l| self.marked.contains(l)) {
                return true;
            }
        }
        for col in 0..self.grid[0].len() {
            if self.grid.iter().all(|r| self.marked.contains(&r[col])) {
                return true;
            }
        }
        false
    }

    fn calculate(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|n| !self.marked.contains(n))
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

pub fn parse(input: &str) -> Game {
    let mut it = input.lines();

    let numbers: Vec<usize> = it
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    let mut boards = vec![];
    it.next().unwrap();
    loop {
        let board: Vec<Vec<usize>> = it
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        if board.is_empty() {
            break;
        }
        boards.push(Board {
            grid: board,
            marked: vec![],
        })
    }
    Game { numbers, boards }
}

pub fn part1(game: &Game) -> usize {
    let mut game = game.clone();
    for num in game.numbers.iter() {
        for board in game.boards.iter_mut() {
            if board.mark(*num) {
                return board.calculate() * num;
            }
        }
    }
    panic!("solution not found");
}

pub fn part2(game: &Game) -> usize {
    let mut boards = game.boards.clone();
    let mut finished = vec![];
    let mut missing = boards.len();
    for num in game.numbers.iter() {
        for (ix, board) in boards.iter_mut().enumerate() {
            if finished.contains(&ix) {
                continue;
            }
            if board.mark(*num) {
                missing -= 1;
                finished.push(ix);
            }
            if missing == 0 {
                return board.calculate() * num;
            }
        }
    }
    panic!("solution not found");
}
