use crate::algorithms::structs::{point::Point, rect::Rect};

pub struct InputData {
    pub p: Point,
    pub rects: Vec<Rect>,
}

impl InputData {
    fn new(p: Point, rects: Vec<Rect>) -> Self {
        Self {
            p: (p),
            rects: (rects),
        }
    }
}

impl From<(Point, Vec<Rect>)> for InputData {
    fn from(value: (Point, Vec<Rect>)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub(super) fn user_input() -> std::io::Result<InputData> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    print!("Enter number of rectangles: ");
    stdin.read_line(&mut buf)?;
    let n: i32 = buf.parse().unwrap();
    let mut rects: Vec<Rect> = Vec::new();

    for i in 0..n {
        print!("[{i} rect] Enter lower-left rectangle coords (\"x y\"): ");
        let ll = read_point(&stdin, &mut buf)?;

        print!("[{i} rect] Enter upper-right rectangle coords (\"x y\"): ");
        let ur = read_point(&stdin, &mut buf)?;

        rects.push(Rect::new(ll, ur));
    }
    print!("Enter point coords (\"x y\"): ");
    let p = read_point(&stdin, &mut buf)?;

    Ok(InputData::from((p, rects)))
}

fn read_point(stdin: &std::io::Stdin, buf: &mut String) -> std::io::Result<Point> {
    stdin.read_line(buf)?;
    let mut p = buf.split(" ");
    let p: (i32, i32) = (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    );
    Ok(Point::from_tup(p))
}
