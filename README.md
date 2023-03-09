# divrs
A Rust/Improved Version of div.
Current Version: 0.4.0 - March 8th, 2023


# Operation
  To use the program, simply run the binary with an argument being the string that you wish to input.

# Current Functions
  This program currently:
  - Can take a character/string and output the Gematria value.

# Changelog

  *0.4.0 (March 8th, 2023)*
  - Altered ``main.rs`` to account for the increase in potential arguments passed to the binary.
  - Altered ``argparse()`` to now have a new flag - ``-c``.
    - ``-c`` increases the amount of cards that will be stored. The amount passed must be more than 0.

  *0.3.0 (March 7th, 2023)*
  - Added a new file: ``./src/argparse.rs``.
    - ``argparse.rs`` includes the function ``argumentparser(arguments: Vec<String>, options: &mut Vec<usize>) -> usize``. It takes the environmental arguments by way of ``arguments`` and parses them, writing the implications of the values (E.g. passing ``-g 2`` implying ``2`` for the type of Gematria) to ``options``. It also returns the index which will be considered the query.
  - Altered ``main.rs`` heavily to now handle the new parsing system.
  - Altered ``gemcalc.rs`` to now include a new function - ``ordgem(character: char) -> usize``. This uses Ordinal (E.g. A/a = 1, B/b = 2) Gematria.
  - Altered ``gematria()`` (``gemwrap.rs``) to now include handling for ``ordgem()``. 

  *0.2.0 (March 6th, 2023)*
  - Added ``./src/tarottool/gemwrap.rs``, which includes the function ``pub fn gematria(input: &String, gemtype: usize) -> usize)``. It acts as a wrapper to calculating Gematria values for characters; it allows for the easy implementation of multiple types of Gematria calculations
  - Altered ``./src/tarottool/gemcalc.rs`` to include a new function: ``pub fn asciigem(character: char) -> usize``. It uses ASCII values to calculate Gematria values for each character.
  - Altered ``./src/tarottool.rs`` to now include info relating to ``gemwrap.rs``.
  - Altered ``./src/main.rs`` to properly parse command-line arguments.

  *0.1.0 (March 5th, 2023)*
  - Added ``./src/``, which includes ``main.rs``.
  - Added ``./src/tarottool.rs``, which includes the formatting for the functions and files in the directory of the same name.
  - Added ``./src/tarottool``, which currently includes the file ``gemcalc.rs``.
    - ``gemcalc.rs`` includes the function ``pub fn chartoenggem(character: char) -> usize``. It takes a ``char`` and returns the English Gematria value of said character.
