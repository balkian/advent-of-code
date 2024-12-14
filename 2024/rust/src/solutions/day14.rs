use image::{Rgb, RgbImage};
use nom::bytes::complete::tag;
use nom::character::complete::i64 as ni64;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::{many1_count, separated_list1};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

use nalgebra::{point, vector, Point2, Vector2};

const SIZE: (i64, i64) = (101, 103);
const USIZE: (usize, usize) = (101, 103);

type Pos = Point2<i64>;
type Vel = Vector2<i64>;

#[derive(Debug, Clone)]
pub struct Robot {
    #[allow(unused)]
    pos: Pos,
    vel: Vel,
}

impl Robot {
    fn advance(&mut self, size: (i64, i64)) {
        self.pos += self.vel;
        self.pos = point![
            self.pos.coords.x.rem_euclid(size.0),
            self.pos.coords.y.rem_euclid(size.1),
        ];
    }
}

fn coord(i: &str) -> IResult<&str, Pos> {
    let (i, (d1, d2)) = separated_pair(ni64, tag(","), ni64)(i)?;
    Ok((i, point![d1, d2]))
}
fn vect(i: &str) -> IResult<&str, Vel> {
    let (i, (d1, d2)) = separated_pair(ni64, tag(","), ni64)(i)?;
    Ok((i, vector![d1, d2]))
}

fn parse_robot(i: &str) -> IResult<&str, Robot> {
    let (i, (pos, vel)) = separated_pair(
        preceded(tag("p="), coord),
        space1,
        preceded(tag("v="), vect),
    )(i)?;
    Ok((i, Robot { pos, vel }))
}

pub fn parse(i: &str) -> Vec<Robot> {
    let (i, machines) = separated_list1(many1_count(line_ending), parse_robot)(i)
        .expect("could not parse machines");
    assert!(i.trim().is_empty());
    machines
}

fn count<const X: usize, const Y: usize>(robots: &[Robot]) -> [[usize; X]; Y] {
    let mut grid = [[0usize; X]; Y];
    let mut xscale: usize = USIZE.0 / X;
    let mut yscale: usize = USIZE.1 / Y;
    if USIZE.0 % xscale != 0 {
        xscale += 1;
    }
    if USIZE.1 % yscale != 0 {
        yscale += 1;
    }

    for robot in robots.iter() {
        grid[(robot.pos.coords.y as usize) / yscale][(robot.pos.coords.x as usize) / xscale] += 1;
    }
    grid
}

fn draw<const X: usize, const Y: usize>(grid: &[[usize; X]; Y], threshold: usize) -> RgbImage {
    let mut img = RgbImage::new(X as u32, Y as u32);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let val = if *cell > threshold { 255 } else { 0 };
            img.put_pixel(x as u32, y as u32, Rgb([0, val, 0]));
        }
    }
    img
}

pub fn part1(i: &[Robot]) -> usize {
    let mut robots: Vec<Robot> = i.to_vec();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.advance(SIZE);
        }
    }
    const LEFTX: i64 = SIZE.0 / 2;
    const RIGHTX: i64 = LEFTX + 1;
    const UPY: i64 = SIZE.1 / 2;
    const DOWNY: i64 = UPY + 1;
    const MAXX: i64 = SIZE.0;
    const MAXY: i64 = SIZE.1;
    let counts = robots.into_iter().fold([0usize; 4], |mut acc, r| {
        match (r.pos.coords.x, r.pos.coords.y) {
            (0..LEFTX, 0..UPY) => {
                acc[0] += 1;
            }
            (RIGHTX..MAXX, 0..UPY) => {
                acc[1] += 1;
            }
            (0..LEFTX, DOWNY..MAXY) => {
                acc[2] += 1;
            }
            (RIGHTX..MAXX, DOWNY..MAXY) => {
                acc[3] += 1;
            }
            (LEFTX, _) | (_, UPY) => {}
            _ => {
                panic!("unknown range {r:?}");
            }
        }
        acc
    });
    counts.into_iter().product()
}

pub fn part2(i: &[Robot]) -> usize {
    let folder = "img";
    if !std::fs::exists(folder).expect("could not read fs") {
        std::fs::create_dir(folder).expect("could not create directory");
    }
    let mut robots: Vec<Robot> = i.to_vec();
    let thres: usize = 0;
    const X: usize = USIZE.0; // / 5;
    const Y: usize = USIZE.1; // / 5;
    for t in 1..10_000 {
        for robot in robots.iter_mut() {
            robot.advance(SIZE);
        }
        // I got this values by inspecting the images. A better approach would
        // be to measure the variance of X and Y positions, and find the minimum
        if t > 114 && ((t - 65) % USIZE.1) == 0 && ((t - 114) % USIZE.0) == 0 {
            let grid = count::<X, Y>(&robots);
            println!("t={t}");
            let img = draw(&grid, thres);
            img.save(format!("{folder}/{t}.png"))
                .expect("could not save image");
            return t;
        }
    }
    panic!("solution not found");
}
