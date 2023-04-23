pub mod algorithms;
mod utils;

use algorithms::traits::lab::LabSolution;

fn main() -> std::io::Result<()> {
    let input_data = utils::user_input()?;
    let _first_ans = algorithms::first::AlgorithmBruteForce::count_rect_for_point(
        &input_data.points,
        &input_data.rects,
    );
    let _second_ans = algorithms::second::AlgorithmOnMap::count_rect_for_point(
        &input_data.points,
        &input_data.rects,
    );
    let _third_ans = algorithms::third::AlgorithmOnPersistenTree::count_rect_for_point(
        &input_data.points,
        &input_data.rects,
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use mod_exp::mod_exp;
    use algorithms::traits::lab::LabSolution;

    use crate::algorithms::{structs::{point::Point, rect::Rect}, self};

    struct TestData {
        rects: Vec<Rect>,
        points: Vec<Point>,
    }

    impl Into<(Vec<Rect>, Vec<Point>)> for TestData {
        fn into(self) -> (Vec<Rect>, Vec<Point>) {
            (self.rects, self.points)
        }

    }
    fn generate_tests() -> TestData {
        let n = 37;
        TestData {
            rects: generate_rects(n),
            points: generate_points(n),
        }
    }

    fn generate_rects(n: i32) -> Vec<Rect> {
        let mut rects: Vec<Rect> = Vec::with_capacity(n as usize);
        for i in 1..37 {
            let lower_l = Point::new(10 * i, 10 * i);
            let upper_r = Point::new(10*(2 * n - i), 10 * (2 * n - i));
            println!("x1: {} y1: {}", lower_l.x, lower_l.y);
            println!("x2: {} y2: {}", upper_r.x, upper_r.y);
            rects.push(Rect::new(lower_l, upper_r));
        }
        rects
    }

    fn generate_points(n: i32) -> Vec<Point> {
        let exp = 31;
        let mut points: Vec<Point> = Vec::with_capacity(n as usize);
        for i in 1..37 {
            let base_x = 2371 * i;
            let base_y = 1979 * i;
            let x: i32 = mod_exp(base_x, exp, 20 * n).try_into().unwrap();
            let y: i32 = mod_exp(base_y, exp, 20 * n).try_into().unwrap();
            points.push(Point::new(x, y));
        }
        points 
    }

    #[test]
    fn firts_algorithm_test() {
        let _test_data = generate_tests();

        let _first_ans = algorithms::first::AlgorithmBruteForce::count_rect_for_point(
            &_test_data.points,
            &_test_data.rects,
        );
        let _second_ans = algorithms::second::AlgorithmOnMap::count_rect_for_point(
            &_test_data.points,
            &_test_data.rects,
        );
        let _third_ans = algorithms::third::AlgorithmOnPersistenTree::count_rect_for_point(
            &_test_data.points,
            &_test_data.rects,
        );
        assert_eq!(_first_ans, _second_ans);
        assert_eq!(_first_ans, _third_ans);
    }

}
