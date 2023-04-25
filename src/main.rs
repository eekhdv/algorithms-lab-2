pub mod algorithms;
mod utils;

fn main() -> std::io::Result<()> {
    // let input_data = utils::user_input()?;
    // let _first_ans = algorithms::first::AlgorithmBruteForce::count_rect_for_point(
    //     &input_data.points,
    //     &input_data.rects,
    // );
    // let _second_ans = algorithms::second::AlgorithmOnMap::count_rect_for_point(
    //     &input_data.points,
    //     &input_data.rects,
    // );
    // let _third_ans = algorithms::third::AlgorithmOnPersistenTree::count_rect_for_point(
    //     &input_data.points,
    //     &input_data.rects,
    // );
    // println!("{:?}", _third_ans);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use algorithms::traits::lab::LabSolution;
    use mod_exp::mod_exp;

    use crate::algorithms::{
        self,
        structs::{point::Point, rect::Rect},
    };

    use rand::prelude::*;

    struct TestData {
        rects: Vec<Rect>,
        points: Vec<Point>,
    }

    impl Into<(Vec<Rect>, Vec<Point>)> for TestData {
        fn into(self) -> (Vec<Rect>, Vec<Point>) {
            (self.rects, self.points)
        }
    }
    #[test]
    fn data_preparation_test() {
        data_preparation_test_case(&_generate_small_data(), "small");
        data_preparation_test_case(&_generate_medium_data(), "medium");
        data_preparation_test_case(&_generate_huge_data(), "huge");
    }

    #[test]
    fn finding_answer_algorithm_test() {
        finding_answer_test_case(&_generate_small_data(), "small");
        finding_answer_test_case(&_generate_medium_data(), "medium");
        finding_answer_test_case(&_generate_medium_data(), "huge");
    }

    fn generate_tests(n: i32) -> TestData {
        TestData {
            rects: generate_rects(n),
            points: generate_points(n),
        }
    }

    fn generate_rects(n: i32) -> Vec<Rect> {
        let mut rects: Vec<Rect> = Vec::with_capacity(n as usize);
        for i in 1..n {
            let ll = Point::new(10 * i, 10 * i);
            let ur = Point::new(10 * (2 * n - i), 10 * (2 * n - i));
            rects.push(Rect::new(ll, ur));
        }
        rects
    }

    fn generate_points(n: i32) -> Vec<Point> {
        let exp = 31;
        let mut points: Vec<Point> = Vec::with_capacity(n as usize);
        for i in 1..n {
            let base_x: i32 = 2371 * i;
            let base_y: i32 = 1979 * i;
            let x = mod_exp(base_x as i128, exp as i128, 20 * n as i128) as i32;
            let y = mod_exp(base_y as i128, exp as i128, 20 * n as i128) as i32;
            points.push(Point::new(x, y));
        }
        points
    }
    fn data_preparation_test_case(test_data: &TestData, case: &str) {
        println!("--- start({}) ---", case);
        println!(
            "{} points and {} rectangles were used.",
            test_data.points.len(),
            test_data.rects.len()
        );

        if test_data.rects.len() <= 2000 {
            let start = Instant::now();
            let _second_data_prep =
                algorithms::second::AlgorithmOnMap::prepare_data(&test_data.rects);
            let duration = start.elapsed();
            println!("2 (p) -> time: {} ns", duration.as_nanos());
        }

        let start = Instant::now();
        let _third_data_prep =
            algorithms::third::AlgorithmOnPersistenTree::prepare_data(&test_data.rects);
        let duration = start.elapsed();
        println!("3 (p) -> time: {} ns", duration.as_nanos());
    }

    fn finding_answer_test_case(_test_data: &TestData, case: &str) {
        println!("--- start({}) ---", case);
        println!(
            "{} points and {} rectangles were used.",
            _test_data.points.len(),
            _test_data.rects.len()
        );

        let print_time = |case_num: &str, time: u128| {
            println!(
                "{} (a) -> time per 1 query: {} ns",
                case_num,
                time / _test_data.points.len() as u128
            );
        };

        let mut all_p_time = 0;
        let mut ans = 0;
        for p in &_test_data.points {
            let start = Instant::now();
            ans = algorithms::first::AlgorithmBruteForce::find_single_point(&_test_data.rects, p);
            let duration = start.elapsed();
            all_p_time += duration.as_nanos();
        }
        print_time(&format!("({}) 3", ans), all_p_time);

        if _test_data.rects.len() <= 2000 {
            let mut all_p_time = 0;
            let mut ans = 0;
            let _second_data_prep =
                algorithms::second::AlgorithmOnMap::prepare_data(&_test_data.rects).unwrap();
            for p in &_test_data.points {
                let start = Instant::now();
                ans += algorithms::second::AlgorithmOnMap::find_single_point(&_second_data_prep, p);
                let duration = start.elapsed();
                all_p_time += duration.as_nanos();
            }
            print_time(&format!("({}) 3", ans), all_p_time);
        }

        let mut all_p_time = 0;
        let mut ans = 0;
        let _third_data_prep =
            algorithms::third::AlgorithmOnPersistenTree::prepare_data(&_test_data.rects).unwrap();
        for p in &_test_data.points {
            let start = Instant::now();
            ans += algorithms::third::AlgorithmOnPersistenTree::find_single_point(
                &_third_data_prep,
                p,
            );
            let duration = start.elapsed();
            all_p_time += duration.as_nanos();
        }
        print_time(&format!("({}) 3", ans), all_p_time);
    }
    fn _generate_small_data() -> TestData {
        let small_d = rand::thread_rng().gen_range(10..3000);
        generate_tests(small_d)
    }
    fn _generate_medium_data() -> TestData {
        let medium_d = rand::thread_rng().gen_range(5000..50000);
        generate_tests(medium_d)
    }
    fn _generate_huge_data() -> TestData {
        let huge_d = rand::thread_rng().gen_range(80000..200000);
        generate_tests(huge_d)
    }
}
