use core::{f32, panic};
use std::{collections::HashMap, usize};

use image::{GenericImageView, RgbImage};
use imageproc::drawing::Canvas;
use nannou::glam::Vec2;
use svg::node::element::tag::Line;

#[derive(Clone, Debug)]
pub struct Point(pub f32, pub f32);

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}

impl Point {
    fn to_vec(&self) -> Vec2 {
        self.clone().into()
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub thickness: f32,
}

impl TryFrom<Vec<(usize, usize)>> for Line {
    type Error = ();
    fn try_from(value: Vec<(usize, usize)>) -> Result<Self, Self::Error> {
        let mut map: HashMap<usize, usize> = HashMap::new();

        for (x, y) in &value {
            if let Some(v) = map.get_mut(&y) {
                *v += 1;
            } else {
                map.insert(*y, 1);
            }
        }

        let values = map.values();
        let min = values.clone().min().ok_or(())?;
        let max = values.max().ok_or(())?;

        // dbg!(map.clone());

        println!("{} {min} {max}", max - min);

        if max - min < 5 {
            map.clear();

            for (x, y) in &value {
                if let Some(v) = map.get_mut(&x) {
                    *v += 1;
                } else {
                    map.insert(*x, 1);
                }
            }

            let values = map.values();
            let min = values.clone().min().ok_or(())?;
            let max = values.max().ok_or(())?;

            if max - min < 5 {
                let min_x = value.iter().map(|(x, _)| x).min().unwrap();
                let max_x = value.iter().map(|(x, _)| x).max().unwrap();
                let min_y = value.iter().map(|(_, y)| y).min().unwrap();
                let max_y = value.iter().map(|(_, y)| y).max().unwrap();

                return Ok(Line {
                    start: Point(*min_x as _, *max_y as _),
                    end: Point(*max_x as _, *min_y as _),
                    thickness: ((min + max) / 2) as f32,
                });
            }
        }

        Err(())
    }
}

impl Line {
    fn similarity_to(&self, other: &Line) -> f32 {
        let start_diff = self.start.to_vec() - other.start.to_vec();
        let end_diff = self.end.to_vec() - other.end.to_vec();

        start_diff.length() + end_diff.length() - self.thickness - other.thickness
    }

    fn merge_with(&self, other: &Line) -> Line {
        let start_diff = self.start.to_vec() - other.start.to_vec();
        let end_diff = self.end.to_vec() - other.end.to_vec();

        Line {
            start: Point(self.start.0 + start_diff.x, self.start.1 + start_diff.y),
            end: Point(self.end.0 + end_diff.x, self.end.1 + end_diff.y),
            thickness: self.thickness + other.thickness,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Shape {
    Line(Line),
    Custom(Vec<(usize, usize)>),
}

impl Shape {
    fn similarity_to(&self, other: &Shape) -> f32 {
        match (self, other) {
            (Self::Line(a), Self::Line(b)) => a.similarity_to(b),
            _ => f32::MAX,
        }
    }

    fn merge_with(&self, other: &Shape) -> Shape {
        match (self, other) {
            (Self::Line(a), Self::Line(b)) => Self::Line(a.merge_with(b)),
            _ => panic!("Can only merge same types"),
        }
    }
}

const THRESHOLD: u8 = 200;

fn horizzontal_lines_from_image(img: &mut RgbImage) -> Vec<Shape> {
    const MIN_LINE_LEN: usize = 20;

    // y , x0, x1
    let mut horizzontal_lines: Vec<(usize, usize, usize, usize)> = vec![];

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;

    for (num, (_, row)) in img
        .enumerate_rows()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        let mut prev: Option<usize> = None;
        for (x, y, c) in row {
            //if c[0] == 255 && c[1] == 255 && c[2] == 255 {
            //    continue;
            //}

            if c[0] < THRESHOLD && c[1] < THRESHOLD && c[2] < THRESHOLD {
                if prev.is_some() {
                    continue;
                }

                prev = Some(x as usize);
            } else {
                if let Some(a) = prev {
                    if x as usize - a >= MIN_LINE_LEN {
                        min_x = min_x.min(a as usize);
                        min_y = min_y.min(num as usize);

                        horizzontal_lines.push((num as usize, a, x as usize - 1, y as usize));
                    }
                    prev = None
                }
            }
        }
    }

    for line in &mut horizzontal_lines {
        for point in line.1..line.2 {
            let pixel = img.get_pixel_mut(point as _, line.3 as _);
            pixel.0 = [255, 255, 255];
        }

        //line.0 -= min_y;
        //line.1 -= min_x;
        //line.2 -= min_x;
    }

    // merge all the lines

    let lines: Vec<_> = horizzontal_lines
        .iter()
        .map(|l| {
            Shape::Line(Line {
                start: Point(l.1 as f32, l.0 as f32),
                end: Point(l.2 as f32, l.0 as f32),
                thickness: 1.,
            })
        })
        .collect();

    let mut iter = lines.iter().peekable();

    let mut lines = vec![];

    let next = iter.next();
    if next.is_some() {
        let mut next = next.unwrap().clone();
        loop {
            let peek = iter.peek();

            if peek.is_none() {
                lines.push(next.clone());
                break;
            }

            let peek = peek.unwrap();

            if next.similarity_to(peek) < 20. {
                let peek = iter.next().unwrap();

                next = next.merge_with(peek);
            } else {
                lines.push(next.clone());
                next = iter.next().unwrap().clone();
            }
        }
    }

    //for line in &mut lines {
    //    match line {
    //        Shape::Line(line) => println!("{:?} {:?} {}", line.start, line.end, line.thickness),
    //        _ => {}
    //    }
    //}

    return lines;
}

fn extract_shape(img: &mut RgbImage, x: usize, y: usize) -> Vec<(usize, usize)> {
    let c = img.get_pixel_mut_checked(x as _, y as _);

    if let Some(c) = c {
        if c.0[0] < THRESHOLD && c.0[1] < THRESHOLD && c.0[2] < THRESHOLD {
            c.0 = [255, 255, 255];
            let mut res = vec![(x, (img.height() as usize - y))];
            res.extend(extract_shape(img, x + 1, y));
            res.extend(extract_shape(img, x - 1, y));
            res.extend(extract_shape(img, x + 1, y + 1));
            res.extend(extract_shape(img, x - 1, y + 1));
            res.extend(extract_shape(img, x, y + 1));
            res.extend(extract_shape(img, x + 1, y - 1));
            res.extend(extract_shape(img, x - 1, y - 1));
            res.extend(extract_shape(img, x, y - 1));

            return res;
        }
    }

    vec![]
}

fn extract_shapes(img: &mut RgbImage) -> Vec<Shape> {
    let mut res = vec![];
    for y in 0..img.height() {
        for x in 0..img.width() {
            let c = img.get_pixel(x, y).0;

            if c[0] < THRESHOLD && c[1] < THRESHOLD && c[2] < THRESHOLD {
                let shape = extract_shape(img, x as _, y as _);

                if shape.len() > 50 {
                    res.push(
                        Line::try_from(shape.clone())
                            .map(|l| Shape::Line(l))
                            .unwrap_or_else(|l| Shape::Custom(shape)),
                    );
                }
            }
        }
    }

    res
}

pub fn shapes_from_image(img: &mut RgbImage) -> Vec<Shape> {
    let mut lines = horizzontal_lines_from_image(img);

    let diagonals = extract_shapes(img);

    println!("{}", diagonals.len());

    lines.extend(diagonals);

    lines
}
