use std::collections::VecDeque;

fn main() {
    let w_size: usize = std::env::args()
        .nth(2)
        .map(|x| x.parse().expect("invalid window size"))
        .unwrap_or(25);
    let numbers: Vec<usize> = aoc_utils::file_iter()
        .map(|x| x.parse().expect("could not parse number"))
        .collect();

    let mut window: VecDeque<usize> = numbers.iter().take(w_size).map(|x| x.to_owned()).collect();
    let mut vuln: Option<&usize> = None;

    for number in &numbers[w_size..] {
        let mut found = false;
        for p1 in &window {
            if number > p1 && window.contains(&(number - p1)) {
                found = true;
                break;
            }
        }
        if !found {
            vuln = Some(number);
            break;
        }
        window.push_back(*number);
        if window.len() > w_size {
            window.pop_front();
        }
    }

    let vuln = *vuln.expect("vulnerable number not found");
    println!("Part 1: {:}", vuln);

    window = VecDeque::new();
    let mut sum = 0;
    for i in numbers {
        sum += i;
        window.push_back(i);
        while sum > vuln {
            sum -= window
                .pop_front()
                .expect("the sum > vuln but the window is empty?");
        }
        if sum == vuln {
            let max = window.iter().max().expect("empty window (impossible?)");
            let min = window.iter().min().expect("empty window (impossible?)");
            let sol2 = max + min;
            println!("Part 2: {:?}", sol2);
            break;
        }
    }
}
