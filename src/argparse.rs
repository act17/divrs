// This function takes the arguments given, alters options for use outside
// of the function, and returns the index of the argument that is the query.
pub fn argumentparser(arguments: Vec<String>, options: &mut Vec<usize>) -> usize {
    // This is used to store which argument is the query.
    let mut querynumber: usize = 1;

    // This loops until the end of the arguments.
    for mut i in 1..arguments.len() {
        // If we are changing the Gematria form with -g...
        if arguments[i] == "-g" {
            // First we add to i as to prevent future parsing errors...
            i += 1;
            // Then we see if there's no following argument:
            if i >= arguments.len() {
                // If so, we print a brief error message and stop parsing for '-g'.
                println!("Error! There's no following option for the '-g' flag!");
                println!("Run the program with '-H' for help!");
                return querynumber;
            }
            // Otherwise, we use a match to determine what to do next:
            match arguments[i].as_str() {
                "0" => options[0] = 0,
		"1" => options[0] = 1,
		"2" => options[0] = 2,
		_ => println!("Error! Following option for '-g' flag invalid!\nGematria type will stay as 0 (Default)!"),
	    };
            // We then proceed to add one to i and check for overflows to prevent errors
            // with interpreting the following parameter to -g as the query.
            i += 1;
            if i >= arguments.len() {
                return querynumber;
            }
        }

        // If we are printing the help message with -H...
        if arguments[i] == "-H" {
            helpprint();
        }
        // Otherwise, we presume this index is the query.
        else {
            querynumber = i;
        }
    }

    return querynumber;
}

// This function prints out a manual to help users with using the software.
pub fn helpprint() {
    println!("Help Page:\n");
    println!("Format for use: divrs ''Query'' <arguments>");
    println!("List of arguments:\n");
    println!("-H  -  Prints this message.");
    println!("-g  -  Changes the Gematria type used.");
    println!("0 = English (Default) | 1 = ASCII | 2 = Ordinal");
}
