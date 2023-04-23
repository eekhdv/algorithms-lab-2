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
