use std::collections::VecDeque;

const W: usize = 25;

fn main() {
    let numbers: Vec<usize> = aoc_utils::file_iter().map(|x|
        x.parse().unwrap()
    ).collect();

    let mut window: VecDeque<usize> = numbers.iter().take(W).map(|x| x.to_owned()).collect();
    let mut vuln: Option<&usize> = None;

    for number in &numbers[W..] {
        // dbg!{&window, &number};
        let mut found = false;
        for p1 in &window {
            if number > p1 && window.contains(&(number-p1)) {
                found = true;
                break
            }
        }
        if !found {
            println!{"First number: {:}", number}
            vuln = Some(number);
            break
        }
        window.push_back(*number);
        if window.len() > W {
            window.pop_front();
        }
    }

    window = VecDeque::new();
    let vuln = *vuln.unwrap();
    let mut sol2:usize = 0;
    for i in numbers {
        window.push_back(i);
        let mut sum: usize = window.iter().sum();
        while sum > vuln{
            sum -= window.pop_front().unwrap();
        }
        if sum == vuln {
            sol2 = window.iter().max().unwrap() + window.iter().min().unwrap();
            break
        }
    }
    println!("{:?}", sol2);
}
