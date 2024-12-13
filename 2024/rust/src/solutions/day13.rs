use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::i64 as ni64;
use nom::character::complete::line_ending;
use nom::multi::{many1, many1_count, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated};
use nom::IResult;
use nom::Parser;

use nalgebra::{vector, Affine2, Matrix2, Point2, Vector2};

type Pos = Vector2<f64>;

#[derive(Debug, Clone)]
struct Button {
    #[allow(unused)]
    label: char,
    coord: Pos,
}

#[derive(Clone, Debug)]
pub struct Machine {
    buttons: Vec<Button>,
    prize: Pos,
}

impl Machine {
    fn pushes2buttons(&self) -> Option<usize> {
        let b0 = self.buttons[0].coord;
        let b1 = self.buttons[1].coord;
        let mat = Affine2::<f64>::from_matrix_unchecked(
            Matrix2::new(b0.x, b1.x, b0.y, b1.y).to_homogeneous(),
        );
        let inv = mat.try_inverse().unwrap();
        let res = inv.transform_point(&(Point2::origin() + self.prize));
        let t0 = res.coords.x.round();
        let t1 = res.coords.y.round();

        if (t0 - res.coords.x).abs() > 0.001f64 {
            return None;
        }
        if (t1 - res.coords.y).abs() > 0.001f64 {
            return None;
        }
        Some(3 * (t0 as usize) + t1 as usize)
    }
}

fn deltacoord(i: &str) -> IResult<&str, Pos> {
    let (i, pos) = alt((
        preceded(tag("X=").or(tag("X")), ni64).map(|x| Vector2::new(x as f64, 0f64)),
        preceded(tag("Y=").or(tag("Y")), ni64).map(|y| Vector2::new(0f64, y as f64)),
    ))(i)?;
    Ok((i, pos))
}

fn parse_coord(i: &str) -> IResult<&str, Pos> {
    let (i, (d1, d2)) = separated_pair(deltacoord, tag(", "), deltacoord)(i)?;
    Ok((i, d1 + d2))
}

fn parse_machine(i: &str) -> IResult<&str, Machine> {
    let button = terminated(
        pair(
            terminated(preceded(tag("Button "), alphanumeric1), tag(": ")),
            parse_coord,
        ),
        line_ending,
    )
    .map(|(label, coord)| Button {
        label: label.chars().next().unwrap(),
        coord,
    });
    let (i, buttons) = many1(button)(i)?;
    let (i, prize) = preceded(tag("Prize: "), parse_coord)(i)?;
    Ok((i, Machine { buttons, prize }))
}

pub fn parse(i: &str) -> Vec<Machine> {
    let (i, machines) = separated_list1(many1_count(line_ending), parse_machine)(i)
        .expect("could not parse machines");
    assert!(i.trim().is_empty());
    machines
}

pub fn part1(i: &[Machine]) -> usize {
    i.iter()
        .filter_map(|machine| machine.pushes2buttons())
        .sum()
}

pub fn part2(i: &[Machine]) -> usize {
    let offset = vector![10000000000000f64, 10000000000000f64];
    let newi: Vec<_> = i
        .iter()
        .map(move |m| {
            let mut m = m.clone();
            m.prize += offset;
            m
        })
        .collect();
    part1(&newi)
}
