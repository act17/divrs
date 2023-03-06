use crate::tarottool::gemcalc::chartoenggem;
pub mod tarottool;

use std::env;

fn main() {
    let argbuff = env::args().nth(1);
    let input: String = argbuff.expect("Failed to read string!").to_string();
    let mut value: usize = 0;

    for i in 0..input.len() {
        value = value + chartoenggem(input.chars().nth(i).unwrap());
    }

    println!("Input: {input}\nValue: {value}");
}
