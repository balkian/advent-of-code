use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Write;

pub type Amphipod = char;

#[derive(Clone, Eq)]
pub struct Hallway {
    cost: usize,
    estimate: Option<usize>,
    cells: Vec<HCell>,
}

impl PartialEq for Hallway {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Hallway {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hallway {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.estimate()).cmp(&(self.cost + self.estimate()))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HCell {
    Empty,
    Occupied(Amphipod),
    Entrance(Room),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Room {
    target: Amphipod,
    positions: Vec<Option<char>>,
    complete: bool,
    intruders: bool,
    next_empty: Option<usize>,
}

impl Room {
    fn done(&self) -> bool {
        self.positions.iter().all(|c| c == &Some(self.target))
    }

    fn new(species: char, positions: Vec<Amphipod>) -> Self {
        let mut room = Room {
            target: species,
            complete: false,
            intruders: false,
            next_empty: None,
            positions: positions
                .into_iter()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect(),
        };
        room.calculate();
        room
    }

    fn calculate(&mut self) {
        self.complete = self
            .positions
            .iter()
            .all(|a| matches!(a, Some(pod) if pod == &self.target));
        self.intruders = self
            .positions
            .iter()
            .any(|a| matches!(a, Some(pod) if pod != &self.target));
        for (idx, val) in self.positions.iter().enumerate() {
            if val.is_none() {
                self.next_empty = Some(idx);
            } else {
                break;
            }
        }
    }

    fn extract(&self) -> Option<(Amphipod, Self, usize)> {
        if self
            .positions
            .iter()
            .all(|c| c.is_none() || c == &Some(self.target))
        {
            return None;
        }
        if let Some((idx, _elem)) = self
            .positions
            .iter()
            .enumerate()
            .find(|(_, pod)| pod.is_some())
        {
            let mut new_room = self.clone();
            let pod = new_room.positions.get_mut(idx).unwrap().take().unwrap();
            new_room.calculate();
            return Some((pod, new_room, 1 + idx));
        }
        None
    }

    fn where_fit(&self, other: Amphipod) -> Option<usize> {
        if self.target != other || self.intruders {
            None
        } else {
            self.next_empty
        }
    }
}

impl std::fmt::Debug for Hallway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cost: {}", self.cost)?;
        write!(f, "{}", self.repr())?;
        Ok(())
    }
}

impl Hallway {
    fn repr(&self) -> String {
        let mut f = String::new();
        let max_level = self
            .cells
            .iter()
            .map(|c| match c {
                HCell::Entrance(room) => room.positions.len(),
                _ => 1,
            })
            .max()
            .unwrap();
        for level in 0..=max_level {
            for i in &self.cells {
                let cs = match i {
                    HCell::Empty | HCell::Entrance(_) if level == 0 => ".".to_string(),
                    HCell::Occupied(fella) if level == 0 => fella.to_string(),
                    HCell::Entrance(room) if level > 0 => room
                        .positions
                        .get(level - 1)
                        .map(|c| match c {
                            Some(c) => c.to_string(),
                            None => ".".to_string(),
                        })
                        .unwrap(),
                    _ => "#".to_string(),
                };
                write!(f, "{}", cs).unwrap();
            }
            writeln!(f).unwrap();
        }
        for i in &self.cells {
            let cs = match i {
                HCell::Entrance(room) => format!("{{{}}}", room.target),
                _ => " ".to_string(),
            };
            write!(f, "{}", cs).unwrap();
        }
        writeln!(f).unwrap();
        f
    }

    fn append(&mut self, pos: usize, start: usize, extra: impl IntoIterator<Item = Amphipod>) {
        if let Some(HCell::Entrance(room)) = self.cells.get_mut(pos) {
            for (delta, value) in extra.into_iter().enumerate() {
                room.positions.insert(delta + start, Some(value));
            }
            room.calculate();
        } else {
            panic!("Trying to update something that is not a room!");
        }
    }

    fn done(&self) -> bool {
        self.cells.iter().all(|c| match c {
            HCell::Empty => true,
            HCell::Entrance(room) => room.done(),
            _ => false,
        })
    }

    fn estimate(&self) -> usize {
        self.estimate.unwrap()
    }

    fn update_estimate(&mut self) {
        self.estimate = Some(
            self.cells
                .iter()
                .enumerate()
                .map(|(idx, c)| match c {
                    // HCell::Empty => 0isize,
                    HCell::Occupied(t) => estimate(t, idx),
                    HCell::Entrance(room) => room
                        .positions
                        .iter()
                        .map(|p| match p {
                            Some(t) => estimate(t, idx),
                            _ => 0,
                        })
                        .sum(),
                    _ => 0,
                })
                .sum(),
        )
    }

    fn evolve(&self) -> Vec<Self> {
        let mut results = vec![];
        for i in 0..(self.cells.len()) {
            for mut r in self.try_move(i) {
                r.update_estimate();
                results.push(r);
            }
        }
        results
    }

    fn find_home(&self, fella: Amphipod, idx: usize, can_hallway: bool) -> Vec<Hallway> {
        // eprintln!("Finding a home for {}", fella);
        let mut options = vec![];
        let mut home: Option<(Room, usize, usize)> = None;
        for positive in [false, true] {
            for delta in 1..self.cells.len() {
                let i = if positive {
                    idx + delta
                } else {
                    idx.overflowing_sub(delta).0
                };
                match self.cells.get(i) {
                    None | Some(HCell::Occupied(_)) => break,
                    Some(HCell::Entrance(room)) if room.target == fella => {
                        if let Some(idx) = room.where_fit(fella) {
                            let mut new_room = room.clone();
                            new_room.positions[idx] = Some(fella);
                            new_room.calculate();

                            home = Some((new_room, i, cost(&fella) * (delta + idx + 1)));
                            // We can break anyway, we know we won't find the room further
                        }
                        if !can_hallway {
                            break;
                        }
                    }
                    Some(HCell::Empty) if can_hallway => {
                        let mut replacement = self.clone();
                        replacement.cells[i] = HCell::Occupied(fella);
                        replacement.cost += delta * cost(&fella);
                        options.push(replacement);
                    }
                    _ => {}
                }
            }
        }
        if let Some((new_room, i, cost)) = home {
            let mut new_hallway = self.clone();
            new_hallway.cells[i] = HCell::Entrance(new_room);
            new_hallway.cost += cost;
            return vec![new_hallway];
        }
        options
    }

    fn try_move(&self, idx: usize) -> Vec<Hallway> {
        match &self.cells[idx] {
            HCell::Empty => return vec![],
            HCell::Entrance(ref room) => {
                // Try to take the uppermost amphipod
                if let Some((fella, new_room, delta_extract)) = room.extract() {
                    let mut emptied = self.clone();
                    emptied.cells[idx] = HCell::Entrance(new_room);
                    emptied.cost += cost(&fella) * delta_extract;
                    let emptied = emptied;
                    return emptied.find_home(fella, idx, true);
                }
                vec![]
            }
            &HCell::Occupied(fella) => {
                // Try to find it a home
                let mut emptied = self.clone();
                emptied.cells[idx] = HCell::Empty;
                let emptied = emptied;
                emptied.find_home(fella, idx, false)
            }
        }
    }
}

const fn cost(pod: &Amphipod) -> usize {
    match pod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Unknown amphipod type"),
    }
}

const fn position(pod: &Amphipod) -> usize {
    match pod {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!("Unknown amphipod type"),
    }
}

fn estimate(pod: &Amphipod, idx: usize) -> usize {
    let pos = position(pod);
    let delta = match pos.cmp(&idx) {
        Ordering::Less => (idx - pos),
        Ordering::Greater => (pos - idx),
        _ => return 0,
    };
    delta * cost(pod)
}

macro_rules! cell_content {
    (.) => {
        '.'
    };
    ($lit:literal) => {
        $lit
    };
    ($name:ident) => {
        stringify!($name).chars().next().unwrap()
    };
}

macro_rules! cell_unit {
    ([$target:tt : $($text:tt)+]) => {
        HCell::Entrance(Room::new(cell_content!($target), vec![$(cell_content!($text)),+] ))
    };
    (.) => {
        HCell::Empty
    };
    ($text:tt) => {
        HCell::Occupied(cell_content!($text))
    };
}

macro_rules! hallway {
    ($($x:tt)+) => {{
        Hallway {
            cost: 0,
            estimate: None,
            cells: vec!(
                $(cell_unit!($x)),+
            )
        }
    }};
}

pub fn parse(_input: &str) -> Hallway {
    return hallway![. . ['A': 'D' 'C'] . ['B': 'B' 'C'] . ['C': 'B' 'D'] . ['D': 'A' 'A'] . . ];
}

fn update4part2(input: &Hallway) -> Hallway {
    let mut out = input.clone();

    out.append(2, 1, vec!['D', 'D']);
    out.append(4, 1, vec!['C', 'B']);
    out.append(6, 1, vec!['B', 'A']);
    out.append(8, 1, vec!['A', 'C']);
    out
}

fn solve(input: &Hallway) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(input.clone());

    while let Some(candidate) = heap.pop() {
        // dbg!(&candidate);
        if candidate.done() {
            return candidate.cost;
        }
        heap.extend(candidate.evolve().into_iter());
    }
    panic!("Solution not found!");
}

pub fn part1(input: &Hallway) -> usize {
    solve(input)
}

pub fn part2(input: &Hallway) -> usize {
    let input = update4part2(input);
    solve(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_home() {
        let hall = hallway!(['A': 'B'] . ['B': '.']);
        let expected = hallway!(['A': '.'] . ['B': 'B']);
        let opts = hall.evolve();

        dbg!(&opts);
        dbg!(opts.len());
        dbg!(&opts[0].cells);
        assert!(opts.iter().find(|c| c.repr() == expected.repr()).is_some())
    }

    #[test]
    fn test_home_hallway() {
        let hall = hallway!(B . [A: .] . [B: .]);
        let expected = hallway!(. . [A: .] . [B: B]);
        let opts = hall.evolve();

        dbg!(hall);
        dbg!(&opts);
        dbg!(opts.len());
        let result = &opts[0];
        dbg!(&result.cells);
        assert_eq!(result.repr(), expected.repr());
        assert_eq!(result.cost, 5 * cost(&'B'));
    }

    #[ignore]
    #[test]
    fn test_example() {
        let hall = hallway![. . [A: B A] . [B: C D] . [C: B C] . [D: D A] . . ];
        assert_eq!(part1(&hall), 12521);
    }
}
