use nom::{                                                        
    branch::alt,
    bytes::complete::tag,                                         
    character::complete::{alphanumeric1, digit1, multispace1, newline, space0, space1},   
    combinator::{eof, map_res, recognize},                             
    multi::{separated_list1},                                     
    sequence::{delimited, terminated, separated_pair, tuple},                 
    IResult,                                                      
};                                                                

use std::ops::Range;
use std::cmp::min;
use std::cmp::Ordering;

                                                                  

fn unsigned(input: &str) -> IResult<&str, usize> {                      
    map_res(recognize(digit1), str::parse)(input)                       
}                                                                       
                                                                        
fn numbers(input: &str) -> IResult<&str, Vec<usize>> {                  
    delimited(space0, separated_list1(space1, unsigned), space0)(input) 
}                                                                       

#[derive(Debug, PartialEq, Eq)]
struct Transformation {
    range: Range<usize>,
    offset: isize,
}

impl Ord for Transformation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl PartialOrd for Transformation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Transformation {
    fn get(&self, key: usize) -> usize {
        ((key as isize) + self.offset) as usize
    }
}

#[derive(Debug)]
pub struct Map<'a> {
    from: &'a str,
    to: &'a str,
    ranges: Vec<Transformation>,
}

impl<'a> Map<'a> {
    fn get_transform(&self, key: usize) -> Result<usize, usize> {
        self.ranges.binary_search_by(|transform| {
            if transform.range.contains(&key) {
                Ordering::Equal
            } else {
                transform.range.start.cmp(&key)
            }
        })
        
    }
    fn get(&self, key: usize) -> usize {
        if let Ok(ix) = self.get_transform(key) {
            self.ranges[ix].get(key)
        } else {
            key
        }
    }

    fn get_ranges(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut chunks = vec![];
        let mut range = range;
        let total_length = range.len();
        let idx = match self.get_transform(range.start) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        
        for idx in idx..self.ranges.len() {
            if range.is_empty() {
                break;
            }
            let transform = &self.ranges[idx];
            if range.start < transform.range.start {
                let end = min(range.end, transform.range.start);
                chunks.push(range.start..end);
                range = end..range.end;
                if range.is_empty() {
                    break;
                }
            } 
                let start = range.start;
                let end = min(range.end, transform.range.end);
                chunks.push(transform.get(start)..transform.get(end));
                range = end..range.end;
        }

        if !range.is_empty() {
            chunks.push(range);
        }
        let output_length = chunks.iter().map(|chunk| chunk.len()).sum::<usize>();
        debug_assert_eq!(output_length, total_length);
        chunks
    }
}

fn parse_map(input: &str) -> IResult<&str, Map<'_>> {
    let (input, (from, to)) = terminated(separated_pair(alphanumeric1, tag("-to-"), alphanumeric1), tuple((tag(" map:"), multispace1)))(input)?;
    let elem = || terminated(unsigned, space0);
    let (rest, ranges) = terminated(separated_list1(newline, tuple((elem(), elem(), elem()))), alt((multispace1, eof)))(input)?;
    let mut ranges: Vec<Transformation> = ranges.into_iter().map(|(to, from, size)| Transformation{range: from..(from+size), offset: (to as isize - from as isize)}).collect();
    ranges.sort();

    Ok((rest, Map{from, to, ranges}))
}

#[derive(Debug)]
pub struct Definition<'a>(Vec<usize>, Vec<Map<'a>>);

impl<'a> Definition<'a> {
    fn get(&self, key: usize) -> usize {
        let mut key = key;
        // We assume maps are in order (see parse)
        for map in &self.1 {
            key = map.get(key);
        }
        key
    }

    fn lowest_location(&self) -> usize {
        self.0.iter().copied().map(|seed| {
            self.get(seed)
        }).min().unwrap()
    }

    fn lowest_location2(&self) -> usize {
        let mut ranges: Vec<Range<usize>> = self.0.chunks(2).map(|chunk| chunk[0]..(chunk[0]+chunk[1])).collect();
        for map in &self.1 {
            ranges = ranges.into_iter().flat_map(|range| map.get_ranges(range)).collect();
        }
        ranges.into_iter().map(|range| range.start).min().expect("no minimum found")
    }
}


pub fn parse(input: &str) -> Definition<'_> {
    let (mut input, seeds) = delimited(tag("seeds: "), numbers, multispace1)(input).expect("could not parse seeds");
    let mut maps = vec![];
    let mut previous = "seed";
    while let Ok((rest, map)) = parse_map(input) {
        input = rest;
        // Make sure our assumptions about order of maps is correct.
        // This will simplify how we find mappings later on.
        debug_assert!(map.from == previous);
        previous = map.to;
        maps.push(map);
    }

    Definition(seeds, maps)
}

pub fn part1(input: &Definition) -> usize {
    input.lowest_location()
}

pub fn part2(input: &Definition) -> usize {
    input.lowest_location2()
}
