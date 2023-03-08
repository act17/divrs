pub mod tarottool;
use crate::tarottool::gemwrap::gematria;

pub mod argparse;
use crate::argparse::argumentparser;
use crate::argparse::helpprint;

use std::env;

// Used as the number of arguments.
const NUMBER_OF_OPTIONS: usize = 1;

fn main() {
    // Initializing the program:
    // We see if the input of the program is null.
    // If so, we print a help message, and terminate.
    let mut argbuff = env::args().nth(1);
    if argbuff == None {
        println!("\nError!\nNo input given! Run the -H parameter!");
        return;
    }

    // We also check to see if the -H option was passed.
    // we do this first, as running the -H option means that the program
    // should terminate.
    let input = argbuff
        .expect("Failed to parse the first argument given.")
        .to_string();
    if input == "-H" {
        helpprint();
        return;
    }

    // Otherwise, we write to a vector all of the passed arguments from 1
    // until the end of the argument list.
    let mut arguments: Vec<String> = vec![String::from("0"); env::args().len()];
    // And a vector for the resulting values of our arguments; called "options".
    let mut options: Vec<usize> = vec![0; NUMBER_OF_OPTIONS];
    /*
     * GUIDE:
     *   Value 0 - Gematria (Should be 0, 1, or 2.)
     */

    // Now we finally fill the arguments vector.
    for i in 1..env::args().len() {
        argbuff = env::args().nth(i);
        arguments[i] = argbuff
            .expect("Failed to parse argument {i} when writing!")
            .to_string();
    }

    // Wi2th that being done, we can now run the in-effect macro to take the arguments,
    // and write them to our options:
    let querynum: usize = argumentparser(arguments, &mut options);
    // The function also returns the number that is the argument in which the query
    // is stored, so we write that to query.
    argbuff = env::args().nth(querynum);
    let query = argbuff
        .expect("Failed to parse argument {querynum} when writing to query!")
        .to_string();

    // Finally, we get to convert our query into a Gematria value.
    let value = gematria(&query, options[0]);

    println!("\n");
    println!("QUERY:    {}", query);
    println!("VALUE:    {}", value);
    println!("GEMATRIA TYPE:");

    match options[0] {
        0 => println!("English"),
        1 => println!("ASCII"),
        2 => println!("Ordinal"),
        _ => panic!("What in the goddamn?"),
    };
}
