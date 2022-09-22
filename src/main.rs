mod input_parser;
mod array_handler;

pub use crate::input_parser::parse_input;
pub use crate::array_handler::generate_unmixed_array;
pub use crate::array_handler::shuffle;
pub use crate::array_handler::write_out_array;

use std::env;


fn main() {

    let args: Vec<String> = env::args().collect();
    run(args);
}

fn run(args: Vec<String>){

    let size = parse_input(&args);
    let mut array = generate_unmixed_array(size);
    write_out_array(&array);
    shuffle(&mut array);
    write_out_array(&array);
}





