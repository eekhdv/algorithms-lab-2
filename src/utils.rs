use crate::algorithms::structs::{point::Point, rect::Rect};

pub struct InputData {
    pub points: Vec<Point>,
    pub rects: Vec<Rect>,
}

impl InputData {
    fn new(points: Vec<Point>, rects: Vec<Rect>) -> Self {
        Self {
            points: (points),
            rects: (rects),
        }
    }
}

impl From<(Vec<Point>, Vec<Rect>)> for InputData {
    fn from(value: (Vec<Point>, Vec<Rect>)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(super) fn user_input() -> std::io::Result<InputData> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    // println!("Enter number of rectangles: ");
    stdin.read_line(&mut buf)?;
    let n: i32 = buf.strip_suffix('\n').unwrap().parse().unwrap();
    buf.clear();
    let mut rects: Vec<Rect> = Vec::new();

    for _i in 0..n {
        // println!(
        //     "[{i} rect] Enter lower-left and upper right rectangle coords (\"x1 y1 x2 y2\"): "
        // );
        let rect = read_rect(&stdin, &mut buf)?;
        rects.push(rect);
        buf.clear();
    }
    // println!("Enter number of points: ");
    stdin.read_line(&mut buf)?;

    // println!("Enter point coords (\"x y\"): ");
    let m: i32 = buf.strip_suffix('\n').unwrap().parse().unwrap();
    let mut points: Vec<Point> = Vec::new();
    buf.clear();

    for _i in 0..m {
        // println!("[{i} point] Enter point coords (\"x y\"): ");
        let p = read_point(&stdin, &mut buf)?;
        buf.clear();
        points.push(p);
    }
    Ok(InputData::from((points, rects)))
}

fn read_point(stdin: &std::io::Stdin, buf: &mut String) -> std::io::Result<Point> {
    stdin.read_line(buf)?;
    let mut p = buf.strip_suffix('\n').unwrap().split(" ");
    let p: (i32, i32) = (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    );
    Ok(Point::from_tup(p))
}

fn read_rect(stdin: &std::io::Stdin, buf: &mut String) -> std::io::Result<Rect> {
    stdin.read_line(buf)?;
    let mut p = buf.strip_suffix('\n').unwrap().split(" ");
    let p1: (i32, i32) = (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    );
    let p2: (i32, i32) = (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    );

    Ok(Rect::new(Point::from_tup(p1), Point::from_tup(p2)))
}
