use image::RgbImage;

pub enum Shape {
    Line((usize, usize, usize)),
}

struct Cell {}

struct Canvas {
    v: Vec<Vec<Cell>>,
}

const THRESHOLD: u8 = 200;

const MIN_LINE_LEN: usize = 30;

pub fn shapes_from_image(img: &RgbImage) -> Vec<Shape> {
    // y , x0, x1
    let mut horizzontal_lines: Vec<(usize, usize, usize)> = vec![];

    for (num, row) in img.enumerate_rows().collect::<Vec<_>>().into_iter().rev() {
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
                println!("{x} {y} = {c:?}");
            } else {
                if let Some(a) = prev {
                    if x as usize - a >= MIN_LINE_LEN {
                        horizzontal_lines.push((num as usize, a, x as usize - 1));
                    }
                    prev = None
                }
            }
        }
    }

    for line in &horizzontal_lines {
        println!("{line:?}")
    }

    horizzontal_lines.iter().map(|l| Shape::Line(*l)).collect()
}
