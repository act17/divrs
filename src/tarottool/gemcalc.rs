// This function takes a char, and returns a usize based on English Gematria.
pub fn enggem(character: char) -> usize {
    // Convert the character into an ascii usize.
    let mut value = character as usize;

    // Check to see if the character is uppercase or lowercase.
    // The adjustment from ASCII to the value in the alphabet (E.g. A/a = 1 or Z/z = 26)
    // depends on if it's uppercase or lowercase.

    // In the case of uppercase...
    if value >= 65 && value <= 90 {
        value = value - 64;
    }
    // In the case of lowercase...
    else if value >= 97 && value <= 122 {
        value = value - 96;
    }
    // In the case that this isn't a traditional ASCII character,
    // and therefore do not matter in Gematria:
    else {
        return 0;
    }

    // This converts the value in the alphabet to the English Gematria:
    return value * 6;
}

// This function simply returns each character as its ASCII value.
pub fn asciigem(character: char) -> usize {
    return character as usize;
}
