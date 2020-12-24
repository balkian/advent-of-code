fn main() {
    let input = "523764819";
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn game(input: &str, size: usize, times: usize) -> Vec<usize> {
    let mut board = vec![0usize; size + 1];

    let digits = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .chain(input.len() + 1..=size);

    let mut ptr = 0;
    let mut last = ptr;

    for this in digits {
        board[last] = this;
        last = this;
    }

    board[last] = board[0];

    for _i in 0..times {
        // if i % 100 == 0 {
        //     println!("Time: {}", i);
        // }
        ptr = board[ptr];
        let start = ptr;

        let mut window = vec![];
        for _ in 0..3 {
            let num = board[ptr];
            window.push(num);
            ptr = num;
        }
        let mut target = start;
        loop {
            target -= 1;
            if target < 1 {
                target = board.len() - 1;
            }
            if !window.contains(&target) {
                break;
            }
        }

        let end = ptr;

        let temp = board[target];
        board[target] = window[0];
        board[start] = board[end];
        board[end] = temp;
        board[0] = ptr;
        ptr = start;
    }
    board
}

#[allow(unused)]
fn print(board: &[usize]) {
    let mut value = board[1];
    // while value != 1 {
    for i in 0..10 {
        print!("{} ", value);
        value = board[value];
    }
    println!();
}

fn part2(input: &str) -> usize {
    let board = game(input, 1_000_000, 10_000_000);
    let p1 = board[1];
    let p2 = board[p1];
    p1 * p2
}
fn part1(input: &str) -> String {
    let board = game(input, 9, 100);
    let mut out = String::new();
    let mut value = board[1];
    while value != 1 {
        out.push_str(&value.to_string());
        value = board[value];
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("389125467"), "67384529".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("389125467"), 149245887792);
    }
}
