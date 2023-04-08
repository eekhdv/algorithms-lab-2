pub mod algorithms;
mod utils;

fn main() -> std::io::Result<()> {
    let input_data = utils::user_input()?;
    let _first_ans = algorithms::first::count_rect_for_point(input_data.p, input_data.rects);
    
    Ok(())
}

