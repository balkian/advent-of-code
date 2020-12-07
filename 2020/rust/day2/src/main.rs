use regex::Regex;

#[derive(Debug)]
struct Password {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl Password {
    fn valid_1(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }
    fn valid_2(&self) -> bool {
        // dbg!{&self.password, &self.min, &self.max, &self.password.len()};
        let c1 = self.password.chars().nth((self.min as usize) - 1).unwrap();
        let c2 = self.password.chars().nth((self.max as usize) - 1).unwrap();

        (c1 == self.letter) ^ (c2 == self.letter)
    }
}

fn solve<T>(it: T)
where
    T: IntoIterator<Item = String>,
{
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d*) (?P<letter>[a-z]+): (?P<password>.+)").unwrap();

    let mut count_1 = 0;
    let mut count_2 = 0;

    for line in it {
        let caps = re.captures(&line).unwrap();
        let p = Password {
            min: caps.name("min").unwrap().as_str().parse().unwrap(),
            max: caps.name("max").unwrap().as_str().parse().unwrap(),
            letter: caps
                .name("letter")
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap(),
            password: caps.name("password").unwrap().as_str().into(),
        };
        if p.valid_1() {
            count_1 += 1;
        }
        if p.valid_2() {
            count_2 += 1;
        }
        // dbg!{&p};
    }
    println!("Valid 1: {:?}", count_1);
    println!("Valid 2: {:?}", count_2);
}

fn main() {
    let it = aoc_utils::file_iter_parsed::<String>();
    solve(it);
}
