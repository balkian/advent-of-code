use std::ops::RangeInclusive;

type ValidRange = RangeInclusive<usize>;

pub fn parse(input: &str) -> Vec<ValidRange> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("There should only be ranges");
            start.parse().expect("invalid number")..=end.parse().expect("invalid number")
        })
        .collect()
}

pub fn part1(ranges: &[ValidRange]) -> usize {
    ranges
        .iter()
        .cloned()
        .flat_map(|range| {
            range.filter(|num| {
                let st = num.to_string();
                let st = st.as_bytes();
                let mid = st.len() / 2;

                (mid > 0) && st[..mid] == st[mid..]
            })
        })
        .sum()
}

pub fn part2(ranges: &[ValidRange]) -> usize {
    ranges
        .iter()
        .cloned()
        .flat_map(|range| range.filter(|&num| filter_str(num)))
        .sum()
}

#[allow(unused)]
#[inline]
pub fn filter_num(num: usize) -> bool {
    let mut mask = 10;

    'stems: while mask < num {
        let (stem, mut chunk) = (num % mask, num / mask);
        if mask > 10 && stem / (mask / 10) == 0 {
            mask *= 10;
            continue;
        }

        while chunk > 0 {
            if chunk % mask != stem {
                mask *= 10;
                continue 'stems;
            }
            chunk /= mask;
        }
        return true;
    }
    false
}

#[allow(unused)]
#[inline]
pub fn filter_str(num: usize) -> bool {
    let st = num.to_string();
    let st = st.as_bytes();
    'stems: for stemsize in 1..=st.len() / 2 {
        let mut chunks = st.chunks(stemsize);
        let stem = chunks.next().expect("len should be at least 2");

        for other in chunks {
            if stem != other {
                continue 'stems;
            }
        }
        return true;
    }
    false
}
