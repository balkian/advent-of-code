
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<&str> {
    input.trim_end_matches('\n').split(',').collect()
}

fn hash(txt: &str) -> usize {
    txt.as_bytes()
        .iter()
        .fold(0, |acc, b| ((acc + *b as usize) * 17) % 256)
}

pub fn part1(steps: &[&str]) -> usize {
    steps.iter().copied().map(hash).sum::<usize>()
}

pub fn part2(steps: &[&str]) -> usize {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = Default::default();

    for step in steps {
        let sep = step
            .find(['-', '='])
            .unwrap_or_else(|| panic!("separator not found {step}"));
        let (lens, num) = step.split_at(sep);
        let num = &num[1..];
        let thisbox = boxes.entry(hash(lens)).or_default();
        let mut lens_ix = None;
        for (ix, (name, _)) in thisbox.iter_mut().enumerate() {
            if name == &lens {
                lens_ix = Some(ix);
                break;
            }
        }
        match &step[sep..sep + 1] {
            "=" => {
                let num = num
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("could not read number from {num}"));
                if let Some(lens_ix) = lens_ix {
                    thisbox[lens_ix].1 = num;
                } else {
                    thisbox.push((lens, num));
                }
            }
            "-" => {
                if let Some(lens_ix) = lens_ix {
                    thisbox.remove(lens_ix);
                }
            }
            _ => panic!("{}", format!("wrong separator {sep}")),
        }
    }
    boxes
        .into_iter()
        .map(|(boxid, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(slot, (_lens, strength))| {
                    // dbg!((lens, boxid+1, slot+1, strength));
                    (boxid + 1) * (slot + 1) * strength
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use aoc_utils::*;
    use super::*;
    #[test]
    fn test_example1() {
        let input = &parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(part1(input), 1320);
        assert_eq!(part1(&example!("day15.example")), 1320);
    }
}

