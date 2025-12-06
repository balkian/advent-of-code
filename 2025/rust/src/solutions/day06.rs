#[derive(Debug, Clone)]
pub enum Operation {
    Sum,
    Multiply,

}

impl Operation {
    fn reduce(&self, nums: &[usize]) -> usize {
        match self {
            Operation::Sum => {
                nums.iter().sum()
            }
            Operation::Multiply => {
                nums.iter().product()
            }
        }
    }
}

type Input = (Vec<Vec<usize>>,Vec<Operation>);

pub fn parse(i: &str) -> Input {
    let mut lines = i.lines();
    let mut cols: Vec<Vec<usize>> = vec![];
    let mut operations = None;
    for line in lines.by_ref() {
        let numbers: Option<Vec<_>> = line.split_whitespace().map(|num| num.parse::<usize>().ok()).collect();
        if let Some(nums) = numbers {
            if cols.is_empty() {
                for num in nums {
                    cols.push(vec![num]);
                }
            } else {
                for (ix, num) in nums.into_iter().enumerate() {
                    cols[ix].push(num);
                }
            }
        } else {
            let ops: Vec<_> = line.split_whitespace().map(|op| 
                match op {
                    "*" => Operation::Multiply,
                    "+" => Operation::Sum,
                    _ => panic!("wrong operation {op}")
                }
            ).collect();
            operations = Some(ops);
            assert!(lines.next().is_none());
            break;
        }
    }
    (cols, operations.expect("operations not received"))
}

pub fn part1(i: &Input) -> usize {
    i.0.clone().into_iter().zip(i.1.clone()).map(|(col, op)| {
        op.reduce(&col)
    }).sum()
}

pub fn part2(i: &Input) -> usize {
    todo!();
}
