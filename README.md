# divrs
A Rust/Improved Version of div.
Current Version: 0.2.0 - March 6th, 2023


# Operation
    To use the program, simply run the binary with an argument being the string that you wish to input.

# Current Functions
    This program currently:
    - Can take a character/string and output the Gematria value.

# Changelog

    *0.2.0 (March 6th, 2023)*
    - Added ``./src/tarottool/gemwrap.rs``, which includes the function ``pub fn gematria(input: &String, gemtype: usize) -> usize)``. It acts as a wrapper to calculating Gematria values for characters; it allows for the easy implementation of multiple types of Gematria calculations.
    - Altered ``./src/tarottool/gemcalc.rs`` to include a new function: ``pub fn asciigem(character: char) -> usize``. It uses ASCII values to calculate Gematria values for each character.
    - Altered ``./src/tarottool.rs`` to now include info relating to ``gemwrap.rs``.
    - Altered ``./src/main.rs`` to properly parse command-line arguments.

    *0.1.0 (March 5th, 2023)*
    - Added ``./src/``, which includes ``main.rs``.
    - Added ``./src/tarottool.rs``, which includes the formatting for the functions and files in the directory of the same name.
    - Added ``./src/tarottool``, which currently includes the file ``gemcalc.rs``.
      - ``gemcalc.rs`` includes the function ``pub fn chartoenggem(character: char) -> usize``. It takes a ``char`` and returns the English Gematria value of said character.