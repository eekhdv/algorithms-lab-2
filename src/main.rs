pub mod algorithms;

fn main() -> std::io::Result<()> {
    let _input_data = user_input()?;
    Ok(())
}

fn user_input() -> std::io::Result<((i32, i32), Vec<((i32, i32), (i32, i32))>)> {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    print!("Enter number of rectangles: ");
    stdin.read_line(&mut buf)?;
    let n: i32 = buf.parse().unwrap();
    let mut rects: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for i in 0..n {
        print!("[{i} rect] Enter lower-left rectangle coords (\"x y\"): ");
        stdin.read_line(&mut buf)?;
        let mut ll = buf.split(" ");
        let ll: (i32, i32) = (
            ll.next().unwrap().parse().unwrap(),
            ll.next().unwrap().parse().unwrap(),
        );

        print!("[{i} rect] Enter upper-right rectangle coords (\"x y\"): ");
        stdin.read_line(&mut buf)?;
        let mut ur = buf.split(" ");
        let ur: (i32, i32) = (
            ur.next().unwrap().parse().unwrap(),
            ur.next().unwrap().parse().unwrap(),
        );

        rects.push((ll, ur));
    }
    print!("Enter point coords (\"x y\"): ");
    stdin.read_line(&mut buf)?;
    let mut p = buf.split(" ");
    let p: (i32, i32) = (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    );

    Ok((p, rects))
}
