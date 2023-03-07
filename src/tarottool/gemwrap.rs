use crate::tarottool::gemcalc::asciigem;
use crate::tarottool::gemcalc::enggem;

// This function wraps the behaviour of converting the input to Gematria.
pub fn gematria(input: &String, gemtype: usize) -> usize {
    let mut value: usize = 0;

    match gemtype {
        // For English Gematria
        0 => {
            for chr in input.chars() {
                value = value + enggem(chr);
            }
        }

        1 => {
            for chr in input.chars() {
                value = value + asciigem(chr);
            }
        }

        _ => panic!("Error! Value for Gematria type not valid!"),
    }

    return value;
}
