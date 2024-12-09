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
            (Byte::Empty(free), Byte::Occupied{size, id}) => {
                if free > size {
                    let size = *size;
                    let free = *free;
                    let id = *id;
                    i[left] = Byte::Empty(free - size);
                    let r = i.remove(right);
                    i.insert(left, r);
                    //right -= 1; There has been an insert one insert earlier
                    left += 1;
                } else if free == size {
                    i[left] = i.remove(right);
                    right -= 1;
                    left += 1;
                } else {
                    let free = *free;
                    let size = *size;
                    let id = *id;
                    i[left] = Byte::Occupied{size: free, id: id};
                    i[right] = Byte::Occupied{size: size - free, id: id};
                    left += 1;
                }
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

pub fn part2(i: &Input) -> usize {
    todo!();
}
