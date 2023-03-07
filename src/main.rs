pub mod tarottool;
use crate::tarottool::gemwrap::gematria;

use std::env;

fn main() {
    // Initializing the program:
    // 0: We see if the input of the program is null.
    // If so, we print a help message, and terminate.
    let mut argbuff = env::args().nth(1);
    if argbuff == None {
        println!("\nError!\nNo input given! Run the -H parameter!");
        return;
    }

    // 1: We call variables relating to initialization.
    let mut input: String; // Used as a readable buffer for input.
    let mut query: String = "0".to_string(); // Used as a storage for the actual query.
    let mut gematriatype: usize = 0; // Relates to the type of Gematria.

    // 2: We parse the arguments with a loop until we find the end of the arguments:
    let mut argumenthandler: usize = 1; // This is used to see which argument we're on.

    loop {
        // We take the argument...
        argbuff = env::args().nth(argumenthandler);
        // Then we read to see if it's data is null. If so, we break this loop.
        if argbuff == None {
            break;
        }

        // We convert the argument into a string for reading.
        input = argbuff.expect("Failed to read string.").to_string();

        // If we are to be printing the help page...
        if input == "-H" {
            helpprint();
            return;
        }

        // If we are to be changing the Gematria with -g...
        if input == "-g" {
            // We then need to parse the next argument.
            // We add one to argumenthandler to prevent future errors.
            argumenthandler += 1;
            let secondargbuff = env::args().nth(argumenthandler);
            // In the case of a null value, we print an error, and terminate
            // the process of parsing this command.
            if secondargbuff == None {
                println!("Error! Option '-g' has no following number!");
                println!("Gematria will be set to 0 (Default). Run with -H for help.");
                break;
            }
            // Then we process the second argument into a string for the sake of
            // comparison.
            let secondarg: String = secondargbuff.expect("Failed to read string.").to_string();
            // Then we see what to do with it:
            match secondarg.as_str() {
		"0" => gematriatype = 0,
                "1" => gematriatype = 1,
		_ => println!("Error! Option '-g' has an invalid number!\nGematria will be set to 0 (Default). Run with -H for help."),
	    }
        }
        // Otherwise, we assume that the input given is our query; and we write that
        // to the dedicated query value.
        else {
            query = input.to_string();
        }

        argumenthandler += 1;
    }

    let value: usize = gematria(&query.to_string(), gematriatype);

    println!("");
    println!(
        "Query:		{query}
            \nGematria:	{gematriatype}
            \nValue:		{value}"
    );
}

pub fn helpprint() {
    println!("HELP PAGE FOR DIVRS:\n");
    println!("Format:\n ./divrs ''<query>'' <arguments>\n");
    println!("Currently supported arguments:");
    println!("-H          -  Prints this message.");
    println!("-g <0 - 1>  -  Changes the type of Gematria used.\n0 = English\n1 = ASCII");
}
