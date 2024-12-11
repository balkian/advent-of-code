#[derive(Debug,Clone)]
pub enum Byte {
    Empty(usize),
    Occupied{size: usize, id: usize},
}

type Input = Vec<Byte>;

pub fn parse(i: &str) -> Input {
    let mut occupied = true;
    let mut res = vec![];
    let mut id = 0;
    for c in i.trim().chars() {
        let n = c.to_digit(10).expect("could not convert number") as usize;
        if occupied {
            res.push(Byte::Occupied{size: n, id});
            id += 1;
        } else {
            res.push(Byte::Empty(n));
        }
        occupied ^= true;
    }
    res
}

pub fn part1(i: &Input) -> usize {
    let mut i = i.clone();
    let mut left = 0;
    let mut right = i.len() - 1; 
    while left < right {
        match (&i[left], &i[right]) {
            (Byte::Occupied{..}, _) => {
                left += 1;
            },
            (_, Byte::Empty(_)) => {
                right -= 1;
            },
            (Byte::Empty(free), Byte::Occupied{size, id: _}) if free > size => {
                    let size = *size;
                    let free = *free;
                    i[left] = Byte::Empty(free - size);
                    let r = i.remove(right);
                    i.insert(left, r);
                    //right -= 1; There has been an insert one insert earlier
                    left += 1;
            },
            (Byte::Empty(free), Byte::Occupied{size, id: _}) if free == size => {
                    i[left] = i.remove(right);
                    right -= 1;
                    left += 1;
            },
            (Byte::Empty(free), Byte::Occupied{size, id}) => {
                    let free = *free;
                    let size = *size;
                    let id = *id;
                    i[left] = Byte::Occupied{size: free, id};
                    i[right] = Byte::Occupied{size: size - free, id};
                    left += 1;
            }
        }
    }
    let mut pos = 0;
    let mut total = 0;
    for block in i {
        match block {
            Byte::Empty(free) => {
                pos += free;
            },
            Byte::Occupied{size, id} => {
                for _ in 0..size {
                    total += pos * id;
                    pos += 1;
                }
            },
        }
    }
    total
}

#[allow(clippy::mut_range_bound)]
pub fn part2(i: &Input) -> usize {
    let mut i = i.clone();
    let mut right = i.len() - 1; 
    while right > 0 {
        let Byte::Occupied{size, id: _} = &i[right] else {
            right -= 1;
            continue;
        };
        for left in 0..right {
            match &i[left] {
                Byte::Empty(free) if free == size => {
                    i.swap(left, right);
                    break;
                },
                Byte::Empty(free) if free > size => {
                    let size = *size;
                    i.insert(left+1, Byte::Empty(free - size));
                    right += 1;
                    i[left] = Byte::Empty(size);
                    i.swap(left, right);
                    break;
                },
                _ => {
                    continue;
                },
            }
        }
        right -= 1;
    }
    let mut pos = 0;
    let mut total = 0;
    for block in i {
        match block {
            Byte::Empty(free) => {
                pos += free;
            },
            Byte::Occupied{size, id} => {
                for _ in 0..size {
                    total += pos * id;
                    pos += 1;
                }
            },
        }
    }
    total
}
