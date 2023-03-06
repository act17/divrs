# divrs
A Rust/Improved Version of div.
Current Version: 0.1.0 - March 5th, 2023


# Operation
    To use the program, simply run the binary with an argument being the string that you wish to input.

# Current Functions
    This program currently:
    - Can take a character/string and output the Gematria value.

# Changelog

    *Alpha 1.0.0 (March 5th, 2023)*
    - Added ``./src/``, which includes ``main.rs``.
    - Added ``./src/tarottool.rs``, which includes the formatting for the functions and files in the directory of the same name.
    - Added ``./src/tarottool``, which currently includes the file ``gemcalc.rs``.
      - ``gemcalc.rs`` includes the function ``pub fn chartoenggem(character: char) -> usize``. It takes a ``char`` and returns the English Gematria value of said character.