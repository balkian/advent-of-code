use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Claim {
    /// Identifier
    id: usize,
    /// The number of inches between the left edge of the fabric and the left edge of the rectangle.
    left: usize,
    ///    The number of inches between the top edge of the fabric and the top edge of the rectangle.
    top: usize,
    ///    The width of the rectangle in inches.
    width: usize,
    ///    The height of the rectangle in inches.
    height: usize,
}

impl Claim {
    // This could be done much more easily with regex, but that's an extra dependency :)
    // Example: #123 @ 3,2: 5x4
    fn from_string_manually(input: &str) -> Claim {
        let mut claim = Claim {
            id: 0,
            left: 0,
            top: 0,
            width: 0,
            height: 0,
        };

        let mut buffer: String = String::new();
        for i in input.chars() {
            match i {
                ' ' => {}
                '#' => {
                    buffer.clear();
                }
                '@' => {
                    claim.id = buffer.parse().unwrap();
                    buffer.clear();
                }
                ',' => {
                    claim.left = buffer.parse().unwrap();
                    buffer.clear();
                }
                ':' => {
                    claim.top = buffer.parse().unwrap();
                    buffer.clear();
                }
                'x' => {
                    claim.width = buffer.parse().unwrap();
                    buffer.clear();
                }
                _ => {
                    buffer.push(i);
                }
            }
        }
        claim.height = buffer.parse().unwrap();
        buffer.clear();
        claim
    }

    fn from_string(input: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let cap = re.captures_iter(input).next().unwrap();
        Self {
            id: cap[1].parse().unwrap(),
            left: cap[2].parse().unwrap(),
            top: cap[3].parse().unwrap(),
            width: cap[4].parse().unwrap(),
            height: cap[5].parse().unwrap(),
        }
    }
}


fn get_matrix(input: &str) -> (Vec<Claim>, Vec<Vec<usize>>) {
    let claims: Vec<Claim> = input.lines().map(|x| Claim::from_string(x)).collect();
    let matrix: Vec<Vec<usize>>= claims.iter().fold(vec!(vec!()), |mut mat, c| {
        for i in c.top..c.top+c.height {
            while i >= mat.len() {
                mat.push(vec!(0; c.left+c.width));
            }
            for j in c.left..c.left+c.width {
                while j >= mat[i].len() {
                    mat[i].push(0);
                }
                mat[i][j] += 1;
            }
        }
        mat
    });
    (claims, matrix)
}

fn solve1(input: &str) -> usize {
    let (_, matrix) = get_matrix(input);
    matrix.iter().flatten().filter(|x| **x> 1).count()
}

fn solve2(input: &str) -> usize {
    let (claims, matrix) = get_matrix(input);
    let nonoverlap: Vec<&Claim> = claims.iter().filter(|c| {

        (c.top..c.top+c.height).flat_map(|i|
                                         (c.left..c.left+c.width).map(move |j| (i, j))
        ).all(|(i, j)| matrix[i][j] == 1)
    }).collect();
    debug_assert_eq!{nonoverlap.len(), 1};
    nonoverlap[0].id
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read file");
    dbg!(solve1(&input));
    dbg!(solve2(&input));
}

#[test]
fn test_parse() {
    let cl = Claim::from_string("#123 @ 3,2: 5x4");
    assert_eq!(cl.id, 123);
    assert_eq!(cl.left, 3);
    assert_eq!(cl.top, 2);
    assert_eq!(cl.width, 5);
    assert_eq!(cl.height, 4);
}

#[test]
fn test_sol1() {
    assert_eq!(solve1("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"), 4);

}

#[test]
fn test_sol2() {
    assert_eq!(solve1("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"), 3);

}
