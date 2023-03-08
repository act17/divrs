use crate::tarottool::gemcalc::asciigem;
use crate::tarottool::gemcalc::enggem;
use crate::tarottool::gemcalc::ordgem;

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
        // For Ascii Gematria
        1 => {
            for chr in input.chars() {
                value = value + asciigem(chr);
            }
        }
        // For Ordinal Gematria
        2 => {
            for chr in input.chars() {
                value = value + ordgem(chr);
            }
        }
        // Otherwise, soil bed.
        _ => panic!("Error! Value for Gematria type not valid!"),
    }

    return value;
}
